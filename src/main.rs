mod hittable;
mod scene;
mod utils;

use std::rc::Rc;

use hittable::{
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    {sphere::Sphere, HitRecord, Hittable, HittableList},
};
use scene::{camera::Camera, ray::Ray};

use utils::{
    colour::{write_colour, Colour},
    vec3::{Point3, Vec3},
};

fn main() {
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.0)));
    let material_sphere_center = Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.5)));
    let material_sphere_left = Rc::new(Dielectric::new(1.5));
    let material_sphere_right = Rc::new(Metal::new(Colour::new(0.8, 0.6, 0.2), 0.0));
    let material_sphere_hollow = Rc::new(Dielectric::new(1.5));

    let sphere_ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground);
    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_sphere_center);
    let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_sphere_left);
    let sphere_right = Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.4, material_sphere_right);
    let sphere_hollow = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_sphere_hollow);

    world.add(Box::new(sphere_ground));
    world.add(Box::new(sphere_center));
    world.add(Box::new(sphere_left));
    world.add(Box::new(sphere_right));
    world.add(Box::new(sphere_hollow));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    cam.lookat = Point3::new(0.0, 0.0, -1.0);
    cam.vup = Point3::new(0.0, 1.0, 0.0);
    cam.vfov = 20.0;
    cam.defocus_angle = 10.0;
    cam.focus_dist = 3.4;

    cam.render(&mut world)
}
