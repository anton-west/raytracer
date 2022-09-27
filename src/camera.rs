use raytracer::deg_to_rad;

use crate::{ASPECT_RATIO};
use crate::vec3::{Vec3, Point3, unit_vector, cross};
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct Camera {
    vfov: f64,
    
    aperture: f64,
    focus_dist: f64,

    aspect_ratio: f64,

    viewport_height: f64,
    viewport_width: f64,
    
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,

    upper_left_corner: Point3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Camera {

        let vfov = deg_to_rad(vfov);
        let h = (vfov/2.0).tan();
        let aspect_ratio = aspect_ratio;
        let viewport_height =  2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let upper_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Camera {
            vfov,
            aperture,
            focus_dist,
            aspect_ratio,
            viewport_height,
            viewport_width,
            origin,
            horizontal,
            vertical,
            upper_left_corner
        }
    }

    pub fn default() -> Camera {
        let vfov = deg_to_rad(90.0);
        let h = (vfov/2.0).tan();
        let aspect_ratio = ASPECT_RATIO;
        let viewport_height =  2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        
        let focus_dist = 1.0;
        let aperture = 2.0;

        let look_from = Vec3::origin();
        let look_at = Vec3::new(0.0, 0.0, -1.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);

        let w = unit_vector(look_from- look_at);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let upper_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Camera {
            vfov,
            aperture,
            focus_dist,
            aspect_ratio,
            viewport_height,
            viewport_width,
            origin,
            horizontal,
            vertical,
            upper_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = (self.aperture/2.0) * Vec3::random_in_unit_disk();
        let uu = unit_vector(self.horizontal);
        let vv = unit_vector(self.vertical);
        let offset = uu * rd.x() + vv * rd.y();
        Ray { 
            origin: self.origin + offset,
            direction: self.upper_left_corner + self.horizontal*u + self.vertical*v - self.origin - offset
        }
    }
}