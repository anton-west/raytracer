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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut ret_rec = None;
        let mut closest_so_far = t_max;
        
        for object in &self.list {
            let op_rec = object.hit(r, t_min, closest_so_far);
            match op_rec {
                Some(rec) => {
                    closest_so_far = rec.t;
                    ret_rec = Some(rec);
                },

                None => (),
            }
        }

        return ret_rec;
    }
}