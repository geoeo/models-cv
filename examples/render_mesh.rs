extern crate nalgebra as na;

use kiss3d::event::{WindowEvent , Key};
use kiss3d::window::Window;
use kiss3d::nalgebra::{Point2, Point3};
use kiss3d::camera::{ArcBall, Camera};
use kiss3d::text::Font;
use kiss3d::resource::Mesh;
use na::Vector3;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        if path.ends_with(".gltf") {
            let (document, buffers) = models_cv::gltf::load(&path);
            let points = models_cv::gltf::load_vertex_positions(&document,&buffers);
            render_mesh(&points);
        } else if path.ends_with(".obj") {
            let model = models_cv::obj::load(&path);
            let points = models_cv::obj::load_vertex_positions(&model);
            render_mesh(&points);
        }
    } else {
        println!("usage: gltf-display <FILE>");
    }
}

fn render_mesh(points: &Vec<Vec<Vector3<f32>>>) -> () {
    let scene_capacity: usize = points.iter().map(|vec| vec.iter().map(|ps| ps.len())).flatten().sum();

    let mut window = Window::new("Gltf Model");
    let mut scene_center = Vector3::<f32>::new(0.0, 0.0, 0.0);
    println!("WARN: Assuming Triangle Mode!");

    for vertices in points {
        let vertices_kiss3d = vertices.into_iter().map(|v| {
            scene_center.x += v.x;
            scene_center.y += v.y;
            scene_center.z += v.z;
            Point3::new(v.x,v.y,v.z)
        }).collect::<Vec<_>>();


        let indices = (0..vertices_kiss3d.len()-2).step_by(3).map(|i| i as u16).map(|i| Point3::new(i,i+1,i+2)).collect::<Vec<_>>();
        let mesh = Rc::new(RefCell::new(Mesh::new(
            vertices_kiss3d, indices, None, None, false,
        )));
        let mut c = window.add_mesh(mesh, kiss3d::nalgebra::Vector3::new(1.0, 1.0, 1.0));
        c.set_color(1.0, 1.0, 1.0);
        c.enable_backface_culling(true);

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
                arc_ball.at(), arc_ball.eye()
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