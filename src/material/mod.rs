mod lambertian;
mod metal;
mod dielectric;

mod utils;

use std::ops::MulAssign;
use num_traits::Float;

use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::vec3::Vec3;

pub use self::lambertian::Lambertian;
pub use self::metal::Metal;
pub use self::dielectric::Dielectric;

pub struct ScatterResult<T: Float+MulAssign> {
    pub attenuation: Vec3<T>,
    pub scattered: Ray<T>,
}

pub trait Material<T: Float+MulAssign> {
    fn scatter(&self, r: &Ray<T>, hr: &HitRecord<T>) -> Option<ScatterResult<T>>;
}
