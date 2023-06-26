extern crate nalgebra as na;

use na::{Vector2, Vector3, Matrix3xX, Matrix3};
use std::collections::{HashMap,HashSet};

const STD_EPS: f32 = 1e-1; 
const BARY_EPS: f64 = 1e-12;
const DET_EPS: f64 = 1e-8;

pub fn filter_visible_screen_points_by_depth(points_screen: &Vec<Vector2<usize>>, points_cam: &Matrix3xX<f32>) -> Vec<Vector2<usize>> {
    assert_eq!(points_screen.len(),points_cam.ncols());
    let mut closest_point_map = HashMap::<(usize,usize), usize>::with_capacity(points_screen.len());
    for (i,screen_p) in points_screen.iter().enumerate() {
        let key = (screen_p.x,screen_p.y);
        match closest_point_map.contains_key(&key) {
            true => {
                let current_point_index = closest_point_map.get(&key).unwrap();
                let current_point_depth = points_cam[*current_point_index];
                let depth = points_cam[(2,i)];
                // GLTF models are displayed along the negative Z-Axis
                if depth > current_point_depth {
                    closest_point_map.insert(key, i);
                }
            },
            false => {closest_point_map.insert(key, i);()}
        }
    }
    closest_point_map.into_values().map(|i| points_screen[i]).collect()
}

pub fn filter_visible_screen_points_by_triangle_intersection(screen_points_with_depth: &Vec<Vector2<usize>>, points_cam: &Matrix3xX<f32>, intrinsic_matrix: &Matrix3<f32>) -> Vec<Vector2<usize>> {
    let screen_point_set = screen_points_with_depth.iter().map(|v| (v.x,v.y)).collect::<HashSet<(usize,usize)>>();
    let fx = intrinsic_matrix[(0,0)] as f64;
    let fy = intrinsic_matrix[(1,1)] as f64;
    let cx = intrinsic_matrix[(0,2)] as f64;
    let cy = intrinsic_matrix[(1,2)] as f64;

    let mut avg_x = 0.0; let mut avg_y = 0.0; let mut avg_z = 0.0;
    let ncols_f32 = points_cam.ncols() as f32;

    for c in points_cam.column_iter() {
        avg_x += c.x;
        avg_y += c.y;
        avg_z += c.z;
    }

    avg_x /= ncols_f32;
    avg_y /= ncols_f32;
    avg_z /= ncols_f32;

    let mut std_x = 0.0; let mut std_y = 0.0; let mut std_z = 0.0;

    for c in points_cam.column_iter() {
        std_x += (c.x - avg_x).powi(2);
        std_y += (c.y - avg_y).powi(2);
        std_z += (c.z - avg_z).powi(2);
    }

    std_x = (std_x/(ncols_f32-1.0)).sqrt();
    std_y = (std_y/(ncols_f32-1.0)).sqrt();
    std_z = (std_z/(ncols_f32-1.0)).sqrt();

    let std_x_pix = std_x as f64/fx*(2.0f64*cx);
    let std_y_pix = std_y as f64/fy*(2.0f64*cy);
    let std_xy_avg = (std_x_pix+std_y_pix)/2.0f64;
    let exp = ((0.5-std_xy_avg).abs()*10.0).floor();

    //TODO: Implement depth buffering for a better result than a heuristic
    let l2_eps = 10.0f64.powf(-exp);

    let sub_pix_res = 10;
    let triangles = (0..points_cam.ncols()-2).step_by(3).map(|i| (points_cam.column(i).into_owned(),points_cam.column(i+1).into_owned(),points_cam.column(i+2).into_owned())).collect::<Vec<(_,_,_)>>();
    screen_point_set.iter().filter(|(u,v)| {
        (0..sub_pix_res).map (|off| {
            let frac = off as f64/sub_pix_res as f64;
            let u_f64 = *u as f64 + frac;
            let v_f64 = *v as f64 + frac;
            // Direction is just pixel back-projected along -Z-Axis
            let dir = (Vector3::<f64>::new(-1.0*(u_f64-cx)/fx,-1.0*(v_f64-cy)/fy,-1.0)).normalize();
            let orig = Vector3::<f64>::zeros();
            let ts = triangles.iter().map(|(v0,v1,v2)| {
                let v0_f64 = v0.cast::<f64>();
                let v1_f64 = v1.cast::<f64>();
                let v2_f64 = v2.cast::<f64>();
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
    }).map(|&(u,v)| Vector2::<usize>::new(u,v)).collect::<Vec<_>>()
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