use crate::ray::Ray;
use crate::vec3::{Vec3, Point3, Color};

struct HitRecord {
    point: Point3,
    normal: Vec3,
    t: f64,
}

trait Hittable {
    fn hit(ray: &Ray, t_min: f64, t_max: f64, hit_record: &HitRecord) -> bool;
}