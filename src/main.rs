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
    random_float, random_float_range,
    vec3::{Point3, Vec3},
};

fn main() {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Colour::new(0.5, 0.5, 0.5));
    let ground_sphere = Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(ground_material),
    );
    world.add(Box::new(ground_sphere));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_float();
            let center = Point3::new(
                a as f32 + 0.9 * random_float(),
                0.2,
                b as f32 + 0.9 * random_float(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo: Colour = Vec3::random() * Vec3::random();
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Lambertian::new(albedo)),
                    )));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = random_float_range(0.0, 0.5);

                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Metal::new(albedo, fuzz)),
                    )));
                } else {
                    // Glass
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Dielectric::new(1.5)),
                    )));
                };
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Point3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&mut world)
}
