use rand::Rng;

use crate::vec3::Vec3;

pub fn reflect(v: Vec3<f32>, n: Vec3<f32>) -> Vec3<f32> {
    v - n * Vec3::dot(&v, &n) * 2.0
}

pub fn refract(v: Vec3<f32>, n: Vec3<f32>, ni_over_nt: f32) -> Option<Vec3<f32>> {
    let uv = Vec3::unit_vector(v);
    let dt = Vec3::dot(&uv, &n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt*dt);
    if discriminant > 0.0 {
        Some((uv - n*dt) * ni_over_nt - n * discriminant.sqrt())
    } else {
        None
    }
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0*r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
