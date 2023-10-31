use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::Point3;

use super::HitRecord;

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32, rec: &mut HitRecord) -> bool {
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
        if root <= ray_tmin || ray_tmax <= root {
            root = (-half_b + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);

        let outward_normal = (rec.p - self.center).scale(1.0 / self.radius);
        rec.set_face_normal(r, &outward_normal);
        return true;
    }
}