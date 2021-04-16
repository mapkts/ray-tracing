//! 3D hittable [`Sphere`]s.
use crate::hittable::{HitRecord, Hittable};
use crate::prelude::*;

#[derive(Debug, Default)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Option<Box<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: impl Material + 'static) -> Self {
        Sphere {
            center,
            radius,
            material: Some(Box::new(material)),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let half_b = oc.dot(ray.direction);
        let c = oc.len_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let t = root;
        let p = ray.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let mut record = HitRecord::new(p, t, self.material.as_ref().map(Box::as_ref));
        record.set_face_normal(ray, outward_normal);

        return Some(record);
    }
}

/// Checks if a ray hit a sphere.
pub fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let half_b = oc.dot(ray.direction);
    let c = oc.len_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}
