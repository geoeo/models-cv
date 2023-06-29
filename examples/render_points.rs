extern crate nalgebra as na;

use kiss3d::event::{WindowEvent , Key};
use kiss3d::window::Window;
use kiss3d::nalgebra::{Point2, Point3,Translation3};
use kiss3d::camera::{ArcBall, Camera};
use kiss3d::text::Font;
use na::Vector3;

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        let points = load_points(&path);
        render_points(&points);
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

fn render_points(points: &Vec<Vec<Vector3<f32>>>) -> () {
    let scene_capacity: usize = points.iter().map(|vec| vec.iter().map(|ps| ps.len())).flatten().sum();

    let mut window = Window::new("Gltf Model");
    let mut scene_nodes = Vec::<kiss3d::scene::SceneNode>::with_capacity(scene_capacity);
    
    let mut scene_center = Vector3::<f32>::new(0.0, 0.0, 0.0);

    for point_vec in points {
        for point in point_vec {
            let mut s = window.add_sphere(0.01);
            s.set_color(1.0, 1.0, 1.0);
            scene_center += point;
            s.append_translation(&Translation3::new(point.x, point.y, point.z));
            scene_nodes.push(s);
        } 
    }

    scene_center *= 1.0/scene_capacity as f32;
    
    let eye = Point3::new(0.0,0.0,5.0);
    let at = Point3::new(scene_center.x,scene_center.y,scene_center.z);
    let mut arc_ball = ArcBall::new(eye, at);
    arc_ball.set_dist_step(1.0);

    while window.render_with_camera(&mut arc_ball) {

        window.draw_text(
            &format!(
                "Cam Loot At: {}\nCam Pos: {}",
                arc_ball.eye(), arc_ball.at()
            ),
            &Point2::new(0.0, 20.0),
            40.0,
            &Font::default(),
            &Point3::new(1.0, 1.0, 1.0),
        );

        for event in window.events().iter() {
            match event.value {
                WindowEvent::Key(Key::R, _, _) => {
                    arc_ball.look_at(eye, at)
                },
                WindowEvent::Key(Key::Left, _, _) => {
                    arc_ball.set_up_axis_dir(-kiss3d::nalgebra::Vector3::x_axis());
                },
                WindowEvent::Key(Key::Right, _, _) => {
                    arc_ball.set_up_axis_dir(kiss3d::nalgebra::Vector3::x_axis());
                },
                WindowEvent::Key(Key::Up, _, _) => {
                    arc_ball.set_up_axis_dir(kiss3d::nalgebra::Vector3::y_axis());
                },
                WindowEvent::Key(Key::Down, _, _) => {
                    arc_ball.set_up_axis_dir(-kiss3d::nalgebra::Vector3::y_axis());
                },
                _ => ()
            }
        }
    }

}