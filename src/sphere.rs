// we need this for consistency with
#![allow(clippy::many_single_char_names)]

use std::ops::MulAssign;
use num_traits::Float;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::{HitRecord, Hitable, HitResult};
use crate::material::Material;

pub struct Sphere<T: Float+MulAssign>  {
    pub center: Vec3<T>,
    pub radius: T,
    pub material: Box<dyn Material<T>>,
}

impl<T: Float+MulAssign> Hitable<T> for Sphere<T> {
    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T) -> Option<HitResult<T>> {
        let oc = r.origin - self.center;
        let a = Vec3::dot(&r.direction, &r.direction);
        let b = Vec3::dot(&oc, &r.direction);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b*b - a*c;
    
        if discriminant > T::zero() {
            let temp = (- b - discriminant.sqrt())/a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitResult {
                    rec: HitRecord {
                        t: temp,
                        p,
                        normal: (p - self.center) / self.radius,
                    },
                    material: self.material.as_ref(),
                });
            }

            let temp = (- b + discriminant.sqrt())/a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitResult {
                    rec: HitRecord {
                        t: temp,
                        p,
                        normal: (p - self.center) / self.radius,
                    },
                    material: self.material.as_ref(),
                });
            }
        }

        None
    }
}
