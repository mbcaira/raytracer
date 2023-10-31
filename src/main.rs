use std::fs::File;
use std::io::Write;

mod colour;
mod hittable;
mod ray;
mod vec3;

use colour::{write_colour, Colour};
use ray::Ray;
use vec3::{Point3, Vec3};

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;

fn hit_sphere(center: &Point3, radius: f32, r: &Ray) -> f32 {
    let oc = *r.origin() - *center;
    let a = r.direction().length() * r.direction().length();
    let half_b = oc.dot(r.direction());
    let c = oc.length() * oc.length() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_colour(r: &Ray) -> Colour {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let n = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        Colour::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0).scale(0.5)
    } else {
        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Colour::new(1.0, 1.0, 1.0).scale(1.0 - a) + Colour::new(0.5, 0.7, 1.0).scale(a)
    }
}

fn main() {
    let image_height = usize::max((IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize, 1);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (IMAGE_WIDTH as f32 / image_height as f32);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Horizontal and vertical viewport edge vectors
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Pixel distance vectors
    let pixel_delta_u = viewport_u.scale(1.0 / IMAGE_WIDTH as f32);
    let pixel_delta_v = viewport_v.scale(1.0 / image_height as f32);

    // Top left pixel/starting point
    let viewport_upper_left = camera_center
        - Vec3::new(0.0, 0.0, focal_length)
        - viewport_u.scale(0.5)
        - viewport_v.scale(0.5);

    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v).scale(0.5);

    let mut file = File::create("./out.ppm").unwrap();
    file.write_all(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, image_height).as_bytes())
        .unwrap();

    for j in 0..image_height {
        for i in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + pixel_delta_u.scale(i as f32) + pixel_delta_v.scale(j as f32);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_colour = ray_colour(&r);
            write_colour(&mut file, pixel_colour);
        }
    }
}
