use std::ops::{Add, Index, Mul, Sub};

use super::{random_float, random_float_range};
#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn scale(&self, scalar: f32) -> Self {
        Vec3 {
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z,
        }
    }

    pub fn dot(&self, v: &Vec3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(&self, v: &Vec3) -> Self {
        Vec3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    pub fn unit_vector(&self) -> Self {
        self.scale(1.0 / self.length())
    }

    pub fn random() -> Self {
        Self::new(random_float(), random_float(), random_float())
    }

    pub fn random_range(min: f32, max: f32) -> Self {
        Self::new(
            random_float_range(min, max),
            random_float_range(min, max),
            random_float_range(min, max),
        )
    }

    fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_range(-1.0, 1.0);
            if (p.length() * p.length()) < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self[0].abs() < s && self[1].abs() < s && self[2].abs() < s
    }

    pub fn reflect(v: &Self, n: &Self) -> Self {
        *v - n.scale(2.0 * v.dot(n))
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Self {
        let cos_theta = uv.scale(-1.0).dot(n).min(1.0);
        let r_out_perp = (*uv + n.scale(cos_theta)).scale(etai_over_etat);
        let r_out_parallel = n.scale(
            -(1.0 - r_out_perp.length() * r_out_perp.length())
                .abs()
                .sqrt(),
        );

        r_out_perp + r_out_parallel
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(
                random_float_range(-1.0, 1.0),
                random_float_range(-1.0, 1.0),
                0.0,
            );

            if (p.length() * p.length()) < 1.0 {
                return p;
            }
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds."),
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
