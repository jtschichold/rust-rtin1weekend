use std::ops::MulAssign;
use num_traits::Float;

use crate::vec3::Vec3;

pub struct Ray<T: Float+MulAssign> {
    pub origin: Vec3<T>,
    pub direction: Vec3<T>,
}

impl<T: Float+MulAssign> Ray<T> {
    pub fn point_at_parameter(&self, t: T) -> Vec3<T>{
        self.origin + self.direction * t
    }
}