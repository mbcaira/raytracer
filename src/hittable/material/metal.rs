use crate::{Colour, Ray, Vec3};

use super::Material;

#[derive(Default, Clone, Copy)]
pub struct Metal {
    albedo: Colour,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &crate::scene::ray::Ray,
        rec: &crate::hittable::HitRecord,
        attenuation: &mut Colour,
        scattered: &mut crate::scene::ray::Ray,
    ) -> bool {
        let reflected = Vec3::reflect(&Vec3::unit_vector(r_in.direction()), &rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + Vec3::random_unit_vector().scale(self.fuzz),
        );
        *attenuation = self.albedo;

        scattered.direction().dot(&rec.normal) > 0.0
    }
}
