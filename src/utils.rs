pub mod colour;
pub mod vec3;

use std::f32::consts::PI;

fn degrees_to_rads(degrees: f32) -> f32 {
    return (degrees * PI) / 180.0;
}
