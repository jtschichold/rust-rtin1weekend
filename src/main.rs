mod vec3;
mod ray;
mod hitable;
mod hitablelist;
mod sphere;
mod camera;
mod material;

use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hitable::Hitable;
use crate::hitablelist::HitableList;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::material::{Lambertian, Metal, Dielectric};

use rand::Rng;
use std::f32;

fn background_color(r: &Ray<f32>) -> Vec3<f32> {
    let unit_direction = Vec3::unit_vector(r.direction);
    let t = 0.5*(unit_direction.get_y() + 1.0);
    Vec3::new(Some([1.0, 1.0, 1.0]))*(1.0 - t) + Vec3::new(Some([0.5, 0.7, 1.0]))*t   
}

fn color(ray: &Ray<f32>, world: &HitableList<f32>, depth: u32) -> Vec3<f32> {
    // 0.001 to avoid shadow acne
    match &world.hit(ray, 0.001, f32::MAX) {
        Some(t) => {
            if depth >= 50 {
                return Vec3::new(Some([0.0, 0.0, 0.0]));
            }

            match (&t.material).scatter(&ray, &t.rec) {
                Some(sr) => {
                    sr.attenuation * color(&sr.scattered, world, depth+1)
                },
                None => Vec3::new(Some([0.0, 0.0, 0.0]))
            }
        },
        None => background_color(ray)
    }
}

fn random_scene() -> HitableList<f32> {
    let mut result: HitableList<f32> = HitableList {
        list: Vec::new()
    };

    result.list.push(Box::new(Sphere {
        center: Vec3::new(Some([0.0, -1000.0, 0.0])),
        radius: 1000.00,
        material: Box::new(Lambertian { albedo: Vec3::new(Some([0.5, 0.5, 0.5])) }),
    }));

    let mut rng = rand::thread_rng();
    let gcenter = Vec3::new(Some([4.0, 0.0, 2.0]));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Vec3::new(Some([
                (a as f32)+0.9*rng.gen::<f32>(),
                0.2,
                (b as f32)+0.9*rng.gen::<f32>()
            ]));
            if (center - gcenter).length() <= 0.9 {
                continue;
            }
            if choose_mat < 0.8 {
                // diffuse
                result.list.push(Box::new(Sphere {
                    center: center,
                    radius: 0.2,
                    material: Box::new(Lambertian {
                        albedo: Vec3::new(Some([
                            rng.gen::<f32>()*rng.gen::<f32>(),
                            rng.gen::<f32>()*rng.gen::<f32>(),
                            rng.gen::<f32>()*rng.gen::<f32>()
                        ])),
                    }),
                }));
            } else if choose_mat < 0.95 {
                // metal
                result.list.push(Box::new(Sphere {
                    center: center,
                    radius: 0.2,
                    material: Box::new(Metal { 
                        albedo: Vec3::new(Some([0.5*(1.0+rng.gen::<f32>()), 0.5*(1.0+rng.gen::<f32>()), 0.5*(1.0+rng.gen::<f32>())])),
                        fuzz: 0.5*rng.gen::<f32>(),
                    }),
                }));
            } else {
                // glass
                result.list.push(Box::new(Sphere {
                    center: center,
                    radius: 0.2,
                    material: Box::new(Dielectric {
                        ref_idx: 1.52,
                    }),
                }));
            }
        }
    }

    result.list.push(Box::new(Sphere {
        center: Vec3::new(Some([0.0, 1.0, 0.0])),
        radius: 1.00,
        material: Box::new(Dielectric { ref_idx: 1.52 }),
    }));
    result.list.push(Box::new(Sphere {
        center: Vec3::new(Some([-4.0, 1.0, 0.0])),
        radius: 1.00,
        material: Box::new(Lambertian { albedo: Vec3::new(Some([0.4, 0.2, 0.1])) }),
    }));
    result.list.push(Box::new(Sphere {
        center: Vec3::new(Some([4.0, 1.0, 0.0])),
        radius: 1.00,
        material: Box::new(Metal {
            albedo: Vec3::new(Some([0.7, 0.6, 0.5])),
            fuzz: 0.0,
        }),
    }));

    result
}

fn main() {
    let nx = 1200;
    let ny = 800;
    let ns = 50;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let world = random_scene();
    // let mut world: HitableList<f32> = HitableList {
    //     list: Vec::new()
    // };
    // world.list.push(Box::new(Sphere {
    //     center: Vec3::new(Some([0.0, 0.0, -1.0])),
    //     radius: 0.5,
    //     material: Box::new(Lambertian { albedo: Vec3::new(Some([0.8, 0.3, 0.3])) }),
    // }));
    // world.list.push(Box::new(Sphere {
    //     center: Vec3::new(Some([0.0, -100.5, -1.0])),
    //     radius: 100.0,
    //     material: Box::new(Lambertian { albedo: Vec3::new(Some([0.8, 0.8, 0.0])) }),
    // }));
    // world.list.push(Box::new(Sphere {
    //     center: Vec3::new(Some([1.0, 0.0, -1.0])),
    //     radius: 0.5,
    //     material: Box::new(Metal { albedo: Vec3::new(Some([0.8, 0.6, 0.2])), fuzz: 1.0 }),
    // }));
    // world.list.push(Box::new(Sphere {
    //     center: Vec3::new(Some([-1.0, 0.0, -1.0])),
    //     radius: 0.5,
    //     material: Box::new(Dielectric { ref_idx: 2.4 }),
    // }));

    let lookfrom = Vec3::new(Some([13.0, 2.0, 3.0]));
    let lookat = Vec3::new(Some([0.0, 0.0, 0.0]));
    let camera: Camera<f32> = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(Some([0.0, 1.0, 0.0])),
        f32::consts::PI / 9.0,
        (nx as f32)/(ny as f32),
        0.1,
        10.0
    );
    let mut rng = rand::thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(Some([0.0, 0.0, 0.0]));

            for _ in 0..ns {
                let u = (i as f32 + (rng.gen::<f32>())) / (nx as f32);
                let v = (j as f32 + (rng.gen::<f32>())) / (ny as f32);
                let r = camera.get_ray(u, v);
                // let p = r.point_at_parameter(2.0);
                col += color(&r, &world, 0); 
            }
            col /= ns as f32;
            col.gamma2_correct();

            let ir = (255.99 * col[0]) as u8;
            let ig = (255.99 * col[1]) as u8;
            let ib = (255.99 * col[2]) as u8;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
