//! 3D Vector types, such as [`Vec3`], [`Point`] and [`Color`].
#![allow(non_snake_case)]
use crate::error::{ErrorKind, Result};
use std::fmt;
use std::io;
use std::ops::*;

/// Types that can be served as scalar values in vector space.
pub trait Scalar:
    Sized
    + Copy
    + AddAssign
    + SubAssign
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
{
}

macro_rules! scalar_impls {
    ($($type: ty)+) => {
        $(
            impl Scalar for $type {}
        )+
    }
}

scalar_impls!( f32 f64 i8 i16 i32 i64 i128 isize );

/// 3-dimensional vector types.
pub trait Vector3d<T: Scalar>: From<(T, T, T)> + Into<(T, T, T)> + Copy {
    fn length(self) -> T;

    fn squared_length(self) -> T {
        let (x, y, z) = self.into();
        x * x + y * y + z * z
    }

    fn normalize(self) -> Self {
        let len = self.length();
        let (x, y, z) = self.into();
        (x / len, y / len, z / len).into()
    }

    fn dot(self, rhs: Self) -> T {
        let ((x, y, z), (u, v, w)) = (self.into(), rhs.into());
        x * u + y * v + z * w
    }

    fn cross(self, rhs: Self) -> Self {
        let ((x, y, z), (u, v, w)) = (self.into(), rhs.into());
        (y * w - z * v, z * u - x * w, x * v - y * u).into()
    }
}

// Implements binop traits on vector types.
//
// It's best we can implement binop traits directly on `Vector3d` such as:
//
// impl<T: Vector3d> Add for T { ... }
//
// But unfortunately, due to orphan rules, Rust now forbids implementing a foreign trait on
// non-local types.
//
// Orphan rules: https://internals.rust-lang.org/t/revisit-orphan-rules/7795
macro_rules! binop_impls {
    // impl Op<Vector>
    (
        op_vector, $vector:ident, $scalar:ident,
        $($trait:ident, $fn:ident, $op:tt),+
    ) => {
        $(impl $trait<Self> for $vector where $vector: Vector3d<$scalar> {
            type Output = $vector;

            fn $fn(self, rhs: Self) -> Self::Output {
                let ((x, y, z), (u, v, w)) :
                    (($scalar, $scalar, $scalar), ($scalar, $scalar, $scalar))
                    = (self.into(), rhs.into());
                (x $op u, y $op v, z $op w).into()
            }
        })+
    };

    // impl Op<Scalar>
    (
        op_scalar, $vector:ident, $scalar:ident,
        $($trait:ident, $fn:ident, $op:tt),+
    ) => {
        $(impl $trait<$scalar> for $vector where $vector: Vector3d<$scalar> {
            type Output = $vector;

            fn $fn(self, rhs: $scalar) -> Self::Output {
                let (x, y, z): ($scalar, $scalar, $scalar) = self.into();
                (x $op rhs, y $op rhs, z $op rhs).into()
            }
        })+

        $(impl $trait<$vector> for $scalar where $vector: Vector3d<$scalar> {
            type Output = $vector;

            fn $fn(self, rhs: $vector) -> Self::Output {
                let (x, y, z): ($scalar, $scalar, $scalar) = rhs.into();
                (x $op self, y $op self, z $op self).into()
            }
        })+
    };

    // impl OpAssgin<Vector>
    (
     op_assgin_vector, $vector:ident, $scalar:ident,
     $($trait:ident, $fn:ident, $op:tt),+
    ) => {
        $(impl $trait<Self> for $vector where $vector: Vector3d<$scalar> {
            fn $fn(&mut self, rhs: Self) {
                let ((mut x, mut y, mut z), (u, v, w)) :
                    (($scalar, $scalar, $scalar), ($scalar, $scalar, $scalar))
                    = ((*self).into(), rhs.into());
                x $op u;
                y $op v;
                z $op w;
                let _ = std::mem::replace(self, (x, y, z).into());
            }
        })+
    };

    // impl OpAssgin<Scalar>
    (
     op_assgin_scalar, $vector:ident, $scalar:ident,
     $($trait:ident, $fn:ident, $op:tt),+
    ) => {
        $(impl $trait<$scalar> for $vector where $vector: Vector3d<$scalar> {
            fn $fn(&mut self, rhs: $scalar) {
                let (mut x, mut y, mut z) : ($scalar, $scalar, $scalar) = (*self).into();
                x $op rhs;
                y $op rhs;
                z $op rhs;
                let _ = std::mem::replace(self, (x, y, z).into());
            }
        })+
    };
}

/// Implements meaningful binary operator traits for vector type.
///
/// - 1st arg should be a 3D vector type.
/// - 2nd arg should be the type of the vector's elements.
///
/// ```ignore
/// struct Vec3(f32, f32, f32);
///
/// // `$vector3D` type must implement `Vector3d` trait beforehead.
/// vector_binop_impls!(Vec3, f32);
/// ```
#[macro_export]
macro_rules! vector_binop_impls {
    ($vector:ident, $scalar:ident) => {
        // vector can `add` or `sub` vector
        binop_impls!(
            op_vector, $vector, $scalar,
            Add, add, +,
            Sub, sub, -,
            Mul, mul, *
        );

        binop_impls!(
            op_assgin_vector, $vector, $scalar,
            AddAssign, add_assign, +=,
            SubAssign, sub_assign, -=,
            MulAssign, mul_assign, *=
        );


        // vector can `mul` or `div` scalar
        binop_impls!(
            op_scalar, $vector, $scalar,
            Mul, mul, *,
            Div, div, /
        );

        binop_impls!(
            op_assgin_scalar, $vector, $scalar,
            MulAssign, mul_assign, *=,
            DivAssign, div_assign, /=
        );

        impl Neg for $vector where $vector: Vector3d<$scalar> {
            type Output = Self;

            fn neg(self) -> Self::Output {
                let (x, y, z): ($scalar, $scalar, $scalar) = self.into();
                (-x, -y, -z).into()
            }
        }
    }
}

///////////////////////////////////////////////////////////////
// => Color
///////////////////////////////////////////////////////////////

/// RGB color data.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

impl From<Color> for (f32, f32, f32) {
    fn from(color: Color) -> (f32, f32, f32) {
        (color.r, color.g, color.b)
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from(tuple: (f32, f32, f32)) -> Color {
        Color {
            r: tuple.0,
            g: tuple.1,
            b: tuple.2,
        }
    }
}

impl Vector3d<f32> for Color {
    fn length(self) -> f32 {
        self.squared_length().sqrt()
    }
}
vector_binop_impls!(Color, f32);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b }
    }

    pub fn write<W: io::Write + fmt::Debug>(
        self,
        stream: &mut W,
        samples_per_pixel: i32,
    ) -> Result<()> {
        let (mut r, mut g, mut b) = self.into();

        let scale = 1.0 / samples_per_pixel as f32;
        r = (r * scale).sqrt();
        g = (g * scale).sqrt();
        b = (b * scale).sqrt();

        writeln!(
            stream,
            "{} {} {}",
            ((r * 256.0) as u8).clamp(0, u8::MAX),
            ((g * 256.0) as u8).clamp(0, u8::MAX),
            ((b * 256.0) as u8).clamp(0, u8::MAX),
        )
        .map_err(|_| ErrorKind::WriteColor(format!("{:?}", stream)))
    }
}

///////////////////////////////////////////////////////////////
// => Point and Vec3
///////////////////////////////////////////////////////////////

macro_rules! vector_type {
    ($(#[$doc:meta])* $vector:ident) => {
        $(#[$doc])*
        #[derive(Copy, Clone, Debug, Default, PartialEq)]
        pub struct $vector {
            pub x: f32,
            pub y: f32,
            pub z: f32,
        }

        impl $vector {
            pub fn new(x: f32, y: f32, z: f32) -> Self {
                $vector { x, y, z }
            }
        }

        impl fmt::Display for $vector {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{} {} {}", self.x, self.y, self.z)
            }
        }

        impl From<$vector> for (f32, f32, f32) {
            fn from($vector: $vector) -> (f32, f32, f32) {
                ($vector.x, $vector.y, $vector.z)
            }
        }

        impl From<(f32, f32, f32)> for $vector {
            fn from(tuple: (f32, f32, f32)) -> $vector {
                $vector {
                    x: tuple.0,
                    y: tuple.1,
                    z: tuple.2,
                }
            }
        }

        impl Vector3d<f32> for $vector {
            fn length(self) -> f32 {
                self.squared_length().sqrt()
            }
        }
        vector_binop_impls!($vector, f32);
    };
}

vector_type!(
    /// Vectors in the 3D spatial space.
    Vec3
);

/// Points in the 3D spatial space.
pub type Point = Vec3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_sqaured_length() {
        let p: Point = (2.0, 4.0, 4.0).into();
        assert_eq!(p.squared_length() as i32, 36);
    }

    #[test]
    fn vector_length() {
        let p: Point = (2.0, 4.0, 4.0).into();
        assert_eq!(p.length() as i32, 6);
    }

    #[test]
    fn vector_normalize() {
        let p: Point = (2.0, 4.0, 4.0).into();
        assert_eq!(
            p.normalize(),
            Point {
                x: 2.0 / 6.0,
                y: 4.0 / 6.0,
                z: 4.0 / 6.0
            }
        );
    }

    #[test]
    fn vector_dot() {
        let p1: Point = (2.0, 4.0, 4.0).into();
        let p2: Point = (1.0, 2.0, 3.0).into();
        assert_eq!(p1.dot(p2), 2.0 * 1.0 + 4.0 * 2.0 + 4.0 * 3.0);
    }

    #[test]
    fn vector_cross() {
        // | x1 |   | x2 |   | y1z2 - z1y2 |
        // | y1 | x | y2 | = | z1x2 - x1z2 |
        // | z1 |   | z2 |   | x1y2 - y1x2 |
        let p1: Point = (2.0, 4.0, 4.0).into();
        let p2: Point = (1.0, 2.0, 3.0).into();
        assert_eq!(
            p1.cross(p2),
            Point {
                x: 4.0 * 3.0 - 4.0 * 2.0,
                y: 4.0 * 1.0 - 2.0 * 3.0,
                z: 2.0 * 2.0 - 4.0 * 1.0
            }
        );
    }

    #[test]
    fn vector_add() {
        let p1: Point = (2.0, 4.0, 4.0).into();
        let p2: Point = (1.0, 2.0, 3.0).into();
        assert_eq!(
            p1 + p2,
            Point {
                x: 3.0,
                y: 6.0,
                z: 7.0
            }
        );
    }

    #[test]
    fn vector_add_assgin() {
        let mut p1: Point = (2.0, 4.0, 4.0).into();
        let p2: Point = (1.0, 2.0, 3.0).into();
        p1 += p2;
        assert_eq!(
            p1,
            Point {
                x: 3.0,
                y: 6.0,
                z: 7.0
            }
        );
    }

    #[test]
    fn vector_sub() {
        let p1: Point = (2.0, 4.0, 4.0).into();
        let p2: Point = (1.0, 2.0, 3.0).into();
        assert_eq!(
            p1 - p2,
            Point {
                x: 1.0,
                y: 2.0,
                z: 1.0
            }
        );
    }

    #[test]
    fn vector_sub_assgin() {
        let mut p1: Point = (2.0, 4.0, 4.0).into();
        let p2: Point = (1.0, 2.0, 3.0).into();
        p1 -= p2;
        assert_eq!(
            p1,
            Point {
                x: 1.0,
                y: 2.0,
                z: 1.0
            }
        );
    }

    #[test]
    fn vector_mul() {
        let p1: Point = (2.0, 4.0, 4.0).into();
        assert_eq!(
            p1 * 0.5,
            Point {
                x: 1.0,
                y: 2.0,
                z: 2.0
            }
        );
    }

    #[test]
    fn vector_mul_assgin() {
        let mut p1: Point = (2.0, 4.0, 4.0).into();
        p1 *= 0.5;
        assert_eq!(
            p1,
            Point {
                x: 1.0,
                y: 2.0,
                z: 2.0
            }
        );
    }

    #[test]
    fn vector_div() {
        let p1: Point = (2.0, 4.0, 4.0).into();
        assert_eq!(
            p1 / 2.0,
            Point {
                x: 1.0,
                y: 2.0,
                z: 2.0
            }
        );
    }

    #[test]
    fn vector_div_assgin() {
        let mut p1: Point = (2.0, 4.0, 4.0).into();
        p1 /= 2.0;
        assert_eq!(
            p1,
            Point {
                x: 1.0,
                y: 2.0,
                z: 2.0
            }
        );
    }

    #[test]
    fn vector_neg() {
        let p1: Point = (2.0, 4.0, 4.0).into();
        assert_eq!(
            -p1,
            Point {
                x: -2.0,
                y: -4.0,
                z: -4.0
            }
        );
    }
}
