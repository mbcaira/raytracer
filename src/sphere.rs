use crate::material::Material;
use crate::vec::Vec3f;

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3f,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn ray_intersect(&self, orig: &Vec3f, direction: &mut Vec3f) -> (bool, f32) {
        let mut intersect = true;
        let l: Vec3f = self.center - *orig;
        let tca = l.dot(direction);
        let d2 = l.dot(&l) - tca * tca;
        if d2 > (self.radius * self.radius) {
            intersect = false;
        }
        let thc = (self.radius * self.radius - d2).sqrt();
        let mut t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0.0 {
            t0 = t1;
        }
        if t0 < 0.0 {
            intersect = false;
        }
        return (intersect, t0);
    }
}
