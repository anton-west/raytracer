mod vec3;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod rectangle;
mod camera;
mod material;

use raytracer::{INFINITY, random_f64,};
use hittable::{Hittable};
use hittable_list::HittableList;
use crate::material::{Material, scatter};
use crate::vec3::{Vec3, Color, Point3, unit_vector,};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::rectangle::{Rectangle_xy, Rectangle_yz, Rectangle_xz};
use crate::camera::Camera;

use std::sync::Arc;
use std::thread;

use std::fs::File;
use std::path::Path;
use std::io::Write;

pub const OUTPUT_FILENAME: &str = "image.ppm";
pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMAGE_HEIGHT: u32 = 512;
pub const IMAGE_WIDTH: u32 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as u32;
pub const SAMPLES_PER_PIXEL: u32 = 15;
pub const MAX_DEPTH: u32 = 5;
pub const THREAD_N: u32 = 8;

//returns a color if ray r hits anything in world, otherwise returns sky color
fn ray_color(r: &Ray, world: &HittableList, depth: u32) -> Color {

    //handle recursion base case, i.e. depth is 0, no more reflections for rays
    if depth <= 0 {return Color::BLACK;}     //TODO: make color constants

    let op_rec = world.hit(r, 0.001, INFINITY);
    match op_rec {
        Some(rec) => {
            let mut r_scattered = Ray::new(Vec3::origin(), Vec3::origin());
            let mut attenuation = Color::BLACK;

            if scatter(&rec.material, r, &rec, &mut attenuation, &mut r_scattered) {
                return attenuation * ray_color(&r_scattered, world, depth-1)
            } else {
                return Color::BLACK
            }
        }
        
        None => {
            //no hit for ray, get sky color and return it
            let unit_direction = unit_vector(r.direction);
            let t = 0.5 * (unit_direction.y() + 1.0);

            return (1.0 - t) * Color::new(1.0, 1.0 , 1.0) + t * Color::new(0.5, 0.7, 1.0)    
        }
    }
     
}


fn main() {

    //world
    let mut list: Vec<Box<dyn Hittable + Send + Sync>> = Vec::new();

    //define some materials
    let material_ground = Material::Lambertian { albedo: Color::new(0.7,0.8,0.3) };
    let material_up = Material::Metallic { albedo: (Color::new(0.28,0.95,0.55)), fuzz: (0.05) };
    let material_left = Material::Metallic { albedo: Color::new(0.5, 0.45, 0.75), fuzz: 0.2, };
    let material_right = Material::Dielectric { index_of_refraction: (1.5), albedo: Color::new(0.8, 0.90, 0.81) };
    let material_center = Material::Lambertian { albedo: Color::new(0.8, 0.2, 0.1) };
    let material_behind = Material::Lambertian { albedo: Color::new(0.1, 0.2, 0.8) };
    let material_pink_glass = Material::Dielectric { index_of_refraction: 2.4, albedo: Color::new(0.99, 0.3, 0.8) };
    //add spheres to list
    //list.push( Box::new( Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, material_ground ) ) );
    //list.push( Box::new( Sphere::new(Point3::new( 0.0, 0.0,    -1.0), 0.5,   material_center ) ) );
    //list.push( Box::new( Sphere::new(Point3::new(-1.0, 0.0,    -1.0), 0.5,   material_left   ) ) );
    //list.push( Box::new( Sphere::new(Point3::new( 1.0, 0.0,    -1.0), 0.5,   material_right  ) ) );
    //list.push( Box::new( Sphere::new(Point3::new( -0.5,1.0,    -1.2), 0.5,   material_up     ) ) );
    //list.push( Box::new( Sphere::new(Point3::new( 1.3, 0.5,    -2.5), 0.8,   material_behind ) ) );
    //list.push( Box::new( Sphere::new(Point3::new( 0.0, 0.0,    -0.25), 0.1,   material_pink_glass ) ) );
    list.push( Box::new( Rectangle_xy::new(-1.0, 1.0, 0.1, 0.6, -1.0, material_ground ) ) );
    list.push( Box::new( Rectangle_yz::new(0.0, 1.0, -1.0, 1.0, 0.0, material_center ) ) );
    list.push( Box::new( Rectangle_xz::new(-1.0, 1.0, -1.0, 1.0, 0.0, material_center ) ) );
    
    
    let world = Arc::new(HittableList::new(list));

    //camera
    let look_from = Point3::new(1.0,0.5,0.0);
    let look_at = Point3::new(0.0,0.5,0.0);
    let vup = Vec3::new(0.0, 1.0,0.0);
    let vfov = 90.0;
    let aperture = 0.01;
    let focus_dist = (look_from - look_at).length();
    let camera = Camera::new(look_from, look_at, vup, vfov, ASPECT_RATIO, aperture, focus_dist);

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

    let number_of_lines_per_thread = IMAGE_HEIGHT / THREAD_N;
    let mut image_array = vec![vec![String::new(); (number_of_lines_per_thread * IMAGE_WIDTH) as usize]; THREAD_N as usize];
    let mut handles = Vec::with_capacity(THREAD_N as usize);

    for nth_thread in 0..THREAD_N {
    
        let arc_world = Arc::clone(&world);
        
        let remainder = if nth_thread == (THREAD_N-1) {IMAGE_HEIGHT % THREAD_N} else {0};

        let start_index = nth_thread * number_of_lines_per_thread;
        let end_index = (nth_thread + 1) * number_of_lines_per_thread + remainder;
        
        //new thread for every line in image
        handles.push(thread::spawn(move || {
            let mut color_array: Vec<String> = Vec::with_capacity((number_of_lines_per_thread * IMAGE_WIDTH) as usize);
            for j in (start_index..end_index).rev() {
                
                let progress = (1.0 - j as f64 / end_index as f64) * 100.0;
                if j % 4 == 0 && nth_thread == 0 {eprintln!("progress: {:.2}%", progress)};

                for i in 0..IMAGE_WIDTH {
                
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                    for _ in 0..SAMPLES_PER_PIXEL {
                        let u = (i as f64 + random_f64()) / (IMAGE_WIDTH-1) as f64;
                        let v  = (j as f64 + random_f64()) / (IMAGE_HEIGHT-1) as f64;

                        let r = camera.get_ray(u, v);
                
                        pixel_color += ray_color(&r, &arc_world, MAX_DEPTH);
                    }

                    let color_string = vec3::color_to_string(pixel_color, 2.0, SAMPLES_PER_PIXEL);
                    let color_string = format!("{color_string}\n");
                    color_array.push(color_string);
                }}
            (color_array, nth_thread)
        }));
    }

    for handle in handles {
        let res_vec = handle.join();
        let color_vec = res_vec.expect("thread did not return color vec");
        let vec = color_vec.0;
        let index = color_vec.1;
        image_array[index as usize] = vec;
    }

    for t in (0..THREAD_N).rev() {
        for element in &image_array[t as usize] {
            let color_string = element;
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