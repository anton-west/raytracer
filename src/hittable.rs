use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Vec3, Point3, dot, Color};
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,

    pub front_face: bool,

    pub material: Material,
}

impl HitRecord {
    //init
    pub fn default() -> HitRecord {
        HitRecord {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            material: Material::Lambertian { albedo: Color::BLACK },
        }
    }
    
    //getters
    pub fn point(&self) -> Point3 {
        self.point
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
    pub fn t(&self) -> f64 {
        self.t
    }
    pub fn material(&self) -> Material {
        self.material
    }

    //setters
    pub fn set_point(&mut self, val: Point3) -> () {
        self.point = val
    }
    pub fn set_normal(&mut self, val: Vec3) -> () {
        self.normal = val
    }
    pub fn set_t(&mut self, val: f64) -> () {
        self.t = val
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) -> () {
        self.front_face = dot(ray.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else {- outward_normal};
    }
    pub fn set_material(&mut self, val: Material) -> () {
        self.material = val
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn set_face_normal1() {
        let mut rec = HitRecord::default();
        let ray = Ray::new(Vec3::origin(), Vec3::new(0.0, 0.0, 1.0));
        let outward_normal = Vec3::new(0.0, 0.0, -1.0);
        
        rec.set_face_normal(&ray, outward_normal);
        
        assert_eq!(rec.front_face, true);
    }

    #[test]
    fn set_face_normal2() {
        let mut rec = HitRecord::default();
        let ray = Ray::new(Vec3::origin(), Vec3::new(0.0, 0.0, -1.0));
        let outward_normal = Vec3::new(0.0, 0.0, -1.0);
        
        rec.set_face_normal(&ray, outward_normal);
        
        assert_eq!(rec.front_face, false);
    }
}