use crate::material::Material;
use crate::vec3::{Vec3, Point3, dot};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
pub struct Rectangle {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    material: Material,
}

impl Rectangle {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k:f64, material: Material) -> Rectangle{
        Rectangle { x0, x1, y0, y1, k, material }
    }

}

impl Hittable for Rectangle {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        //find t value for when ray hits plane of rectangle
        let a_z = r.origin.z();
        let b_z = r.direction.z();

        let t = (self.k - a_z) / b_z;

        if t < t_min || t_max < t {
            return None
        }

        //check if ray hits rectangle
        let x = r.origin.x() + t * r.direction.x();
        let y = r.origin.y() + t * r.direction.y();

        if (self.x0 <= x) && (x <= self.x1) && (self.y0 <= y) && (y <= self.y1) {
            //is hit
            let mut rec = HitRecord::default();
            
            rec.t = t;
            rec.point = Vec3::new(x, y, self.k);
            let outward_normal = Vec3::new(0.0, 0.0, 1.0);
            rec.set_face_normal(r, outward_normal);
            rec.material = self.material;

            return Some(rec);
        } else {
            return None
        }
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
}