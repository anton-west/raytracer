use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::{Vec3, Color, reflect, unit_vector, dot};

#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum Material {
    Lambertian { albedo: Color },
    Metallic { albedo: Color},
    //Dielectric {},
}

pub fn scatter(
        material: &Material,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        r_scattered: &mut Ray
    ) -> bool {
    
    match material {
        &Material::Lambertian { albedo } => {
            let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

            if scatter_direction.near_zero() {
                scatter_direction = rec.normal;
            }

            *r_scattered = Ray::new(rec.point, scatter_direction);
            *attenuation = albedo;
            true
        }
        &Material::Metallic { albedo } => {
            let reflected = reflect(unit_vector(r_in.direction), rec.normal());
            *r_scattered = Ray::new(rec.point(), reflected);
            *attenuation = albedo;

            dot(r_scattered.direction, rec.normal) > 0.0
        }
    }
}