extern crate models_cv;
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
    let positions_raw = models_cv::load_position_vectors(position_buffer_info, &buffers);
    Ok(())
}