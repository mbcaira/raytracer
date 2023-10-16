use crate::vec::Vec3;

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3<f32>,
    pub radius: f32,
}

impl Sphere {
    pub fn ray_intersect(&self, orig: &Vec3<f32>, direction: &Vec3<f32>) -> bool {
        let l: Vec3<f32> = self.center - *orig;
        let tca = l.dot(direction);
        let d2 = l.dot(&l) - tca * tca;
        if d2 > (self.radius * self.radius) {
            return false;
        }
        let thc = (self.radius * self.radius - d2).sqrt();
        let mut t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0.0 {
            t0 = t1;
        }
        if t0 < 0.0 {
            return false;
        }
        true
    }
}
