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
pub fn pixel_within_triangle(triangle: &Triangle<2>, p: &Vector2<f32>) -> bool {
    edge_function(&triangle.get_v0(),&triangle.get_v1(),p) >= 0.0 &&
    edge_function(&triangle.get_v1(),&triangle.get_v2(),p) >= 0.0 &&
    edge_function(&triangle.get_v2(),&triangle.get_v0(),p) >= 0.0
}
