use std::error::Error as StdError;

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        run(&path).expect("runtime error");
    } else {
        println!("usage: gltf-display <FILE>");
    }
}

fn run(path: &str) -> Result<(), Box<dyn StdError>> {
    let (document, buffers) = models_cv::gltf::load(path);
    let _ = models_cv::gltf::load_vertex_positions(&document,&buffers);
    Ok(())
}