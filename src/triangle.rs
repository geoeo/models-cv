extern crate nalgebra as na;

use na::{SVector,SVectorView};

// Assuming Counter-Clockwise winding order
pub struct Triangle<const D: usize> {
    v0: SVector::<f32,D>,
    v1: SVector::<f32,D>,
    v2: SVector::<f32,D>
}

impl<const D: usize> Triangle<D> {
    pub fn from_vec(v0: &SVector::<f32,D>, v1: &SVector::<f32,D>, v2: &SVector::<f32,D>) -> Triangle<D> {
        Triangle::<D> {v0: v0.clone(), v1: v1.clone(), v2: v2.clone()}
    }

    pub fn from_view(v0: &SVectorView::<f32,D>, v1: &SVectorView::<f32,D>, v2: &SVectorView::<f32,D>) -> Triangle<D> {
        Triangle::<D> {v0: v0.into_owned(), v1: v1.into_owned(), v2: v2.into_owned()}
    }

    /**
     * Returns a bound box with (min, max) coordinates in the triangle's coordiante system
     */
    pub fn calculate_boudning_box(&self) -> (SVector<f32,D>, SVector<f32,D>) {
        let mut min_arr = [0f32;D];
        let mut max_arr = [0f32;D];

        for i in 0..D {
            let min = self.v0[i].min(self.v1[i].min(self.v2[i]));
            let max = self.v0[i].max(self.v1[i].max(self.v2[i]));
            min_arr[i] = min;
            max_arr[i] = max;
        }

        (SVector::<f32,D>::from(min_arr), SVector::<f32,D>::from(max_arr))
    }

    pub fn get_v0(&self) -> SVector::<f32,D> {self.v0}
    pub fn get_v1(&self) -> SVector::<f32,D> {self.v1}
    pub fn get_v2(&self) -> SVector::<f32,D> {self.v2}
}


