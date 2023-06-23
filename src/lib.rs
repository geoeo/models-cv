extern crate gltf;
extern crate nalgebra as na;
mod byte_array_info;

use byte_array_info::ByteArrayInfo;
use na::Vector3;

/**
 * Returns a Vec<ByteArrayInfo> of position data
 */
pub fn find_position_buffer_data(document: &gltf::Document) -> Vec<ByteArrayInfo> { 
    document.meshes().map(|mesh| {
        mesh.primitives().map(|primitive| {
            primitive.attributes().filter(|attribute| attribute.0 == gltf::Semantic::Positions).map(|attr| {
                let buffer_view = attr.1.view().expect("Buffer is sparse. This is not implemented");
                ByteArrayInfo::new (attr.1.data_type(), attr.1.dimensions(), buffer_view.buffer().index(),buffer_view.offset(), buffer_view.length(), buffer_view.stride())
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
