//! 3D hittable [`Sphere`]s.
use crate::hittable::{HitRecord, Hittable};
use crate::ray::*;
use crate::vec::*;

#[derive(Debug, Default)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let half_b = oc.dot(ray.direction);
        let c = oc.squared_length() - self.radius * self.radius;

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
        let record = HitRecord::new(p, t, outward_normal);

        return Some(record);
    }
}

/// Checks if a ray hit a sphere.
pub fn hit_sphere(center: Point, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let half_b = oc.dot(ray.direction);
    let c = oc.squared_length() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

// /// Checks if a ray hit a sphere.
// pub fn hit_sphere(center: Point, radius: f32, ray: &Ray) -> f32 {
//     let oc = ray.origin - center;
//     let a = ray.direction.dot(ray.direction);
//     let b = 2.0 * oc.dot(ray.direction);
//     let c = oc.dot(oc) - radius * radius;
//     let discriminant = b * b - 4.0 * a * c;
//     if discriminant < 0.0 {
//         return -1.0;
//     } else {
//         return (-b - discriminant.sqrt()) / (2.0 * a);
//     }
// }
