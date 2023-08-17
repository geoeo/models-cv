extern crate nalgebra as na;

use na::{Vector2, Matrix3xX};
use std::collections::HashMap;
use crate::triangle::Triangle;
use crate::rasterizer;


pub enum FilterType {
    Depth,
    Rasterizer
}

pub fn filter_visible_screen_points_by_depth(screen_points_with_index: &Vec<(usize,Vector2<f32>)>, points_cam: &Matrix3xX<f32>,) -> Vec<(usize,Vector2<usize>)> {
    let mut closest_point_map = HashMap::<(usize,usize), usize>::with_capacity(screen_points_with_index.len());
    for (i,&(global_id,screen_p)) in screen_points_with_index.iter().filter(|(_,p)| p.x > 0.0 && p.y >= 0.0).enumerate() {
        let key = (screen_p.x.floor() as usize,screen_p.y.floor() as usize);
        match closest_point_map.contains_key(&key) {
            true => {
                let current_point_index = closest_point_map.get(&key).unwrap();
                let current_point_depth = points_cam[*current_point_index];
                let depth = points_cam[(2,global_id)];
                // GLTF models are displayed along the negative Z-Axis
                if depth > current_point_depth  {
                    closest_point_map.insert(key, i);
                }
            },
            false => {closest_point_map.insert(key, i);()}
        }
    }
    closest_point_map.into_values().map(|i| screen_points_with_index[i]).map(|(i,v)| (i, Vector2::new(v.x.floor() as usize, v.y.floor() as usize))).collect()
}

pub fn filter_visible_screen_points_by_rasterizer(screen_cam_triangles: &Vec<(Triangle<2>,Triangle<3>)>, screen_width: f32, screen_height: f32) -> Vec<(usize,Vector2<usize>)> {
    assert!(screen_height.fract() <= f32::EPSILON);
    assert!(screen_width.fract() <= f32::EPSILON);
    let mut depth_buffer = HashMap::<(usize,usize),(f32,Option<(usize,Vector2<usize>)>)>::with_capacity((screen_height*screen_width) as usize);
    for (tri_2d,tri_3d) in screen_cam_triangles.iter() {
        let barycentric_coordiantes_with_pixel = rasterizer::calc_all_pixels_within_triangle(tri_2d);
        let barycentric_coordiantes = barycentric_coordiantes_with_pixel.iter().map(|(w0,w1,w2,_)| (*w0,*w1,*w2)).collect::<Vec<_>>();
        let pixel_depths = rasterizer::calc_inv_z_for_all_pixels(&barycentric_coordiantes,tri_3d);
        let mut triangle_association_map = HashMap::<(usize,usize),usize>::with_capacity(3);
        triangle_association_map.insert((tri_2d.get_v0().x.floor() as usize,tri_2d.get_v0().y.floor() as usize), tri_2d.get_id0().expect("Expected id for v0!"));
        triangle_association_map.insert((tri_2d.get_v1().x.floor() as usize,tri_2d.get_v1().y.floor() as usize), tri_2d.get_id1().expect("Expected id for v1!"));
        triangle_association_map.insert((tri_2d.get_v2().x.floor() as usize,tri_2d.get_v2().y.floor() as usize), tri_2d.get_id2().expect("Expected id for v2!"));
        for i in 0..pixel_depths.len() {
            let depth = pixel_depths[i];
            assert!(depth < 0.0);
            let pixel = barycentric_coordiantes_with_pixel[i].3;
            let key = (pixel.x.floor() as usize,pixel.y.floor() as usize);
            let pixel_u = Vector2::new(key.0,key.1);
            let pixel_is_vertex = triangle_association_map.contains_key(&key);
            match (depth_buffer.contains_key(&key), pixel_is_vertex) {
                (false,false) => {
                    depth_buffer.insert(key.clone(), (depth,None));
                    ()
                },
                (false,true) => {
                    depth_buffer.insert(key.clone(), (depth,Some((*triangle_association_map.get(&key).unwrap(),pixel_u))));
                    ()
                },
                (true,false) => {
                    let &(current_depth,_v) = depth_buffer.get(&key).unwrap();
                    assert!(current_depth < 0.0);
                    // GLTF is defined along -Z
                    if depth > current_depth {
                        depth_buffer.insert(key.clone(), (depth,None));
                    }
                },
                (true,true) => {
                    let &(current_depth,_v) = depth_buffer.get(&key).unwrap();
                    assert!(current_depth < 0.0);
                    // GLTF is defined along -Z
                    if depth > current_depth {
                        depth_buffer.insert(key.clone(), (depth,Some((*triangle_association_map.get(&key).unwrap(),pixel_u))));
                    }
                }
            }
        }      
    }
    depth_buffer.values().into_iter().filter(|(_, some_v)| some_v.is_some()).map(|(_,some_v)|some_v.unwrap()).collect()
}