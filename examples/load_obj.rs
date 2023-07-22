use std::{
    error::Error as StdError,
    boxed::Box
};

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        run(&path).expect("runtime error");
    } else {
        println!("usage: load-obj <FILE>");
    }
}

fn run(path: &str) -> Result<(), Box<dyn StdError>> {
    let (models, _) =
        tobj::load_obj(path, &tobj::LoadOptions::default()).expect("Failed to OBJ load file");
    
    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        println!("");
        println!("model[{}].name             = \'{}\'", i, m.name);
        println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

        for vtx in 0..mesh.positions.len() / 3 {
            println!(
                "position[{}] = ({}, {}, {})",
                vtx,
                mesh.positions[3 * vtx],
                mesh.positions[3 * vtx + 1],
                mesh.positions[3 * vtx + 2]
            );
        }
    }
    
    Ok(())
}