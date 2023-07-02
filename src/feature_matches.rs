extern crate nalgebra as na;

use na::{Vector2,Matrix3x4};
use std::collections::HashMap;

#[derive(Debug,PartialEq)]
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
    pub fn to_serial(fm_vec: &Vec<FeatureMatches>) -> Vec<(usize, [f32;12], Vec<(usize,(usize,usize))>)> {
        fm_vec.into_iter().map(|fm|{
            let mut map_vec =  Vec::<(usize,(usize,usize))>::with_capacity(fm.match_map.len());

            for (k,v) in &fm.match_map {
                map_vec.push((*k,(v.x,v.y)));
            }
    
            (fm.cam_id, [
                fm.view_matrix[(0,0)],
                fm.view_matrix[(0,1)],
                fm.view_matrix[(0,2)],
                fm.view_matrix[(0,3)],
                fm.view_matrix[(1,0)],
                fm.view_matrix[(1,1)],
                fm.view_matrix[(1,2)],
                fm.view_matrix[(1,3)],
                fm.view_matrix[(2,0)],
                fm.view_matrix[(2,1)],
                fm.view_matrix[(2,2)],
                fm.view_matrix[(2,3)],
                ],
                map_vec
            )
        }).collect::<Vec<_>>()

    }

    pub fn from_serial(serial: &Vec<(usize, [f32;12], Vec<(usize,(usize,usize))>)>) -> Vec<FeatureMatches> {
        serial.into_iter().map(|s| {
            let cam_id = s.0;
            let arr = &s.1;
            let view_matrix = Matrix3x4::<f32>::new(
                arr[0],arr[1],arr[2],arr[3],
                arr[4],arr[5],arr[6],arr[7],
                arr[8],arr[9],arr[10],arr[11],
            );
            let vec = &s.2;
            let mut match_map = HashMap::<usize,Vector2<usize>>::with_capacity(vec.len());
            for &(point_id,(x,y)) in vec {
                match_map.insert(point_id, Vector2::new(x,y));
            }
    
            FeatureMatches {
                match_map,
                cam_id,
                view_matrix
            }
    
        }).collect::<Vec<_>>()

    }
}


