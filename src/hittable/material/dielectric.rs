use crate::{
    scene::ray::Ray,
    utils::{colour::Colour, random_float, vec3::Vec3},
};

use super::Material;

pub struct Dielectric {
    ir: f32,
}

impl Dielectric {
    pub fn new(ir: f32) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        // Schlick's approximation for reflectance
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &crate::scene::ray::Ray,
        rec: &crate::hittable::HitRecord,
        attenuation: &mut crate::utils::colour::Colour,
        scattered: &mut crate::scene::ray::Ray,
    ) -> bool {
        *attenuation = Colour::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = Vec3::unit_vector(r_in.direction());

        let cos_theta = unit_direction.scale(-1.0).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_float() {
                Vec3::reflect(&unit_direction, &rec.normal)
            } else {
                Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
            };

        *scattered = Ray::new(rec.p, direction);
        true
    }
}
