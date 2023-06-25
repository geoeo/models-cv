extern crate gltf;
extern crate nalgebra as na;
extern crate png;

mod byte_array_info;
pub mod filter;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::result::Result;

use byte_array_info::ByteArrayInfo;
use na::{Vector2,Vector3,Matrix3,Matrix4xX,Matrix3x4, Matrix3xX};
use png::EncodingError;

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

pub fn project_points(points: &Vec<Vector3<f32>>, intrinsic_matrix: &Matrix3<f32>, view_matrix: &Matrix3x4<f32>, screen_width: f32, screen_height: f32) -> (Vec<(Vector2<usize>,f32)>, Matrix3xX<f32>) {
    let mut ps = Matrix4xX::<f32>::from_element(points.len(), 1.0);
    for i in 0..points.len() {
        ps.fixed_view_mut::<3,1>(0,i).copy_from(&points[i]);
    }
    let points_cam = view_matrix*ps;
    let projected_points = intrinsic_matrix*&points_cam;
    let screen_points = projected_points.column_iter()
        .filter(|c| c[2] != 0.0)
        .map(|c| (c[0]/c[2],c[1]/c[2], c[2]))
        .filter(|&(x,y,_)| 0.0 <= x && 0.0 <= y && x < screen_width && y < screen_height)
        .map(|(x,y,z)| (Vector2::new(x.floor() as usize, y.floor() as usize),z))
        .collect::<Vec<_>>();
    (screen_points, points_cam)
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

pub fn write_data_to_file(path_str: &str, data_vec: &Vec<u8>, screen_width: u32, screen_height: u32) -> Result<(),EncodingError> {
    let path = Path::new(path_str);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, screen_width, screen_height); 
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000)
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data_vec[..]) // Save
}
