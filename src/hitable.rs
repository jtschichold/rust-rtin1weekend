use std::ops::MulAssign;
use num_traits::Float;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

pub struct HitRecord<T: Float+MulAssign> {
    pub t: T,
    pub p: Vec3<T>,
    pub normal: Vec3<T>,
}

pub struct HitResult<'a, T: Float+MulAssign> {
    pub rec: HitRecord<T>,
    pub material: &'a Box<dyn Material<T>>,
}

pub trait Hitable<T: Float+MulAssign> {
    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T) -> Option<HitResult<T>> {
        None
    }
}
