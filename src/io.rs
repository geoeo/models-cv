use crate::camera_features::CameraFeatures;
use std::fs;

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