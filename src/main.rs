mod vec3;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;

use hittable::HitRecord;
use hittable::Hittable;
use hittable_list::HittableList;
use crate::vec3::{Vec3, Color, Point3, unit_vector};
use crate::ray::Ray;
use crate::sphere::Sphere;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_HEIGHT: u32 = 256;
const IMAGE_WIDTH: u32 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as u32;

fn ray_color(r: &Ray, world: &HittableList) -> Color {

    let mut rec: HitRecord = HitRecord::default();
    
    if world.hit(r, 0.0, f64::MAX, &mut rec) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction = unit_vector(r.direction);
    let t = (unit_direction.y() + 1.0) * 0.5;
    Color::new(1.0, 1.0 , 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}


fn main() {

    //camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let camera_origin = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);

    let lower_left_corner = camera_origin - (horizontal/2.0) - (vertical/2.0) - Vec3(0.0, 0.0, focal_length);
    
    //world
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    //add spheres to list
    list.push( Box::new( Sphere::new(Point3::new(0.0, 0.0, -1.0 ), 0.5) ) );
    list.push( Box::new( Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0) ) );

    let world: HittableList = HittableList::new(list);

    //rendering
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {

        let progress = (1.0 - j as f64 / IMAGE_HEIGHT as f64) * 100.0;
        if j % 32 == 0 {eprintln!("progress: {:.2}%", progress)};


        for i in 0..IMAGE_WIDTH {

            let u = i as f64 / (IMAGE_WIDTH-1) as f64;
            let v  = j as f64 / (IMAGE_HEIGHT-1) as f64;

            let r = Ray { 
                origin: camera_origin,
                direction: lower_left_corner + horizontal * u + vertical * v - camera_origin,
            };
            
            let color: Color = ray_color(&r, &world);
            let color_string = vec3::color_to_string(color);
            println!("{color_string}");

        }
    }
}