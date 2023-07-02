use crate::feature_matches::FeatureMatches;
use std::fs;

pub fn serialize_feature_matches(path_str: &str, feature_matches_vec: &Vec<FeatureMatches>) -> std::io::Result<()> {
    let serial_state = FeatureMatches::to_serial(feature_matches_vec);
    let serde_yaml = serde_yaml::to_string(&serial_state);
    fs::write(path_str,serde_yaml.unwrap())
}

pub fn deserialize_feature_matches(path_str: &str) -> Vec<FeatureMatches> {
    let serde_yaml = fs::read_to_string(path_str).expect("Unable to read file!");
    let serial_state = serde_yaml::from_str(&serde_yaml).expect("Unable to parse YAML");
    FeatureMatches::from_serial(&serial_state)
}