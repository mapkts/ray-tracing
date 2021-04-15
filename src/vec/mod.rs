//! 3D Vector types, such as [`Vec3`] and [`Point3`].
#[macro_use]
pub mod raw;

use self::raw::*;
use crate::util::*;
use std::fmt;
use std::ops::*;

/// A 3D generic vector type.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec3d<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// RGB color data.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Color<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl_vec_common!(named_struct@3, Vec3d(x, y, z), (x, y, z));
impl_vec_specific!(@3, Vec3d);

impl_vec_common!(named_struct@3, Color(r, g, b), (r, g, b));
impl_vec_specific!(@3, Color);

impl<T> fmt::Display for Vec3d<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl<T> fmt::Display for Color<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

/// Represents vectors in the 3D spatial space.
pub type Vec3 = Vec3d<f64>;

/// Represents points in the 3D spatial space.
pub type Point3 = Vec3d<f64>;

impl Vec3 {
    #[inline]
    pub fn random(rng: &mut impl rand::Rng) -> Self {
        Vec3::new(rng.gen(), rng.gen(), rng.gen())
    }

    #[inline]
    pub fn random_within(rng: &mut impl rand::Rng, range: Range<f64>) -> Self {
        Vec3::new(
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
            rng.gen_range(range),
        )
    }

    pub fn random_in_unit_sphere(rng: &mut impl rand::Rng) -> Self {
        // Reject all points that are outside the unit sphere until catch a point inside the unit 
        // sphere.
        loop {
            let p = Self::random_within(rng, -1.0..1.0);
            if p.len_squared() < 1.0 {
                return p
            }
        }
    }

    pub fn random_in_unit_hemisphere(rng: &mut impl rand::Rng, normal: Vec3) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere(rng);
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_unit_vector() -> Self {
        // TODO: Cache thread rng intead of calling this every time.
        let mut rng = rand::thread_rng();
        Self::random_in_unit_sphere(&mut rng).normal()
    }
}
