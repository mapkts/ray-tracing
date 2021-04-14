//! [`Hittable`]s and [`HitRecord`] type.
use crate::ray::Ray;
use crate::vec::*;

/// A hittable object that a ray can intersect with.
pub trait Hittable {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitRecord>;
}

/// A record that contains the information of a hit.
#[derive(Debug)]
pub struct HitRecord {
    pub p: Point,
    pub t: f32,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point, t: f32, normal: Vec3) -> Self {
        HitRecord {
            p,
            t,
            normal,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

/// A list of hittable objects.
pub struct HittableList {
    objects: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: impl Hittable + Sync + Send + 'static) {
        self.objects.push(Box::new(object));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut record = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                record.replace(rec);
            }
        }

        record
    }
}
