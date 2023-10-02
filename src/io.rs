extern crate nalgebra as na;

use crate::camera_features::CameraFeatures;
use crate::landmark::Landmark;
use std::fs;
use na::Vector2;

pub fn serialize_feature_matches(path_str: &str, camera_feature_vec: &Vec<CameraFeatures>) -> std::io::Result<()> {
    let serial_state = CameraFeatures::to_serial(camera_feature_vec);
    let serde_yaml = serde_yaml::to_string(&serial_state);
    fs::write(path_str,serde_yaml.unwrap())
}

pub fn deserialize_feature_matches(path_str: &str) -> Vec<CameraFeatures> {
    let serde_yaml = fs::read_to_string(path_str).expect("Unable to read file!");
    let serial_state = serde_yaml::from_str(&serde_yaml).expect("Unable to parse YAML");
    CameraFeatures::from_serial(&serial_state)
}

pub fn serialize_landmarks(path_str: &str, landmark_vec: &Vec<Landmark>) -> std::io::Result<()> {
    let serial_state = Landmark::to_serial(landmark_vec);
    let serde_yaml = serde_yaml::to_string(&serial_state);
    fs::write(path_str,serde_yaml.unwrap())
}

pub fn deserialize_landmarks(path_str: &str) -> Vec<Landmark> {
    let serde_yaml = fs::read_to_string(path_str).expect("Unable to read file!");
    let serial_state = serde_yaml::from_str(&serde_yaml).expect("Unable to parse YAML");
    Landmark::from_serial(&serial_state)
}

pub fn calculate_rgb_byte_vec(screen_points: &Vec<Vector2<usize>>, screen_width: usize, screen_height: usize) -> Vec<u8> {
    let mut dat_vec: Vec<u8> = vec![0;3*screen_width*screen_height];

    let screen_points_in_range = screen_points.iter().filter(|p| p.x < screen_width && p.y < screen_height);

    for screen_point in screen_points_in_range {
        let x = screen_point.x;
        let y = screen_point.y;
        let screen_idx = y*screen_width + x;
        let byte_idx = 3*screen_idx;
        dat_vec[byte_idx] = 255;
        dat_vec[byte_idx+1] = 255;
        dat_vec[byte_idx+2] = 255;
    }

    dat_vec
}