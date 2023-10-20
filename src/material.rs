use crate::vec::Vec3f;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse_colour: Vec3f,
    pub albedo: (f32, f32),
    pub specular_exponent: f32,
}
