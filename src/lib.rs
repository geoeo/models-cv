use std::collections::HashSet;
extern crate gltf;

/**
 * Returns (buffer index, offset,length,Option<stride>) of position buffers
 */
pub fn find_position_buffer_data(document: &gltf::Document) -> Vec<(usize, usize, usize,Option<usize>)> { 
    // meshes -> primitives -> attributes
    let position_attribute_indices : HashSet<_> = document.meshes().map(|mesh| {
        mesh.primitives().map(|primitive| {
            primitive.attributes().filter(|attribute| attribute.0 == gltf::Semantic::Positions).map(|attr| attr.1.index())
        }).flatten()
    }).flatten().collect();
    document.views().enumerate().filter(|(i,_)| position_attribute_indices.contains(i)).map(|(_,v)| (v.buffer().index(),v.offset(),v.length(),v.stride())).collect()
}

pub fn load_position_vectors(position_buffer_info: Vec<(usize, usize, usize,Option<usize>)>, buffers: &Vec<gltf::buffer::Data>) -> Vec<&[u8]> {
    position_buffer_info.into_iter().map(|(buffer_index,offset,length,option_stride)| {
        let byte_end = offset+length;
        match option_stride {
            None => &buffers[buffer_index].0[offset..byte_end],
            Some(_) => panic!("TODO Strided Position Loading") // After every read, continue for stide - size bytes
        }
    }).collect()
}
