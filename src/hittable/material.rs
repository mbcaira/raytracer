pub mod lambertian;
pub mod metal;

use crate::{scene::ray::Ray, utils::colour::Colour};

use super::HitRecord;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool;
}
