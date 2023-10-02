extern crate nalgebra as na;

use na::Vector3;

#[derive(Debug,PartialEq)]
pub struct Landmark {
    id: usize,
    position: Vector3<f32>
}

impl Landmark {
    pub fn new(id: &usize, position: &Vector3<f32>) -> Landmark {
        Landmark {
            id: id.clone(),
            position: position.clone()
        }
    }
    pub fn get_id(&self) -> &usize {&self.id}
    pub fn get_position(&self) -> &Vector3<f32> {&self.position}
    pub fn to_serial(landmark_vec: &Vec<Landmark>) -> Vec<(usize, [f32;3])> {
        landmark_vec.into_iter().map(|l|{
            let pos = l.get_position();
            (l.id, [pos.x,pos.y,pos.z] )
        }).collect::<Vec<_>>()
    }

    pub fn from_serial(serial: &Vec<(usize, [f32;3])>) -> Vec<Landmark> {
        serial.into_iter().map(|s| {
            let id = s.0;
            let pos = &s.1;

            Landmark {
                id,
                position: Vector3::<f32>::new(pos[0],pos[1],pos[2])
            }
    
        }).collect::<Vec<_>>()

    }
}


