use crate::vec::Vec3;

pub struct Sphere {
    pub center: Vec3<f32>,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3<f32>, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn ray_intersect(&self, orig: &Vec3<f32>, direction: &Vec3<f32>, mut t0: f32) -> bool {
        let l: Vec3<f32> = self.center - *orig;

        let tca = (l * *direction).sum();
        let d2 = (l * l).sum() - tca.powi(2);
        if d2 > self.radius.powi(2) {
            return false;
        }
        let thc = (self.radius.powi(2) - d2).sqrt();
        t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0.0 {
            t0 = t1;
        }
        if t0 < 0.0 {
            return false;
        }
        return true;
    }
}
