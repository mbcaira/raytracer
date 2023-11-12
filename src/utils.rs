use rand::Rng;

pub mod colour;
pub mod interval;
pub mod vec3;

use std::f32::consts::PI;

fn degrees_to_rads(degrees: f32) -> f32 {
    return (degrees * PI) / 180.0;
}

pub fn random_float(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
