extern crate nalgebra as na;

use models_cv::io::{serialize_feature_matches,deserialize_feature_matches};
use na::{Vector3,Isometry3,Point3, Matrix3};

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        let (points,names) = load_data(&path);
        let name = names.first().expect("No name for mesh");
        project_points(&points.into_iter().flatten().collect::<Vec<_>>(), name);
    } else {
        println!("usage: gltf-display <FILE>");
    }
}

fn load_data(path: &str) -> (Vec<Vec<Vector3<f32>>>, Vec<String>) {
    let (document,buffers,_) = gltf::import(path).expect("Could not load gltf file");
    let position_buffer_info = models_cv::find_position_buffer_data(&document);
    let positions_byte_data = models_cv::load_position_byte_data(position_buffer_info, &buffers);
    let names = document.meshes().into_iter().map(|m| m.name().expect("no name for mesh").to_string()).collect::<Vec<String>>();
    (models_cv::convert_byte_data_to_vec3(positions_byte_data), names)
}

fn project_points(points: &Vec<Vector3<f32>>, mesh_name: &String) -> () {
    let scene_capacity: usize = points.len();

    let mut scene_center = Vector3::<f32>::new(0.0, 0.0, 0.0);

    for point in points {
        scene_center += point;
    } 

    scene_center *= 1.0/scene_capacity as f32;
    
    let eyes = vec![Point3::new(0.0,0.0,5.0),Point3::new(-2.0,0.0,4.5),Point3::new(0.0,0.0,-5.0)];
    
    let at = Point3::new(scene_center.x,scene_center.y,scene_center.z);
    let view_matrices = eyes.iter().map(|eye| {
        let view_matrix = Isometry3::look_at_rh(&eye, &at, &Vector3::y_axis()).to_matrix();
        view_matrix.fixed_view::<3,4>(0, 0).into_owned()
    }).collect::<Vec<_>>();
    let screen_width = 640.0;
    let screen_height = 480.0;
    let f = 1000.0; 
    let cx = screen_width/2.0;
    let cy = screen_height/2.0;
    let intrinsic_matrix = Matrix3::<f32>::new(
        f,0.0,cx,
        0.0,f,cy,
        0.0,0.0,1.0);
    let intrinsic_matrices = vec![intrinsic_matrix;view_matrices.len()];
    
    let visible_screen_points_with_idx 
        = models_cv::filter_screen_points_for_camera_views(
            points,&intrinsic_matrix,
            &view_matrices,
            screen_width,
            screen_height,
            models_cv::filter::FilterType::TriangleIntersection
        );
    
    let camera_features = models_cv::generate_matches(&view_matrices,&intrinsic_matrices, &visible_screen_points_with_idx);
    let path = format!("/home/marc/Workspace/Rust/models-cv/output/camera_features_{}.yaml",mesh_name);
    serialize_feature_matches(&path, &camera_features).expect("Serialzing failed");
    let loaded_data = deserialize_feature_matches(&path);
    assert_eq!(camera_features,loaded_data);


}