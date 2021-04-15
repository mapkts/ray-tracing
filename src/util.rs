//! Utility functions.
use rand::prelude::*;

/// Generates a random number within range [0, 1).
pub fn random_f64() -> f64 {
    rand::thread_rng().gen()
}

/// Generates a random number within range [min, max).
pub fn random_f64_within(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

/// Converts degrees to radians.
pub fn degrees_to_radians(degree: f64) -> f64 {
    degree * std::f64::consts::PI / 180.0
}
