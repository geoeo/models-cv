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
    pub fn to_serial(&self) -> (usize, [f32;12], Vec<(usize,(usize,usize))>) {
        let mut map_vec =  Vec::<(usize,(usize,usize))>::with_capacity(self.match_map.len());

        for (k,v) in &self.match_map {
            map_vec.push((*k,(v.x,v.y)));
        }

        (self.cam_id, [
                self.view_matrix[(0,0)],
                self.view_matrix[(0,1)],
                self.view_matrix[(0,2)],
                self.view_matrix[(0,3)],
                self.view_matrix[(1,0)],
                self.view_matrix[(1,1)],
                self.view_matrix[(1,2)],
                self.view_matrix[(1,3)],
                self.view_matrix[(2,0)],
                self.view_matrix[(2,1)],
                self.view_matrix[(2,2)],
                self.view_matrix[(2,3)],
            ],
            map_vec
        )
    }

    pub fn from_serial(serial: &(usize, [f32;12], Vec<(usize,(usize,usize))>)) -> FeatureMatches {
        let cam_id = serial.0;
        let arr = &serial.1;
        let view_matrix = Matrix3x4::<f32>::new(
            arr[0],arr[1],arr[2],arr[3],
            arr[4],arr[5],arr[6],arr[7],
            arr[8],arr[9],arr[10],arr[11],
        );
        let vec = &serial.2;
        let mut match_map = HashMap::<usize,Vector2<usize>>::with_capacity(vec.len());
        for &(point_id,(x,y)) in vec {
            match_map.insert(point_id, Vector2::new(x,y));
        }

        FeatureMatches {
            match_map,
            cam_id,
            view_matrix
        }

    }
}


