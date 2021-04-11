use crate::ray::*;
use crate::vector::*;

/// Checks if a ray hit a sphere.
pub fn hit_sphere(center: Point, radius: f32, ray: &Ray) -> bool {
    // tt * bb + 2tb(A - C) + (A - C)(A - C) - rr = 0
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    return discriminant > 0.0;
}
