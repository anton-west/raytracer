use crate::vec3::{Vec3, Point3, dot};
use crate::hittable::{HitRecord, Hittable};
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(self, ray: &crate::ray::Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let oc: Vec3 = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;   //no hit
        }
        
        let sqrtd = f64::sqrt(discriminant);

        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);
        hit_record.normal = (hit_record.point - self.center) / self.radius;

        return true;
    }
}