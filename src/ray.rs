use crate::vec3::*;

#[derive(Debug, Copy, Clone)]
pub struct  Ray {
    pub origin: Point3,
    pub direction: Point3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Point3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}
