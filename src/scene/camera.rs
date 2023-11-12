use std::fs::File;
use std::io::Write;

use crate::{
    utils::{interval::Interval, random_float},
    write_colour, Colour, HitRecord, Hittable, Point3, Ray, Vec3,
};

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: usize,
    pub samples_per_pixel: usize,
    image_height: usize,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    fn ray_colour(r: &Ray, world: &mut dyn Hittable) -> Colour {
        let mut rec = HitRecord::default();
        if world.hit(r, Interval::new(0.0, f32::INFINITY), &mut rec) {
            return (rec.normal + Colour::new(1.0, 1.0, 1.0)).scale(0.5);
        }
        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Colour::new(1.0, 1.0, 1.0).scale(1.0 - a) + Colour::new(0.5, 0.7, 1.0).scale(a)
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let pixel_center = self.pixel00_loc
            + self.pixel_delta_u.scale(i as f32)
            + self.pixel_delta_v.scale(j as f32);

        let pixel_sample = pixel_center + self.pixel_sample_square();
        let ray_direction = pixel_sample - self.center;

        Ray::new(self.center, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_float(0.0, 1.0);
        let py = -0.5 + random_float(0.0, 1.0);

        self.pixel_delta_u.scale(px) + self.pixel_delta_v.scale(py)
    }

    fn initialize(&mut self) {
        self.image_height = usize::max((self.image_width as f32 / self.aspect_ratio) as usize, 1);
        self.center = Point3::new(0.0, 0.0, 0.0);

        // Viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        // Horizontal and vertical (down) viewport edge vectors
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Pixel delta vectors, horizontal and vertical
        self.pixel_delta_u = viewport_u.scale(1.0 / self.image_width as f32);
        self.pixel_delta_v = viewport_v.scale(1.0 / self.image_height as f32);

        // Location of upper left/starting pixel
        let viewport_upper_left = self.center
            - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u.scale(0.5)
            - viewport_v.scale(0.5);

        self.pixel00_loc =
            viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v).scale(0.5);
    }

    pub fn render(&mut self, world: &mut dyn Hittable) {
        self.initialize();

        let mut file = File::create("./out.ppm").unwrap();
        file.write_all(format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes())
            .unwrap();

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_colour = pixel_colour + Camera::ray_colour(&r, world);
                }
                write_colour(&mut file, pixel_colour, self.samples_per_pixel)
            }
        }
    }
}
