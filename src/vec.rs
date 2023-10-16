use std::ops::{Add, Div, Index, Mul, Sub};

#[derive(Clone, Copy, Default, Debug)]
pub struct Vec3<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl Vec3<f32> {
    pub fn normalize(&mut self) {
        let length = (self.r * self.r + self.g * self.g + self.b * self.b).sqrt();
        self.r /= length;
        self.g /= length;
        self.b /= length;
    }

    pub fn dot(&self, other: &Vec3<f32>) -> f32 {
        self.r * other.r + self.g * other.g + self.b * other.b
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl<T> Mul for Vec3<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl<T> Div for Vec3<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            r: self.r / other.r,
            g: self.g / other.g,
            b: self.b / other.b,
        }
    }
}
