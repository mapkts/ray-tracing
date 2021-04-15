//! Macro definitions.
use crate::prelude::{Point3, Rgb, Vec3};

/// A convenient macro to generate Vec3<f64>.
#[macro_export]
macro_rules! v3 {
    ($x:literal $(,)? $y:literal $(,)? $z:literal $(,)?) => {
        Vec3::new($x as f64, $y as f64, $z as f64)
    };
}

/// A convenient macro to generate Point3<f64>.
#[macro_export]
macro_rules! p3 {
    ($x:literal $(,)? $y:literal $(,)? $z:literal $(,)?) => {
        Point3::new($x as f64, $y as f64, $z as f64)
    };
}

/// A convenient macro to generate Rgb<f64>.
#[macro_export]
macro_rules! rgb {
    ($x:literal $(,)? $y:literal $(,)? $z:literal $(,)?) => {
        Rgb::new($x as f64, $y as f64, $z as f64)
    };
}
