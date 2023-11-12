use std::fs::File;
use std::io::Write;

use crate::Vec3;

use super::interval::Interval;

pub type Colour = Vec3;

pub fn write_colour(file: &mut File, pixel_colour: Colour, samples_per_pixel: usize) {
    let t = 1.0 / samples_per_pixel as f32;
    let scaled_pixel_colour = pixel_colour.scale(1.0 / samples_per_pixel as f32);
    let r = scaled_pixel_colour.x();
    let g = scaled_pixel_colour.y();
    let b = scaled_pixel_colour.z();

    let intensity = Interval::new(0.000, 0.999);

    let ir = (256.0 * intensity.clamp(r)) as i32;
    let ig = (256.0 * intensity.clamp(g)) as i32;
    let ib = (256.0 * intensity.clamp(b)) as i32;
    file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())
        .unwrap();
}
