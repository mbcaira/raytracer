use std::fs::File;
use std::io::Write;

use crate::Vec3;

use super::interval::Interval;

pub type Colour = Vec3;

fn linear_to_gamma(linear_component: f32) -> f32 {
    linear_component.sqrt()
}
pub fn write_colour(file: &mut File, pixel_colour: Colour, samples_per_pixel: usize) {
    let scaled_pixel_colour = pixel_colour.scale(1.0 / samples_per_pixel as f32);
    let r = linear_to_gamma(scaled_pixel_colour.x());
    let g = linear_to_gamma(scaled_pixel_colour.y());
    let b = linear_to_gamma(scaled_pixel_colour.z());

    let intensity = Interval::new(0.000, 0.999);

    let ir = (256.0 * intensity.clamp(r)) as i32;
    let ig = (256.0 * intensity.clamp(g)) as i32;
    let ib = (256.0 * intensity.clamp(b)) as i32;
    file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())
        .unwrap();
}
