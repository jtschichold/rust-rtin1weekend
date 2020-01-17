use std::ops::MulAssign;
use num_traits::Float;

use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::vec3::Vec3;
use crate::material::{Material, ScatterResult};

use super::utils::reflect;

pub struct Metal<T: Float+MulAssign> {
    pub albedo: Vec3<T>,
    pub fuzz: T,
}

impl Material<f32> for Metal<f32> {
    fn scatter(&self, r: &Ray<f32>, hr: &HitRecord<f32>) -> Option<ScatterResult<f32>> {
        let f = if self.fuzz < 1.0 {
            self.fuzz
        } else {
            1.0
        };

        let reflected = reflect(Vec3::unit_vector(r.direction), hr.normal);
        let scattered = Ray {
            origin: hr.p,
            direction: reflected + Vec3::random_in_unit_sphere() * f,
        };
        let attenuation = self.albedo;

        if Vec3::dot(&scattered.direction, &hr.normal) <= 0.0 {
            return None;
        }

        Some(ScatterResult { attenuation, scattered })
    }
}
