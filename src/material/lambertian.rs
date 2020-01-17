use std::ops::MulAssign;
use num_traits::Float;

use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::vec3::Vec3;
use crate::material::{Material, ScatterResult};

pub struct Lambertian<T: Float+MulAssign> {
    pub albedo: Vec3<T>,
}

impl Material<f32> for Lambertian<f32> {
    fn scatter(&self, r: &Ray<f32>, hr: &HitRecord<f32>) -> Option<ScatterResult<f32>> {
        let target = hr.p + hr.normal + Vec3::random_in_unit_sphere();
        let scattered = Ray {
            origin: hr.p,
            direction: target - hr.p,
        };
        let attenuation = self.albedo;

        Some(ScatterResult { attenuation, scattered })
    }
}