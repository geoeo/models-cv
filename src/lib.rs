extern crate nalgebra as na;

pub mod camera_features;
pub mod filter;
pub mod io;
pub mod gltf;

use std::iter::zip;
use std::collections::HashMap;
use na::{Vector2,Vector3,Matrix3,Matrix4xX,Matrix3x4, Matrix3xX};


pub fn project_points(points: &Vec<Vector3<f32>>, intrinsic_matrix: &Matrix3<f32>, view_matrix: &Matrix3x4<f32>, screen_width: f32, screen_height: f32) -> (Vec<(usize,Vector2<usize>)>, Matrix3xX<f32>) {
    let mut ps = Matrix4xX::<f32>::from_element(points.len(), 1.0);
    for i in 0..points.len() {
        ps.fixed_view_mut::<3,1>(0,i).copy_from(&points[i]);
    }
    let points_cam = view_matrix*ps;
    let projected_points = intrinsic_matrix*&points_cam;
    let screen_points_with_idx = projected_points.column_iter().enumerate()
        .filter(|(_,c)| c[2] != 0.0)
        .map(|(i,c)| (i,(c[0]/c[2],c[1]/c[2])))
        .filter(|&(_,(x,y))| 0.0 <= x && 0.0 <= y && x < screen_width && y < screen_height)
        .map(|(i,(x,y))| (i,Vector2::new(x.floor() as usize, y.floor() as usize)))
        .collect::<Vec<_>>();
    (screen_points_with_idx, points_cam)
}

pub fn filter_screen_points_for_camera_views(points: &Vec<Vector3<f32>>, intrinsic_matrix: &Matrix3<f32>, view_matrices: &Vec<Matrix3x4<f32>>, screen_width: f32, screen_height: f32, filter_type: filter::FilterType) -> Vec<Vec<(usize,Vector2<usize>)>> {
    view_matrices.iter().map(|view_matrix| {
        let (points_screen_with_idx, points_cam) = project_points(points, &intrinsic_matrix, &view_matrix.fixed_view::<3,4>(0, 0).into_owned(),screen_width, screen_height);
        match filter_type {
            filter::FilterType::Depth => filter::filter_visible_screen_points_by_depth(&points_screen_with_idx,&points_cam),
            filter::FilterType::TriangleIntersection => filter::filter_visible_screen_points_by_triangle_intersection(&points_screen_with_idx,&points_cam,&intrinsic_matrix)
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


