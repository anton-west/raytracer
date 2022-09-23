use crate::material::Material;
use crate::vec3::{Vec3, Point3, dot};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Sphere{
        Sphere { center, radius, material }
    }

}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

        let oc: Vec3 = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = dot(oc, r.direction);
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;   //no hit
        }
        
        let sqrtd = discriminant.sqrt();
        
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        
        let mut ret_rec = HitRecord::default();
        ret_rec.t = root;
        ret_rec.point = r.at(ret_rec.t);
        ret_rec.material = self.material;

        let outward_normal = (ret_rec.point - self.center) / self.radius;
        ret_rec.set_face_normal(r, outward_normal);

        return Some(ret_rec);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere1() {
        let r = Ray::new(Vec3::origin(), Vec3::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Vec3::origin(), 5.0, Material::Lambertian { albedo: Vec3::origin() });
        
        let did_hit = sphere.hit(&r, 0.001, 10000.0);
        
        assert_eq!(did_hit.is_some(), true);

    }

    #[test]
    fn sphere2() {
        let r = Ray::new(Vec3::origin(), Vec3::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Vec3::origin(), 5.0, Material::Lambertian { albedo: Vec3::origin() });
        
        let op_rec = sphere.hit(&r, 0.001, 10000.0);

        assert_eq!(op_rec.expect("Should not be none!").front_face, false);

    }

    #[test]
    fn sphere3() {
        let r = Ray::new(Vec3::origin(), Vec3::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 10.0), 5.0, Material::Lambertian { albedo: Vec3::origin() });
        
        let op_rec = sphere.hit(&r, 0.001, 10000.0);

        assert_eq!(op_rec.expect("Should not be none!").front_face, true);

    }
}