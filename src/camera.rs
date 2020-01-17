use std::ops::{MulAssign, Neg};
use num_traits::Float;

use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera<T: Float+MulAssign+Neg> {
    pub origin: Vec3<T>,
    pub lower_left_corner: Vec3<T>,
    pub horizontal: Vec3<T>,
    pub vertical: Vec3<T>,
    pub lens_radius: T,
    pub u: Vec3<T>,
    pub v: Vec3<T>,
    pub w: Vec3<T>,
}

impl<T: Float+MulAssign+Neg> Camera<T> {
    pub fn new(lookfrom: Vec3<T>, lookat: Vec3<T>, vup: Vec3<T>, vfov: T, aspect: T, aperture: T, focus_dist: T) -> Camera<T> {
        let one = T::one();
        let two = one + one;

        let lens_radius = aperture / two;
        
        let half_height = T::tan(vfov / two) * focus_dist;
        let half_width = aspect * half_height;

        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);

        Camera {
            lower_left_corner: lookfrom - u*half_width - v*half_height - w*focus_dist,
            horizontal: u*two*half_width,
            vertical: v*two*half_height,
            origin: lookfrom,
            lens_radius,
            u, v, w,
        }
    }

    pub fn get_ray(&self, s: T, t: T) -> Ray<T> {
        let rd = Vec3::random_in_unit_sphere() * self.lens_radius;
        let offset = self.u * rd.get_x() + self.v * rd.get_y();
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        }
    }
}
