use rand::prelude::*;

/// Generates a random number in range [0, 1)
pub fn random_f32() -> f32 {
    rand::thread_rng().gen()
}

/// Generates a random number in the given range [min, max)
pub fn random_f32_within(min: f32, max: f32) -> f32 {
    rand::thread_rng().gen_range(min..max)
}

/// Converts degrees to radians
pub fn degrees_to_radians(degree: f32) -> f32 {
    degree * std::f32::consts::PI / 180.0
}
