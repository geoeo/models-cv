extern crate nalgebra as na;

use na::{Vector2, Matrix2};
use crate::triangle::Triangle;

const EPS: f32 = 5e-3;

//https://www.scratchapixel.com/lessons/3d-basic-rendering/rasterization-practical-implementation/rasterization-stage.html

/**
 * Computes the area of the parallelogram spanned by the intrinsic triangle using the determinant
 */
fn edge_function(a: &Vector2<f32>, b: &Vector2<f32>, p: &Vector2<f32>) -> f32 {
    let b_p = p-b;
    let b_a = a-b;
    let mat = Matrix2::<f32>::from_rows(&[b_p.transpose(),b_a.transpose()]);
    mat.determinant()
}

/**
 *  * Assume the triangle vertices a,b are in counter-clockwise winding order -> Fix this. GLTF can be any winding order
 */
pub fn pixel_within_triangle_and_barycentric(triangle: &Triangle<2>, p: &Vector2<f32>) -> (f32,f32,f32,bool) {
    let area = edge_function(&triangle.get_v0(),&triangle.get_v1(),&triangle.get_v2());
    let f = match area > 0.0 {
        true => 1.0,
        false => -1.0
    };

    let w2 = edge_function(&triangle.get_v0(),&triangle.get_v1(),p)*f/area;
    let w0 = edge_function(&triangle.get_v1(),&triangle.get_v2(),p)*f/area;
    let w1 = edge_function(&triangle.get_v2(),&triangle.get_v0(),p)*f/area;

    let inside = w2 >= -EPS && w0 >= -EPS && w1 >= -EPS;

    (w0,w1,w2,inside)
}

/**
 * Returns all tuples (w0, w1, w2, p) consisting of the baryentric coordiantes (w0, w1, w2) of a pixel (p),
 * for all pixels inside the  given triangle.
 */
pub fn calc_all_pixels_within_triangle(triangle: &Triangle<2>) -> Vec<(f32,f32,f32,Vector2<f32>)> {
    let (min, max) = triangle.calculate_boudning_box();
    let min_x_f = min.x.floor() as usize;
    let min_y_f = min.y.floor() as usize;
    let max_x_f = max.x.ceil() as usize;
    let max_y_f = max.y.ceil() as usize;

    let y_range = min_y_f..max_y_f;

    y_range.map(|y| {
        let x_range = min_x_f..max_x_f;
        let y_c = y as f32 + 0.5f32;
        x_range.map(move |x| {
            let x_c = x as f32 + 0.5f32;
            Vector2::new(x_c,y_c)
        })
    }).flatten()
    .map(|p| (pixel_within_triangle_and_barycentric(&triangle,&p),p))
    .filter(|&((_,_,_,inside),_)| inside)
    .map(|((w0,w1,w2,_),p)| (w0,w1,w2,p)).collect()
}

/**
 * Calcualte the inv depth for all pixels inside a triangle using perspective correct interpolation
 */
pub fn calc_inv_z_for_all_pixels(barycentric_pixels: &Vec<(f32,f32,f32)>, triangle3d: &Triangle<3>) -> Vec<f32> {
    barycentric_pixels.iter().map(|(w0,w1,w2)| {
        let inv_z = (w0 / triangle3d.get_v0().z) + (w1 / triangle3d.get_v1().z) + (w2 / triangle3d.get_v2().z);
        inv_z
    }).collect()
}
