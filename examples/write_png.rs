extern crate models_cv;

fn main() {
    let data_vec = vec![255, 0, 0, 0, 0, 0, 0, 255 ,0, 0,0,255]; // An array containing a RGB sequence. First pixel is red and second pixel is black.
    models_cv::io::write_data_to_file("/home/marc/Workspace/Rust/models-cv/output/test.png",&data_vec,2,2).unwrap();
}