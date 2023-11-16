use std::rc::Rc;

use crate::utils::interval::Interval;
use crate::Hittable;
use crate::Point3;
use crate::Ray;

use super::material::Material;
use super::HitRecord;

pub struct Sphere {
    center: Point3,
    radius: f32,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, mat: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&mut self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = *r.origin() - self.center;
        let a = r.direction().length() * r.direction().length();
        let half_b = oc.dot(r.direction());
        let c = oc.length() * oc.length() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        // Find nearest root in the range
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        // Define range for the hit to count for normals to be drawn
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);

        let outward_normal = (rec.p - self.center).scale(1.0 / self.radius);
        rec.set_face_normal(r, &outward_normal);
        rec.mat = Rc::clone(&self.mat);
        return true;
    }
}
