use std::fs::File;
use std::io::Write;

use crate::Vec3;

pub type Colour = Vec3;

pub fn write_colour(file: &mut File, pixel_colour: Colour) {
    // Scale integer values to be between 0 and 255
    let ir = (255.999 * pixel_colour.x()) as i32;
    let ig = (255.999 * pixel_colour.y()) as i32;
    let ib = (255.999 * pixel_colour.z()) as i32;

    file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())
        .unwrap();
}
