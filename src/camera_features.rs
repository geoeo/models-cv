extern crate nalgebra as na;

use na::{Vector2,Matrix3x4,Matrix3};
use std::collections::HashMap;

#[derive(Debug,PartialEq)]
pub struct CameraFeatures {
    // The first entry of the tuple is the global point id
    feature_map: HashMap<usize,Vector2<usize>>,
    cam_id: usize,
    view_matrix: Matrix3x4<f32>,
    intrinsic_matrix: Matrix3<f32>
}

impl CameraFeatures {
    pub fn new(match_map: HashMap<usize,Vector2<usize>>, cam_id: usize, view_matrix: Matrix3x4<f32>,intrinsic_matrix: Matrix3<f32>) -> CameraFeatures {
        CameraFeatures {
            feature_map: match_map,
            cam_id,
            view_matrix,
            intrinsic_matrix
        }
    }
    pub fn get_feature_map(&self) -> &HashMap<usize,Vector2<usize>> {&self.feature_map}
    pub fn get_cam_id(&self) -> usize {self.cam_id}
    pub fn get_view_matrix(&self) ->  Matrix3x4<f32> {self.view_matrix}
    pub fn get_intrinsic_matrix(&self) ->  Matrix3<f32> {self.intrinsic_matrix}
    pub fn to_serial(fm_vec: &Vec<CameraFeatures>) -> Vec<(usize, [f32;12],[f32;9], Vec<(usize,(usize,usize))>)> {
        fm_vec.into_iter().map(|fm|{
            let mut map_vec =  Vec::<(usize,(usize,usize))>::with_capacity(fm.feature_map.len());

            for (k,v) in &fm.feature_map {
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
                [
                fm.intrinsic_matrix[(0,0)],
                fm.intrinsic_matrix[(0,1)],
                fm.intrinsic_matrix[(0,2)],
                fm.intrinsic_matrix[(1,0)],
                fm.intrinsic_matrix[(1,1)],
                fm.intrinsic_matrix[(1,2)],
                fm.intrinsic_matrix[(2,0)],
                fm.intrinsic_matrix[(2,1)],
                fm.intrinsic_matrix[(2,2)]
                ],
                map_vec
            )
        }).collect::<Vec<_>>()

    }

    pub fn from_serial(serial: &Vec<(usize, [f32;12], [f32;9], Vec<(usize,(usize,usize))>)>) -> Vec<CameraFeatures> {
        serial.into_iter().map(|s| {
            let cam_id = s.0;
            let view_arr = &s.1;
            let intrinsic_arr = &s.2;
            let vec = &s.3;

            let view_matrix = Matrix3x4::<f32>::new(
                view_arr[0],view_arr[1],view_arr[2],view_arr[3],
                view_arr[4],view_arr[5],view_arr[6],view_arr[7],
                view_arr[8],view_arr[9],view_arr[10],view_arr[11],
            );
            let intrinsic_matrix = Matrix3::<f32>::new(
                intrinsic_arr[0],intrinsic_arr[1],intrinsic_arr[2],
                intrinsic_arr[3],intrinsic_arr[4],intrinsic_arr[5],
                intrinsic_arr[6],intrinsic_arr[7],intrinsic_arr[8],
            );

            let mut feature_map = HashMap::<usize,Vector2<usize>>::with_capacity(vec.len());
            for &(point_id,(x,y)) in vec {
                feature_map.insert(point_id, Vector2::new(x,y));
            }
    
            CameraFeatures {
                feature_map,
                cam_id,
                view_matrix,
                intrinsic_matrix
            }
    
        }).collect::<Vec<_>>()

    }
}


