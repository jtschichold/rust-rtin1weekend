use std::ops::MulAssign;
use num_traits::Float;
use rand::Rng;

use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::vec3::Vec3;
use crate::material::{Material, ScatterResult};

use super::utils::{reflect, refract, schlick};

pub struct Dielectric<T: Float+MulAssign> {
    pub ref_idx: T,
}

impl Material<f32> for Dielectric<f32> {
    fn scatter(&self, r: &Ray<f32>, hr: &HitRecord<f32>) -> Option<ScatterResult<f32>> {
        let reflected = reflect(r.direction, hr.normal);

        let attenuation = Vec3::new(Some([1.0, 1.0, 1.0]));

        let mut outward_normal = hr.normal;
        let mut ni_over_nt = 1.0 / self.ref_idx;
        let mut cosine = -Vec3::dot(&r.direction, &hr.normal) / r.direction.length();
        if Vec3::dot(&r.direction, &hr.normal) > 0.0 {
            outward_normal = -hr.normal;
            ni_over_nt = self.ref_idx;
            cosine *= -self.ref_idx;
        }

        let mut reflect_prob = 1.0;
        let mut refracted: Vec3<f32> = Vec3::new(None);
        if let Some(r) = refract(r.direction, outward_normal, ni_over_nt) {
            reflect_prob = schlick(cosine, self.ref_idx);
            refracted = r;
        }

        let mut rng = rand::thread_rng();

        if rng.gen::<f32>() < reflect_prob {
            Some(ScatterResult {
                attenuation,
                scattered: Ray {
                    origin: hr.p,
                    direction: reflected,
                },
            })
        } else {
            Some(ScatterResult {
                attenuation,
                scattered: Ray {
                    origin: hr.p,
                    direction: refracted,
                },
            })
        }
    }
}
