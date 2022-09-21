mod vec3;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod camera;

use raytracer::{INFINITY, random_f64,};
use hittable::{Hittable, HitRecord};
use hittable_list::HittableList;
use crate::vec3::{Vec3, Color, Point3, unit_vector,};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::camera::Camera;

use std::fs::File;
use std::path::Path;
use std::io::Write;

pub const OUTPUT_FILENAME: &str = "image2.ppm";
pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMAGE_HEIGHT: u32 = 400;
pub const IMAGE_WIDTH: u32 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as u32;
pub const SAMPLES_PER_PIXEL: u32 = 50;
pub const MAX_DEPTH: u32 = 25;

//returns a color if ray r hits anything in world, otherwise returns sky color
fn ray_color(r: &Ray, world: &HittableList, depth: u32) -> Color {

    //handle recursion base case, i.e. depth is 0, no more reflections for rays
    if depth <= 0 {return Color::origin();}     //TODO: make color constants

    let mut rec = HitRecord::default();
    
    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let target = rec.point + rec.normal + Vec3::random_unit_vector();
        let new_ray = &Ray::new(rec.point, target - rec.point);
        return  0.5 * ray_color(new_ray, world, depth - 1) ;
    }
    
    //no hit for ray, get sky color and return it
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

    //open output file for writing
    //TODO: move file io to own function
    let path = Path::new(OUTPUT_FILENAME);
    let display = path.display();
    let mut output_file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let image_header = format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
    match output_file.write_all(image_header.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => (),
    }

    //rendering loop, shoot rays into world and color pixels accordingly
    //start timer here to measure rendering time
    let now = std::time::Instant::now();

    for j in (0..IMAGE_HEIGHT).rev() {

        let progress = (1.0 - j as f64 / IMAGE_HEIGHT as f64) * 100.0;
        if j % 4 == 0 {eprintln!("progress: {:.2}%", progress)};

        for i in 0..IMAGE_WIDTH {
            
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_f64()) / (IMAGE_WIDTH-1) as f64;
                let v  = (j as f64 + random_f64()) / (IMAGE_HEIGHT-1) as f64;

                let r = camera.get_ray(u, v);
            
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }

            let color_string = vec3::color_to_string(pixel_color, 2.0, SAMPLES_PER_PIXEL);
            let color_string = format!("{color_string}\n");
            match output_file.write_all(color_string.as_bytes()) {
                Err(why) => panic!("couldn't write to {}: {}", display, why),
                Ok(_) => (),
            }
        }
    }

    let elapsed_dur = now.elapsed();
    
    let hours = elapsed_dur.as_secs() / (60*60);
    let mins = elapsed_dur.as_secs() / 60 - hours*60;
    let frac_sec = elapsed_dur.as_secs_f64() - (mins*60) as f64;

    eprintln!("\nTime to render: {:02}:{:02}:{:05.02}", hours, mins, frac_sec);

}