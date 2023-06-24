extern crate gltf;
extern crate nalgebra as na;
mod byte_array_info;

use std::collections::HashMap;

use byte_array_info::ByteArrayInfo;
use na::{Vector2,Vector3,Matrix3,Matrix4xX,Matrix3x4};

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

pub fn project_points(points: &Vec<Vector3<f32>>, intrinsic_matrix: &Matrix3<f32>, view_matrix: &Matrix3x4<f32>) -> Vec<(Vector2<usize>,f32)> {
    let projection_matrix = intrinsic_matrix*view_matrix;
    let mut ps = Matrix4xX::<f32>::from_element(points.len(), 1.0);
    for i in 0..points.len() {
        ps.fixed_view_mut::<3,1>(0,i).copy_from(&points[i]);
    }
    let projected_points = projection_matrix*ps;
    projected_points.column_iter().map(|c| (Vector2::new((c[0]/c[2]).floor() as usize,(c[1]/c[2]).floor() as usize),c[2])).collect::<Vec<_>>()
}

pub fn filter_visible_screen_points(screen_points_with_depth: &Vec<(Vector2<usize>,f32)>) -> Vec<Vector2<usize>> {
    let mut closest_point_map = HashMap::<(usize,usize), usize>::with_capacity(screen_points_with_depth.len());
    for (i,(screen_p,depth)) in screen_points_with_depth.iter().enumerate() {
        let key = (screen_p.x,screen_p.y);
        match closest_point_map.contains_key(&key) {
            true => {
                let current_point_index = closest_point_map.get(&key).unwrap();
                let current_point_depth = screen_points_with_depth[*current_point_index].1;
                // GLTF models are displayed along the negative Z-Axis
                if *depth > current_point_depth {
                    closest_point_map.insert(key, i);
                }
            },
            false => {closest_point_map.insert(key, i);()}
        }
    }
    closest_point_map.into_values().map(|i| screen_points_with_depth[i].0).collect()
}
