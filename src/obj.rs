extern crate nalgebra as na;

use na::Vector3;

pub fn load(path: &str) -> obj::Obj {
    obj::Obj::load(path).unwrap()
}

pub fn load_vertex_positions(models: &obj::Obj) -> Vec<Vec<Vector3<f32>>> {
    models.data.objects.iter().map(|o| {
        o.groups.iter().map(|g| {
            g.polys.iter().map(|p| {
                let i1 = p.0[0].0;
                let i2 = p.0[1].0;
                let i3 = p.0[2].0;

                let p0 = models.data.position[i1];
                let p1 = models.data.position[i2];
                let p2 = models.data.position[i3];

                vec!(
                    Vector3::new(p0[0],p0[1],p0[2]),
                    Vector3::new(p1[0],p1[1],p1[2]),
                    Vector3::new(p2[0],p2[1],p2[2]))
            }).flatten().collect::<Vec<_>>()
        }).flatten().collect::<Vec<_>>()
    }).collect::<Vec<_>>()

}