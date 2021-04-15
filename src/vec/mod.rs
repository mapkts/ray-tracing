//! 3D Vector types, such as [`Vec3`] and [`Point3`].
#[macro_use]
pub mod raw;

use self::raw::*;
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
