use std::error::Error as StdError;

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        run(&path).expect("runtime error");
    } else {
        println!("usage: gltf-display <FILE>");
    }
}

fn run(path: &str) -> Result<(), Box<dyn StdError>> {
    let (document,buffers,_) = gltf::import(path)?;
    let position_buffer_info = models_cv::find_position_buffer_data(&document);
    let positions_byte_data = models_cv::load_position_byte_data(position_buffer_info, &buffers);
    let _ = models_cv::convert_byte_data_to_vec3(positions_byte_data);
    Ok(())
}