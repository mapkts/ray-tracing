use crate::ray::Ray;
use crate::vector::*;

#[derive(Debug)]
pub struct HitRecord {
    pub p: Point,
    pub t: f32,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new(p: Point, t: f32, normal: Vec3) -> Self {
        HitRecord { p, t, normal }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitRecord>;
}


