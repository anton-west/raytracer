use crate::{ASPECT_RATIO, IMAGE_HEIGHT, IMAGE_WIDTH};
use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;

pub struct Camera {
    aspect_ratio: f64,

    viewport_height: f64,
    viewport_width: f64,
    focal_length: f64,
    
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,

    lower_left_corner: Point3,
}

impl Camera {
    pub fn default() -> Camera {
        let aspect_ratio = ASPECT_RATIO;
        let viewport_height =  2.0;
        let viewport_width = ASPECT_RATIO * viewport_height;
        let focal_length = 1.0;
        let origin = Point3::origin();
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray { 
            origin: self.origin,
            direction: self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin
        }
    }
}