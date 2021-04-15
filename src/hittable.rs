//! [`Hittable`]s and [`HitRecord`] type.
use crate::prelude::*;

/// A hittable object that a ray can intersect with.
pub trait Hittable {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<HitRecord>;
}

/// A record that contains the information of a hit.
#[derive(Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub t: f64,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, t: f64, normal: Vec3) -> Self {
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
