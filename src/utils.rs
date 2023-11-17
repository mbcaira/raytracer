pub mod colour;
pub mod interval;
pub mod vec3;

pub fn random_float() -> f32 {
    rand::random()
}

pub fn random_float_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random_float()
}
