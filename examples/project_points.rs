extern crate models_cv;
extern crate nalgebra as na;

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
    
    //let eye = Point3::new(0.0,0.0,-5.0);
    let eye = Point3::new(-2.0,0.0,4.5);
    let at = Point3::new(scene_center.x,scene_center.y,scene_center.z);
    let view_matrix = Isometry3::look_at_rh(&eye, &at, &Vector3::y_axis()).to_matrix();
    let screen_width = 640.0;
    let screen_height = 480.0;
    let f = 1000.0; 
    let cx = screen_width/2.0;
    let cy = screen_height/2.0;
    let intrinsic_matrix = Matrix3::<f32>::new(
        f,0.0,cx,
        0.0,f,cy,
        0.0,0.0,1.0);
    
    let (points_screen, points_cam) = models_cv::project_points(points, &intrinsic_matrix, &view_matrix.fixed_view::<3,4>(0, 0).into_owned(),screen_width, screen_height);
    //let visible_screen_points = models_cv::filter::filter_visible_screen_points_by_depth(&points_screen,&points_cam);
    let visible_screen_points = models_cv::filter::filter_visible_screen_points_by_triangle_intersection(&points_screen,&points_cam,&intrinsic_matrix);
    let data_vec = models_cv::calculate_rgb_byte_vec(&visible_screen_points, screen_width as usize, screen_height as usize);
    models_cv::write_data_to_file("/home/marc/Workspace/Rust/models-cv/output/test_suzanne.png", &data_vec,screen_width as u32, screen_height as u32).expect("Writing png failed!");


}