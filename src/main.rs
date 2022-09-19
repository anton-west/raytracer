mod vec3;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod camera;

use raytracer::{INFINITY, random_f64};
use hittable::{Hittable, HitRecord};
use hittable_list::HittableList;
use crate::vec3::{Vec3, Color, Point3, unit_vector};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::camera::Camera;

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMAGE_HEIGHT: u32 = 256;
pub const IMAGE_WIDTH: u32 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as u32;
pub const SAMPLES_PER_PIXEL: u32 = 250;

fn ray_color(r: &Ray, world: &HittableList) -> Color {

    let mut rec = HitRecord::default();
    
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction = unit_vector(r.direction);
    let t = (unit_direction.y() + 1.0) * 0.5;
    Color::new(1.0, 1.0 , 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}


fn main() {

    //world
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    //add spheres to list
    list.push( Box::new( Sphere::new(Point3::new(0.0, 0.0, -1.0 ), 0.5) ) );
    list.push( Box::new( Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0) ) );
    let world: HittableList = HittableList::new(list);

    //camera
    let camera = Camera::default();
    
    //rendering
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {

        let progress = (1.0 - j as f64 / IMAGE_HEIGHT as f64) * 100.0;
        if j % 32 == 0 {eprintln!("progress: {:.2}%", progress)};


        for i in 0..IMAGE_WIDTH {
            
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_f64()) / (IMAGE_WIDTH-1) as f64;
                let v  = (j as f64 + random_f64()) / (IMAGE_HEIGHT-1) as f64;

                let r = camera.get_ray(u, v);
            
                pixel_color = pixel_color + ray_color(&r, &world);  //TODO: impl add assign
            }
            let color_string = vec3::color_to_string(pixel_color, SAMPLES_PER_PIXEL);
            print!("{color_string}\n");
        }
    }
}