//! The [`Ray`] type. We can't live without it.
use crate::vec::{Point, Vec3};

/// Rays in the 3D space.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Point {
        self.origin + t * self.direction
    }
}
