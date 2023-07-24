extern crate nalgebra as na;

use na::{Vector2, Matrix2};
use crate::triangle::Triangle;

//TODO: https://www.scratchapixel.com/lessons/3d-basic-rendering/rasterization-practical-implementation/rasterization-stage.html

/**
 * Computes the area of the parallelogram spanned by the intrinsic triangle using the determinant
 * Assume the triangle vertices a,b are in counter-clockwise winding order
 */
fn edge_function(a: &Vector2<f32>, b: &Vector2<f32>, p: &Vector2<f32>) -> f32 {
    let b_p = p-b;
    let b_a = a-b;
    let mat = Matrix2::<f32>::from_rows(&[b_p.transpose(),b_a.transpose()]);
    mat.determinant()
}

//TODO: test this
pub fn pixel_within_triangle_and_barycentric(triangle: &Triangle<2>, p: &Vector2<f32>) -> (f32,f32,f32,bool) {
    let area = edge_function(&triangle.get_v0(),&triangle.get_v1(),&triangle.get_v2());
    assert!(area > 0.0);

    let w2 = edge_function(&triangle.get_v0(),&triangle.get_v1(),p)/area;
    let w0 = edge_function(&triangle.get_v1(),&triangle.get_v2(),p)/area;
    let w1 = edge_function(&triangle.get_v2(),&triangle.get_v0(),p)/area;

    let inside = w2 >= 0.0 && w0 >= 0.0 && w1 >= 0.0;

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
    let max_x_f = max.x.floor() as usize;
    let max_y_f = max.y.floor() as usize;

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

//TODO: test this
/**
 * Calcualte the depth for all pixels inside a triangle using perspective correct interpolation
 */
pub fn calc_z_for_all_pixels(barycentric_pixels: &Vec<(f32,f32,f32,Triangle<3>)>) -> Vec<f32> {
    barycentric_pixels.iter().map(|(w0,w1,w2,triangle3d)| {
        let inv_z = w0 / triangle3d.get_v0().z + w1 / triangle3d.get_v1().z + w2 / triangle3d.get_v2().z;
        1.0 / inv_z
    }).collect()
}
