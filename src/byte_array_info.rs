extern crate gltf;
use gltf::accessor::{DataType,Dimensions};

pub struct ByteArrayInfo {
    data_type: DataType, 
    data_dimension: Dimensions,
    buffer_index: usize,
    byte_offset: usize,
    byte_length: usize,
    byte_stride: Option<usize>
}

impl ByteArrayInfo {
    pub fn new(
        data_type: DataType, 
        data_dimension: Dimensions,
        buffer_index: usize,
        byte_offset: usize,
        byte_length: usize,
        byte_stride: Option<usize>) 
            -> ByteArrayInfo {
            ByteArrayInfo {
                data_type,
                data_dimension,
                buffer_index,
                byte_offset,
                byte_length,
                byte_stride
            }
    }

    pub fn get_data_type(&self) -> DataType {self.data_type}
    pub fn get_data_dimension(&self) -> Dimensions {self.data_dimension}
    pub fn get_buffer_index(&self) -> usize {self.buffer_index}
    pub fn get_byte_offset(&self) -> usize {self.byte_offset}
    pub fn get_byte_length(&self) -> usize {self.byte_length}
    pub fn get_byte_stride(&self) -> Option<usize> {self.byte_stride}
}