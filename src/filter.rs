extern crate nalgebra as na;

use na::{Vector2, Vector3, Matrix3xX, Matrix3};
use std::collections::HashMap;
use crate::triangle::Triangle;

const BARY_EPS: f64 = 1e-1;
const DET_EPS: f64 = 1e-8;

pub enum FilterType {
    Depth,
    TriangleIntersection
}

pub fn filter_visible_screen_points_by_depth(screen_points_with_index: &Vec<(usize,Vector2<usize>)>, points_cam: &Matrix3xX<f32>,) -> Vec<(usize,Vector2<usize>)> {
    let mut closest_point_map = HashMap::<(usize,usize), usize>::with_capacity(screen_points_with_index.len());
    for (i,&(global_id,screen_p)) in screen_points_with_index.iter().enumerate() {
        let key = (screen_p.x,screen_p.y);
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
    closest_point_map.into_values().map(|i| screen_points_with_index[i]).collect()
}

pub fn filter_visible_screen_points_by_triangle_intersection(screen_points_with_index: &Vec<(usize,Vector2<usize>)>, points_cam: &Matrix3xX<f32>, intrinsic_matrix: &Matrix3<f32>) -> Vec<(usize,Vector2<usize>)> {
    let fx = intrinsic_matrix[(0,0)] as f64;
    let fy = intrinsic_matrix[(1,1)] as f64;
    let cx = intrinsic_matrix[(0,2)] as f64;
    let cy = intrinsic_matrix[(1,2)] as f64;


    //TODO uniform eps
    let l2_eps = 1e-2; // Suzanne
    //let l2_eps = 1e1; // Cube, Sphere

    let sub_pix_res = 1;
    let triangles = (0..points_cam.ncols()-2).step_by(3).map(|i| Triangle::<3>::from_view(&points_cam.column(i),&points_cam.column(i+1),&points_cam.column(i+2))).collect::<Vec<Triangle<3>>>();
    screen_points_with_index.iter().filter(|(_,screen_point)| {
        let u = screen_point.x;
        let v = screen_point.y;
        (0..sub_pix_res).map (|off| {
            let frac = off as f64/sub_pix_res as f64;
            let u_f64 = u as f64 + frac;
            let v_f64 = v as f64 + frac;
            // Direction is just pixel back-projected along -Z-Axis
            let dir = (Vector3::<f64>::new(-1.0*(u_f64-cx)/fx,-1.0*(v_f64-cy)/fy,-1.0)).normalize();
            let orig = Vector3::<f64>::zeros();
            let ts = triangles.iter().map(|triangle| {
                let v0_f64 = triangle.get_v0().cast::<f64>();
                let v1_f64 = triangle.get_v1().cast::<f64>();
                let v2_f64 = triangle.get_v2().cast::<f64>();
                ray_triangle_intersection(&orig, &dir, &v0_f64, &v1_f64, &v2_f64)
            }).filter(|o| o.is_some()).collect::<Vec<_>>();
            match ts.len() {
                0 => false,
                _ => {
                    let t = ts.iter().map(|o| o.unwrap()).reduce(f64::min).expect("Computeration of intersection param t failed!");
                    let p = t*dir;
                    let smallest_norm_opt = points_cam.column_iter().map(|c| (c.into_owned().cast::<f64>()-p).norm()).reduce(f64::min);
                    match smallest_norm_opt {
                        None => false,
                        Some(v) if v > l2_eps => false,
                        Some(v) if v <= l2_eps => true,
                        Some(_) => false
                    }
                }
            }
        }).any(|b| b)
    }).map(|&(i,screen_point)| (i,screen_point)).collect::<Vec<_>>()
}

//https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection.html
pub fn ray_triangle_intersection(orig: &Vector3<f64>, dir: &Vector3<f64>, v0: &Vector3<f64>, v1: &Vector3<f64>, v2: &Vector3<f64>) -> Option<f64> {
    let v0v1 = v1 - v0;
    let v0v2 = v2 - v0;
    let p_vec = dir.cross(&v0v2);
    let det = v0v1.dot(&p_vec);

    match det {
        det if det < DET_EPS => None,
        det => {
            let inv_det = 1.0 / det;
            let t_vec = orig-v0;
            let u = t_vec.dot(&p_vec)*inv_det;
            let q_vec = t_vec.cross(&v0v1);
            let v = dir.dot(&q_vec)*inv_det;

            match (u,v) {
                (u,v) if u < -BARY_EPS || u > 1.0 + BARY_EPS || v < -BARY_EPS || u+v > 1.0 + 2.0*BARY_EPS => None,
                _ => Some(v0v2.dot(&q_vec)*inv_det)
            }
        }
    }
}