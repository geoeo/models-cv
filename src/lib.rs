extern crate nalgebra as na;

mod byte_array_info;
pub mod feature_matches;
pub mod filter;
pub mod io;

use std::iter::zip;
use std::collections::HashMap;
use byte_array_info::ByteArrayInfo;
use na::{Vector2,Vector3,Matrix3,Matrix4xX,Matrix3x4, Matrix3xX};


/**
 * Returns a Vec<ByteArrayInfo> of position data
 */
pub fn find_position_buffer_data(document: &gltf::Document) -> Vec<ByteArrayInfo> { 
    document.meshes().map(|mesh| {
        mesh.primitives().map(|primitive| {
            primitive.attributes().filter(|attribute| attribute.0 == gltf::Semantic::Positions).map(|attr| {
                let buffer_view = attr.1.view().expect("Buffer is sparse. This is not implemented");
                ByteArrayInfo::new (attr.1.data_type(),attr.1.dimensions(), buffer_view.buffer().index(),buffer_view.offset(), buffer_view.length(), buffer_view.stride())
            })
        }).flatten()
    }).flatten().collect()
}

pub fn load_position_byte_data(position_buffer_info: Vec<ByteArrayInfo>, buffers: &Vec<gltf::buffer::Data>) -> Vec<&[u8]> {
    position_buffer_info.into_iter().map(|info| {
        let byte_end = info.get_byte_offset()+info.get_byte_length();
        match info.get_byte_stride() {
            None => &buffers[info.get_buffer_index()].0[info.get_byte_offset()..byte_end],
            Some(_) => panic!("TODO Strided Position Loading") // After every read, continue for stide - size bytes
        }
    }).collect()
}

pub fn convert_byte_data_to_vec3(position_byte_data: Vec<&[u8]>) -> Vec<Vec<Vector3<f32>>> {
    position_byte_data.into_iter().map(|byte_slice| {
        let vec3_capacity = byte_slice.len() / 12;
        let mut data_vector = Vec::<Vector3<f32>>::with_capacity(vec3_capacity);
        for i in 0..vec3_capacity {
            let x_s = 12*i;
            let x_f = x_s + 4;
            let y_s = x_f;
            let y_f = x_s + 8;
            let z_s = y_f;
            let z_f = x_s + 12;
            let x_raw_arr = byte_slice[x_s..x_f].try_into().expect("X: Could not convert 4 byte slice to array");
            let y_raw_arr = byte_slice[y_s..y_f].try_into().expect("Y: Could not convert 4 byte slice to array");
            let z_raw_arr = byte_slice[z_s..z_f].try_into().expect("Z: Could not convert 4 byte slice to array");
            let x = f32::from_le_bytes(x_raw_arr);
            let y = f32::from_le_bytes(y_raw_arr);
            let z = f32::from_le_bytes(z_raw_arr);
            data_vector.push(Vector3::<f32>::new(x, y, z));
        }
        data_vector
    }).collect()
}

pub fn project_points(points: &Vec<Vector3<f32>>, intrinsic_matrix: &Matrix3<f32>, view_matrix: &Matrix3x4<f32>, screen_width: f32, screen_height: f32) -> (Vec<(usize,Vector2<usize>)>, Matrix3xX<f32>) {
    let mut ps = Matrix4xX::<f32>::from_element(points.len(), 1.0);
    for i in 0..points.len() {
        ps.fixed_view_mut::<3,1>(0,i).copy_from(&points[i]);
    }
    let points_cam = view_matrix*ps;
    let projected_points = intrinsic_matrix*&points_cam;
    let screen_points_with_idx = projected_points.column_iter().enumerate()
        .filter(|(_,c)| c[2] != 0.0)
        .map(|(i,c)| (i,(c[0]/c[2],c[1]/c[2])))
        .filter(|&(_,(x,y))| 0.0 <= x && 0.0 <= y && x < screen_width && y < screen_height)
        .map(|(i,(x,y))| (i,Vector2::new(x.floor() as usize, y.floor() as usize)))
        .collect::<Vec<_>>();
    (screen_points_with_idx, points_cam)
}

pub fn filter_screen_points_for_camera_views(points: &Vec<Vector3<f32>>, intrinsic_matrix: &Matrix3<f32>, view_matrices: &Vec<Matrix3x4<f32>>, screen_width: f32, screen_height: f32, filter_type: filter::FilterType) -> Vec<Vec<(usize,Vector2<usize>)>> {
    view_matrices.iter().map(|view_matrix| {
        let (points_screen_with_idx, points_cam) = project_points(points, &intrinsic_matrix, &view_matrix.fixed_view::<3,4>(0, 0).into_owned(),screen_width, screen_height);
        match filter_type {
            filter::FilterType::Depth => filter::filter_visible_screen_points_by_depth(&points_screen_with_idx,&points_cam),
            filter::FilterType::TriangleIntersection => filter::filter_visible_screen_points_by_triangle_intersection(&points_screen_with_idx,&points_cam,&intrinsic_matrix)
        }
    }).collect::<Vec<_>>()
}

pub fn calculate_rgb_byte_vec(screen_points: &Vec<Vector2<usize>>, screen_width: usize, screen_height: usize) -> Vec<u8> {
    let mut dat_vec: Vec<u8> = vec![0;3*screen_width*screen_height];

    for screen_point in screen_points {
        let x = screen_point.x;
        let y = screen_point.y;
        let screen_idx = y*screen_width + x;
        let byte_idx = 3*screen_idx;
        dat_vec[byte_idx] = 255;
        dat_vec[byte_idx+1] = 255;
        dat_vec[byte_idx+2] = 255;
    }

    dat_vec
}

pub fn generate_matches(view_matrices: &Vec<Matrix3x4<f32>>, intrinsic_matrices: &Vec<Matrix3<f32>>, features: &Vec<Vec<(usize,Vector2<usize>)>>) -> Vec<feature_matches::FeatureMatches> {
    assert_eq!(view_matrices.len(), features.len());
    zip(zip(view_matrices,intrinsic_matrices),features).enumerate().map(|(cam_id,((view_matrix,intrinsic_matrix),screen_points_with_id))| {
        let point_map = screen_points_with_id.into_iter().map(|&(k,v)| (k,v)).collect::<HashMap<usize,Vector2<usize>>>();
        feature_matches::FeatureMatches::new(point_map,cam_id,view_matrix.clone(),intrinsic_matrix.clone()) 
    }).collect()
}


