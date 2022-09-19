use crate::ray::Ray;
use crate::vec3::{Vec3, Point3, dot};
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,

    pub front_face: bool,
}

impl HitRecord {
    //init
    pub fn default() -> HitRecord {
        HitRecord {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: true
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
}
