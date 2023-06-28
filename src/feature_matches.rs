extern crate nalgebra as na;
use na::{Vector2,Matrix3x4};

use std::collections::HashMap;

pub struct FeatureMatches {
    match_map: HashMap<usize,Vector2<usize>>,
    cam_id: usize,
    view_matrix: Matrix3x4<f32>
}

impl FeatureMatches {
    pub fn new(match_map: HashMap<usize,Vector2<usize>>, cam_id: usize, view_matrix: Matrix3x4<f32>) -> FeatureMatches {
        FeatureMatches {
            match_map,
            cam_id,
            view_matrix
        }
    }
    pub fn get_match_map(&self) -> &HashMap<usize,Vector2<usize>> {&self.match_map}
    pub fn get_cam_id(&self) -> usize {self.cam_id}
    pub fn get_view_matrix(&self) ->  Matrix3x4<f32> {self.view_matrix}
}