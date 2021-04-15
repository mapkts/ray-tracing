//! Definition of constants.
pub const INIFINTY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0
}
