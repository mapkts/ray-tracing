//! The [`Ray`] type. We can't live without it.
use crate::vector::{Point, Vec3};

/// Rays in the 3D space.
#[derive(Debug)]
pub struct Ray {
    pub orig: Point,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point, dir: Vec3) -> Self {
        Ray { orig, dir }
    }

    pub fn at(&self, t: f32) -> Point {
        self.orig + t * self.dir
    }
}
