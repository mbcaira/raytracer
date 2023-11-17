pub mod material;
pub mod sphere;

use std::rc::Rc;

use crate::{utils::interval::Interval, Point3, Ray, Vec3};

use self::material::{lambertian::Lambertian, Material};
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub mat: Rc<dyn Material>,
}

impl Default for HitRecord {
    fn default() -> Self {
        let default_material: Rc<dyn Material> = Rc::new(Lambertian::default());

        Self {
            p: Point3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
            mat: default_material,
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = outward_normal.scale(-1.0);
        }
    }
}

pub trait Hittable {
    fn hit(&mut self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&mut self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &mut self.objects {
            if object.hit(ray, Interval::new(ray_t.min, closest_so_far), rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }
}
