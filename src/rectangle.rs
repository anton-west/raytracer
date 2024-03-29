use crate::material::Material;
use crate::vec3::{Vec3};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
pub struct RectangleXY {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    material: Material,
}

pub struct RectangleYZ {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Material,
}

pub struct RectangleXZ {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Material,
}

impl RectangleXY {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k:f64, material: Material) -> RectangleXY{
        RectangleXY { x0, x1, y0, y1, k, material }
    }

}

impl RectangleYZ {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k:f64, material: Material) -> RectangleYZ{
        RectangleYZ { y0, y1, z0, z1, k, material }
    }

}

impl RectangleXZ {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k:f64, material: Material) -> RectangleXZ{
        RectangleXZ { x0, x1, z0, z1, k, material }
    }

}

impl Hittable for RectangleXY {
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

impl Hittable for RectangleYZ {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        //find t value for when ray hits plane of rectangle
        let a_x = r.origin.x();
        let b_x = r.direction.x();

        let t = (self.k - a_x) / b_x;

        if t < t_min || t_max < t {
            return None
        }

        //check if ray hits rectangle
        let y = r.origin.y() + t * r.direction.y();
        let z = r.origin.z() + t * r.direction.z();

        if (self.y0 <= y) && (y <= self.y1) && (self.z0 <= z) && (z <= self.z1) {
            //is hit
            let mut rec = HitRecord::default();
            
            rec.t = t;
            rec.point = Vec3::new(self.k, y, z);
            let outward_normal = Vec3::new(1.0, 0.0, 0.0);
            rec.set_face_normal(r, outward_normal);
            rec.material = self.material;

            return Some(rec);
        } else {
            return None
        }
    }
}

impl Hittable for RectangleXZ {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        //find t value for when ray hits plane of rectangle
        let a_y = r.origin.y();
        let b_y = r.direction.y();

        let t = (self.k - a_y) / b_y;

        if t < t_min || t_max < t {
            return None
        }

        //check if ray hits rectangle
        let x = r.origin.x() + t * r.direction.x();
        let z = r.origin.z() + t * r.direction.z();

        if (self.x0 <= x) && (x <= self.x1) && (self.z0 <= z) && (z <= self.z1) {
            //is hit
            let mut rec = HitRecord::default();
            
            rec.t = t;
            rec.point = Vec3::new(x, self.k, z);
            let outward_normal = Vec3::new(0.0, 1.0, 0.0);
            rec.set_face_normal(r, outward_normal);
            rec.material = self.material;

            return Some(rec);
        } else {
            return None
        }
    }
}

/*
#[cfg(test)]
mod tests {
    //use super::*;
}
*/