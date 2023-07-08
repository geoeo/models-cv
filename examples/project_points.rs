extern crate nalgebra as na;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::result::Result;
use png::EncodingError;
use na::{Vector3,Isometry3,Point3, Matrix3};

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        let points = load_points(&path);
        project_points(&points.into_iter().flatten().collect::<Vec<_>>());
    } else {
        println!("usage: gltf-display <FILE>");
    }
}

fn load_points(path: &str) -> Vec<Vec<Vector3<f32>>> {
    let (document,buffers,_) = gltf::import(path).expect("Could not load gltf file");
    let position_buffer_info = models_cv::find_position_buffer_data(&document);
    let positions_byte_data = models_cv::load_position_byte_data(position_buffer_info, &buffers);
    models_cv::convert_byte_data_to_vec3(positions_byte_data)
}

fn project_points(points: &Vec<Vector3<f32>>) -> () {
    let scene_capacity: usize = points.len();

    let mut scene_center = Vector3::<f32>::new(0.0, 0.0, 0.0);

    for point in points {
        scene_center += point;
    } 

    scene_center *= 1.0/scene_capacity as f32;
    
    let eyes = vec![Point3::new(0.0,0.0,5.0),Point3::new(-2.0,0.0,5.0),Point3::new(0.0,0.0,-5.0)];
    
    let at = Point3::new(scene_center.x,scene_center.y,scene_center.z);
    let view_matrices = eyes.iter().map(|eye| {
        let view_matrix = Isometry3::look_at_rh(&eye, &at, &Vector3::y_axis()).to_matrix();
        view_matrix.fixed_view::<3,4>(0, 0).into_owned()
    }).collect::<Vec<_>>();
    let screen_width = 640.0;
    let screen_height = 480.0;
    let f = -1000.0; 
    let cx = screen_width/2.0;
    let cy = screen_height/2.0;
    let intrinsic_matrix = Matrix3::<f32>::new(
        f,0.0,cx,
        0.0,f,cy,
        0.0,0.0,1.0);
    
    let visible_screen_points_with_idx 
        = models_cv::filter_screen_points_for_camera_views(
            points,&intrinsic_matrix,
            &view_matrices,
            screen_width,
            screen_height,
            models_cv::filter::FilterType::TriangleIntersection
        );
    for (camera_id, visible_points_for_cam) in visible_screen_points_with_idx.iter().enumerate() {
        let visible_screen_points = visible_points_for_cam.iter().map(|&(_,v)| v).collect::<Vec<_>>();
        let data_vec = models_cv::calculate_rgb_byte_vec(&visible_screen_points, screen_width as usize, screen_height as usize);
        let name = format!("/home/marc/Workspace/Rust/models-cv/output/test_suzanne_{}.png",camera_id+1);
        write_png_data_to_file(name.as_str(), &data_vec,screen_width as u32, screen_height as u32).expect("Writing png failed!");
        write_test_png();
    }

}

fn write_png_data_to_file(path_str: &str, data_vec: &Vec<u8>, screen_width: u32, screen_height: u32) -> Result<(),EncodingError> {
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

fn write_test_png() -> () {
    let data_vec = vec![255, 0, 0, 0, 0, 0, 0, 255 ,0, 0,0,255]; // An array containing a RGB sequence. First pixel is red and second pixel is black.
    write_png_data_to_file("/home/marc/Workspace/Rust/models-cv/output/test.png",&data_vec,2,2).unwrap();
}