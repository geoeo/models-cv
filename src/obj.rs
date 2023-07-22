extern crate nalgebra as na;

use na::Vector3;

pub fn load(path: &str) -> Vec<tobj::Model> {
    let (models, _) = tobj::load_obj(path, &tobj::LoadOptions::default()).expect("Failed to OBJ load file");
    models
}

pub fn load_vertex_positions(models: &Vec<tobj::Model>) -> Vec<Vec<Vector3<f32>>> {
    models.iter().map(|m| {
        let indices = &m.mesh.indices;
        let obj_positions = &m.mesh.positions;
        let number_of_indices = indices.len();
        let number_of_vertices = number_of_indices/3;
        let mut positions = Vec::<Vector3<f32>>::with_capacity(number_of_vertices);
        for i in (0..number_of_indices).step_by(3) {
            let i_x = indices[i] as usize;
            let i_y = indices[i+1] as usize;
            let i_z = indices[i+2] as usize;

            let v_x = obj_positions[i_x];
            let v_y = obj_positions[i_y];
            let v_z = obj_positions[i_z];
            positions.push(Vector3::new(v_x,v_y,v_z));
        }
        positions
    }).collect()
}