extern crate nalgebra as na;

pub mod camera_features;
pub mod landmark;
pub mod filter;
pub mod io;
pub mod gltf;
pub mod obj;
pub mod rasterizer;
pub mod triangle;

use std::iter::zip;
use std::collections::HashMap;

use na::{Vector2,Vector3,Matrix3,Matrix4xX,Matrix3x4, Matrix3xX, Point3};
use triangle::Triangle;

/**
 * Returns A vector of indexed points in image space where the index represents the column of the corresponding 3D point matrix in camera space
 */
pub fn project_points(indexed_landmarks: &Vec<landmark::Landmark>, intrinsic_matrix: &Matrix3<f32>, view_matrix: &Matrix3x4<f32>) -> (Vec<(usize,Vector2<f32>)>, Matrix3xX<f32>) {
    let mut ps = Matrix4xX::<f32>::from_element(indexed_landmarks.len(), 1.0);

    for i in 0..indexed_landmarks.len() {
        let p = &indexed_landmarks[i].get_position();
        ps.fixed_view_mut::<3,1>(0,i).copy_from(p);
    }
    let points_cam = view_matrix*(&ps);
    let projected_points = intrinsic_matrix*&points_cam;

    let screen_points_with_idx = projected_points.column_iter().enumerate()
        .map(|(i,c)|{
            let landmark_id = *indexed_landmarks[i].get_id();
            (landmark_id,c)
        } )
        .filter(|(_,c)| c.z != 0.0)
        .map(|(i,c)| (i,(c.x/c.z,c.y/c.z)))
        .map(|(i,(x,y))| (i,Vector2::new(x, y)))
        .collect::<Vec<_>>();
    (screen_points_with_idx, points_cam)
}

pub fn group_points_to_triangles(pixels_with_id: &Vec<(usize,Vector2<f32>)>, cam_points: &Matrix3xX<f32>) -> Vec<(Triangle<2>,Triangle<3>)> {
    (0..pixels_with_id.len()-2).step_by(3).map(|i| {
        let (id_0,pix_v0) = pixels_with_id[i];
        let (id_1,pix_v1) = pixels_with_id[i+1];
        let (id_2,pix_v2) = pixels_with_id[i+2];
        let cam_point_v0 = cam_points.column(id_0);
        let cam_point_v1 = cam_points.column(id_1);
        let cam_point_v2 = cam_points.column(id_2);
        (
            Triangle::from_vec(&pix_v0.cast::<f32>(), Some(id_0) ,&pix_v1.cast::<f32>(), Some(id_1), &pix_v2.cast::<f32>(), Some(id_2)),
            Triangle::from_view(&cam_point_v0, Some(id_0), &cam_point_v1, Some(id_1), &cam_point_v2, Some(id_2))
        )
    }).collect::<Vec<_>>()
}

pub fn filter_screen_points_for_camera_views(indexed_landmarks: &Vec<landmark::Landmark>, intrinsic_matrix: &Matrix3<f32>, view_matrices: &Vec<Matrix3x4<f32>>, screen_width: f32, screen_height: f32, filter_type: filter::FilterType) -> Vec<Vec<(usize,Vector2<usize>)>> {
    view_matrices.iter().map(|view_matrix| {
        let (points_screen_with_idx, points_cam) = project_points(indexed_landmarks, &intrinsic_matrix, &view_matrix.fixed_view::<3,4>(0, 0).into_owned());
        match filter_type {
            filter::FilterType::Depth => filter::filter_visible_screen_points_by_depth(&points_screen_with_idx,&points_cam),
            filter::FilterType::Rasterizer => {
                let screen_cam_triangles = group_points_to_triangles(&points_screen_with_idx,&points_cam);
                filter::filter_visible_screen_points_by_rasterizer(&screen_cam_triangles,screen_width, screen_height)
            }
        }
    }).collect::<Vec<_>>()
}


pub fn generate_matches(view_matrices: &Vec<Matrix3x4<f32>>, intrinsic_matrices: &Vec<Matrix3<f32>>, features: &Vec<Vec<(usize,Vector2<usize>)>>) -> Vec<camera_features::CameraFeatures> {
    assert_eq!(view_matrices.len(), features.len());
    zip(zip(view_matrices,intrinsic_matrices),features).enumerate().map(|(cam_id,((view_matrix,intrinsic_matrix),screen_points_with_id))| {
        let point_map = screen_points_with_id.into_iter().map(|&(k,v)| (k,v)).collect::<HashMap<usize,Vector2<usize>>>();
        camera_features::CameraFeatures::new(point_map,cam_id,view_matrix.clone(),intrinsic_matrix.clone()) 
    }).collect()
}

pub fn generate_landmarks(indexed_points: &Vec<(usize,Vector3<f32>)>) -> Vec<landmark::Landmark> {
    indexed_points.iter().map(|(i,p)| {
        landmark::Landmark::new(i,p) 
    }).collect()
}

pub fn generate_camera_trajectory(start: &Point3<f32>, target: &Point3<f32>, arc_angle: f32, step_count: usize) -> Vec<Point3<f32>> {
    assert!(arc_angle > 0.0 && arc_angle <= 360.0);
    let pos = start-target;
    let r = pos.iter().map(|v| v.powi(2)).sum::<f32>().sqrt();
    let theta = (pos.y/r).acos();
    let phi = pos.z.atan2(pos.x);
    let arc_angle_rad = arc_angle * std::f32::consts::PI/180.0;

    (0..step_count+1).map(|s|{
        let ratio = (s as f32) / (step_count as f32);
        let rad_offset = ratio*arc_angle_rad;
        let p = phi + rad_offset;
        let x_new = target.x + r*theta.sin()*p.cos();
        let y_new = target.y;
        let z_new = target.x + r*theta.sin()*p.sin();
        Point3::<f32>::new(x_new,y_new,z_new)
    }).collect()
}


