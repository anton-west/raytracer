use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;


pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> HittableList {
        HittableList { list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.list {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.t = closest_so_far;
                rec.point = temp_rec.point;
                rec.normal = temp_rec.normal;
                rec.front_face = temp_rec.front_face;
                rec.material = temp_rec.material;
            }
        }
        hit_anything
    }
}