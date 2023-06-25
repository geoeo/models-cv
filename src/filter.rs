extern crate nalgebra as na;

use na::{Vector2, Vector3, Matrix3xX, Matrix3};
use std::collections::{HashMap,HashSet};

const L2_EPS: f32 = 5e-3; //TODO: This factor depends strongly on the mesh vertex spread
const BARY_EPS: f32 = 1e-8; 
const DET_EPS: f32 = 1e-8;

pub fn filter_visible_screen_points_by_depth(screen_points_with_depth: &Vec<(Vector2<usize>,f32)>) -> Vec<Vector2<usize>> {
    let mut closest_point_map = HashMap::<(usize,usize), usize>::with_capacity(screen_points_with_depth.len());
    for (i,(screen_p,depth)) in screen_points_with_depth.iter().enumerate() {
        let key = (screen_p.x,screen_p.y);
        match closest_point_map.contains_key(&key) {
            true => {
                let current_point_index = closest_point_map.get(&key).unwrap();
                let current_point_depth = screen_points_with_depth[*current_point_index].1;
                // GLTF models are displayed along the negative Z-Axis
                if *depth > current_point_depth {
                    closest_point_map.insert(key, i);
                }
            },
            false => {closest_point_map.insert(key, i);()}
        }
    }
    closest_point_map.into_values().map(|i| screen_points_with_depth[i].0).collect()
}

pub fn filter_visible_screen_points_by_triangle_intersection(screen_points_with_depth: &Vec<(Vector2<usize>,f32)>, points_cam: &Matrix3xX<f32>, intrinsic_matrix: &Matrix3<f32>) -> Vec<Vector2<usize>> {
    let screen_point_set = screen_points_with_depth.iter().map(|(v,_)| (v.x,v.y)).collect::<HashSet<(usize,usize)>>();
    let fx = intrinsic_matrix[(0,0)];
    let fy = intrinsic_matrix[(1,1)];
    let cx = intrinsic_matrix[(0,2)];
    let cy = intrinsic_matrix[(1,2)];
    let triangles = (0..points_cam.ncols()-2).step_by(3).map(|i| (points_cam.column(i).into_owned(),points_cam.column(i+1).into_owned(),points_cam.column(i+2).into_owned())).collect::<Vec<(_,_,_)>>();
    screen_point_set.iter().filter(|(u,v)| {
        let u_f32 = *u as f32 + 0.5;
        let v_f32 = *v as f32 + 0.5;
        // Direction is just pixel back-projected along -Z-Axis
        let dir = Vector3::<f32>::new(-1.0*(u_f32-cx)/fx,-1.0*(v_f32-cy)/fy,-1.0);
        let orig = Vector3::<f32>::zeros();
        let ts = triangles.iter().map(|(v0,v1,v2)| {
            ray_triangle_intersection(&orig, &dir, v0, v1, v2)
        }).filter(|o| o.is_some()).collect::<Vec<_>>();
        match ts.len() {
            0 => false,
            _ => {
                let t = ts.iter().map(|o| o.unwrap()).reduce(f32::min).expect("Computeration of intersection param t failed!");
                let p = t*dir;
                let smallest_norm_opt = points_cam.column_iter().map(|c| (c-p).norm()).reduce(f32::min);
                match smallest_norm_opt {
                    None => false,
                    Some(v) if v > L2_EPS => false,
                    Some(v) if v <= L2_EPS => true,
                    Some(_) => false
                }
            }
        }

    }).map(|&(u,v)| Vector2::<usize>::new(u,v)).collect::<Vec<_>>()
}

pub fn ray_triangle_intersection(orig: &Vector3<f32>, dir: &Vector3<f32>, v0: &Vector3<f32>, v1: &Vector3<f32>, v2: &Vector3<f32>) -> Option<f32> {
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