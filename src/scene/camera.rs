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
    pub max_depth: usize,
    pub vfov: f32,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f32,
    pub focus_dist: f32,
    image_height: usize,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    fn ray_colour(r: &Ray, max_depth: usize, world: &mut dyn Hittable) -> Colour {
        let mut rec = HitRecord::default();

        if max_depth <= 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        if world.hit(r, Interval::new(0.001, f32::INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Colour::default();

            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * Self::ray_colour(&scattered, max_depth - 1, world);
            }

            return Colour::default();
        }
        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Colour::new(1.0, 1.0, 1.0).scale(1.0 - a) + Colour::new(0.5, 0.7, 1.0).scale(a)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();

        self.center + (self.defocus_disk_u.scale(p.x()) + self.defocus_disk_v.scale(p.y()))
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let pixel_center = self.pixel00_loc
            + self.pixel_delta_u.scale(i as f32)
            + self.pixel_delta_v.scale(j as f32);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_float();
        let py = -0.5 + random_float();

        self.pixel_delta_u.scale(px) + self.pixel_delta_v.scale(py)
    }

    fn initialize(&mut self) {
        self.image_height = usize::max((self.image_width as f32 / self.aspect_ratio) as usize, 1);
        self.center = self.lookfrom;

        // Viewport dimensions
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        // Unit basis vectors for camera coordinate frame
        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.vup.cross(&self.w).unit_vector();
        self.v = self.w.cross(&self.u);

        // Horizontal and vertical (down) viewport edge vectors
        let viewport_u = self.u.scale(viewport_width);
        let viewport_v = self.v.scale(-viewport_height);

        // Pixel delta vectors, horizontal and vertical
        self.pixel_delta_u = viewport_u.scale(1.0 / self.image_width as f32);
        self.pixel_delta_v = viewport_v.scale(1.0 / self.image_height as f32);

        // Location of upper left/starting pixel
        let viewport_upper_left = self.center
            - (self.w.scale(self.focus_dist))
            - viewport_u.scale(0.5)
            - viewport_v.scale(0.5);

        self.pixel00_loc =
            viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v).scale(0.5);

        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u.scale(defocus_radius);
        self.defocus_disk_v = self.v.scale(defocus_radius)
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
                    pixel_colour = pixel_colour + Self::ray_colour(&r, self.max_depth, world);
                }
                write_colour(&mut file, pixel_colour, self.samples_per_pixel)
            }
        }
    }
}
