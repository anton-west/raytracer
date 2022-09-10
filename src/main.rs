use raytracer::*;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_HEIGHT: u32 = 1024;
const IMAGE_WIDTH: u32 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as u32;

fn main() {

    //camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let camera_origin = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);

    let lower_left_corner = camera_origin - (horizontal/2.0) - (vertical/2.0) - Vec3(0.0, 0.0, focal_length); 

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {

        let progress = (1.0 - j as f64 / IMAGE_HEIGHT as f64) * 100.0;
        if j % 32 == 0 {eprintln!("progress: {:.2}%", progress)};


        for i in 0..IMAGE_WIDTH {

            let u = i as f64 / (IMAGE_WIDTH-1) as f64;
            let v  = j as f64 / (IMAGE_HEIGHT-1) as f64;

            let ray = Ray { 
                origin: camera_origin,
                direction: lower_left_corner + horizontal * u + vertical * v - camera_origin,
            };
            
            let color = ray_color(&ray);
            let color_string = color_to_string(color);
            println!("{color_string}");

        }
    }
}

fn ray_color(ray: &Ray) -> Color {

    let t = hit_sphere(&Vec3(0.0, 0.0, -1.0), 0.5, ray);

    if t > 0.0 {
        let normal = unit_vector(ray.at(t) - Vec3(0.0, 0.0, -1.0));
        return Vec3(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0) * 0.5
    }

    let unit_direction = ray.direction;
    let t = (unit_direction.y() + 1.0) * 0.5;
    Vec3(1.0, 1.0 , 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc: Vec3 = ray.origin - *center;
    let a = ray.direction.length_squared();
    let half_b = dot(oc, ray.direction);
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b * half_b - a*c;
    
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - f64::sqrt(discriminant)) / a
    }
}