mod hittable;
mod scene;
mod utils;

use hittable::{sphere::Sphere, HitRecord, Hittable, HittableList};
use scene::camera::Camera;
use scene::ray::Ray;
use utils::colour::{write_colour, Colour};
use utils::vec3::{Point3, Vec3};

fn main() {
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;

    cam.render(&mut world)
}
