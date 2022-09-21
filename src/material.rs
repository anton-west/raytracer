use raytracer::{clamp, random_f64};
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::{Vec3, Color, reflect, unit_vector, dot, refract};

#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum Material {
    Lambertian { albedo: Color, },
    Metallic {
        albedo: Color,
        fuzz: f64,
    },
    Dielectric { index_of_refraction: f64, },
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
        &Material::Metallic { albedo, fuzz} => {
            let reflected = reflect(unit_vector(r_in.direction), rec.normal());
            let reflected = reflected + clamp(fuzz,0.0,1.0) * Vec3::random_in_unit_sphere();
            *r_scattered = Ray::new(rec.point(), reflected);
            *attenuation = albedo;

            dot(r_scattered.direction, rec.normal) > 0.0
        }
        &Material::Dielectric { index_of_refraction } => {

            let refraction_ratio = if rec.front_face {1.0 / index_of_refraction} else {index_of_refraction};
            let unit_dir = unit_vector(r_in.direction);
            
            let cos_theta = dot(-unit_dir, rec.normal).min(1.0);
            let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

            let cannot_refract = refraction_ratio*sin_theta > 1.0; 
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_f64() {
                let reflected = reflect(unit_dir, rec.normal);
                *attenuation = Color::new(1.0, 1.0, 1.0);
                *r_scattered = Ray::new(rec.point(), reflected);
                return true
            } else {
                let refracted = refract(unit_dir, unit_vector(rec.normal), refraction_ratio);
                *attenuation = Color::new(1.0, 1.0, 1.0);
                *r_scattered = Ray::new(rec.point(), refracted);
                return true
            }
        }
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    //Schlick approximation for reflectance
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 + r0)*(1.0 - cosine).powi(5)
}