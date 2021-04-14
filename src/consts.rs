//! Definition of constants.
pub const INIFINTY: f32 = f32::INFINITY;
pub const PI: f32 = std::f32::consts::PI;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * PI / 180.0
}
