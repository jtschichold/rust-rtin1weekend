use std::ops::MulAssign;
use num_traits::Float;

use crate::hitable::{Hitable, HitResult};
use crate::ray::Ray;

pub struct HitableList<T: Float+MulAssign> {
    pub list: Vec<Box<dyn Hitable<T>>>,
}

impl<T: Float+MulAssign> Hitable<T> for HitableList<T> {
    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T) -> Option<HitResult<T>> {
        let mut closest_so_far = t_max;
        let mut hit_record: Option<HitResult<T>> = None;

        for h in &self.list {
            match (*h).hit(r, t_min, closest_so_far) {
                Some(t) => {
                    closest_so_far = t.rec.t;
                    hit_record = Some(t);
                },
                None => (),
            }
        }

        hit_record
    }
}