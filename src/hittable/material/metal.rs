use crate::{Colour, Ray, Vec3};

use super::Material;

#[derive(Default, Clone, Copy)]
pub struct Metal {
    albedo: Colour,
}

impl Metal {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
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
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        true
    }
}
