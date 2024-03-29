use std::ops;
use raytracer::{clamp, random_f64, random_in_range};

#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }
    
    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn new(x:f64, y:f64, z:f64) -> Vec3 {
        Vec3(x,y,z)
    }

    pub fn origin() -> Point3 {
        Vec3(0.0, 0.0, 0.0)
    }

    pub fn random() -> Vec3 {
        Vec3(random_f64(), random_f64(), random_f64())
    }

    pub fn random_in_range(min: f64, max: f64) -> Vec3 {
        Vec3(
            random_in_range(min, max),
            random_in_range(min, max),
            random_in_range(min, max),
        )
    }

    pub fn get_components(&self) -> (f64, f64, f64) {
        (self.0, self.1 , self.2)
    }

    pub fn length(self) -> f64 {
        let x = self.0;
        let y = self.1;
        let z = self.2;

        (x*x + y*y + z*z).sqrt()
    }

    pub fn length_squared(self) -> f64 {
        let x = self.0;
        let y = self.1;
        let z = self.2;

        x*x + y*y + z*z
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut p = Vec3::random_in_range(-1.0, 1.0);
        while p.length_squared() >= 1.0 {
            p = Vec3::random_in_range(-1.0, 1.0);
        }
        return p
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut p = Vec3::new(random_in_range(-1.0, 1.0), random_in_range(-1.0, 1.0), 0.0);
        while p.length_squared() >= 1.0 {
            p = Vec3::new(random_in_range(-1.0, 1.0), random_in_range(-1.0, 1.0), 0.0);
        }
        return p
    }

    pub fn random_unit_vector() -> Vec3 {
        unit_vector(Vec3::random_in_unit_sphere())
    }

    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        (self.0 < s) && (self.1 < s) && (self.2 < s)
    }

}

impl Color {
    pub const RED: Vec3 = Vec3(1.0, 0.0, 0.0);
    pub const GREEN: Vec3 = Vec3(0.0, 1.0, 0.0);
    pub const BLUE: Vec3 = Vec3(0.0, 0.0, 1.0);
    pub const YELLOW: Vec3 = Vec3(1.0, 1.0, 0.0);
    pub const MAGENTA: Vec3 = Vec3(1.0, 0.0, 1.0);
    pub const BLACK: Vec3 = Vec3(0.0, 0.0, 0.0);
    pub const WHITE: Vec3 = Vec3(1.0, 1.0, 1.0);
    pub const GRAY: Vec3 = Vec3(0.5, 0.5, 0.5);
}

impl ops::Neg for Vec3{
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        self + - other
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Vec3 {
        Vec3(self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec3: Vec3) -> Vec3 {
        Vec3(vec3.0 * self, vec3.1 * self, vec3.2 * self)
    }
}


impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        let a = &self;
        let b = &rhs;

        let i = a.0*b.0;
        let j = a.1*b.1;
        let k = a.2*b.2;

        Vec3(i,j,k)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    
    fn div(self, scalar: f64) -> Vec3 {
        self * (1.0 / scalar)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        if self.0 == other.0 && self.1 == other.1 && self.2 == other.2 {
            true
        } else {
            false
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) -> () {
        *self = *self + rhs;
    }
}

pub fn dot(a: Vec3, b: Vec3) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    let i = a.1*b.2 - a.2*b.1;
    let j = a.2*b.0 - a.0*b.2;
    let k = a.0*b.1 - a.1*b.0;

    Vec3(i,j,k)
}

pub fn unit_vector(vector: Vec3) -> Vec3 {
    vector / vector.length()
}

pub fn color_to_string(color: Color, gamma: f64, samples_per_pixel: u32) -> String{
    let x = color.0;
    let y = color.1;
    let z = color.2;

    let scale = 1.0 / (samples_per_pixel as f64);

    let r = (x * scale).powf(1.0/gamma);
    let g = (y * scale).powf(1.0/gamma);
    let b = (z * scale).powf(1.0/gamma);

    let r = (256.0 * clamp(r, 0.0, 0.999)) as u8;
    let g = (256.0 * clamp(g, 0.0, 0.999)) as u8;
    let b = (256.0 * clamp(b, 0.0, 0.999)) as u8;

    format!("{r} {g} {b}")
}

pub fn reflect(vec_incoming: Vec3, normal: Vec3) -> Vec3 {
    vec_incoming - 2.0 * dot(vec_incoming, normal)*normal
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(-uv, n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta*n);
    let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;

    r_out_perp + r_out_parallel
}

///////////////////
// tests go here //
///////////////////

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add() {
        let a = Vec3(1.0,2.0,3.0);
        let b = Vec3(2.0,4.0,5.0);
        let result = a + b;
        assert_eq!(result, Vec3(3.0,6.0,8.0));
    }

    #[test]
    fn sub() {
        let a = Vec3(1.0,2.0,3.0);
        let b = Vec3(2.0,4.0,5.0);
        let result = a - b;
        assert_eq!(result, Vec3(-1.0,-2.0,-2.0));
    }

    #[test]
    fn neg() {
        let a = Vec3(1.0,2.0,3.0);
        let result = -a;
        assert_eq!(result, Vec3(-1.0,-2.0,-3.0));
    }

    #[test]
    fn scalar_mul1() {
        let a = Vec3(1.0,2.0,3.0);
        let result = a * 5.0;
        assert_eq!(result, Vec3(5.0, 10.0, 15.0));
    }

    #[test]
    fn scalar_mul2() {
        let a = Vec3(1.0,2.0,3.0);
        let result = 5.0 * a;
        assert_eq!(result, Vec3(5.0, 10.0, 15.0));
    }

    #[test]
    fn scalar_div() {
        let a = Vec3(1.0,2.0,3.0);
        let result = a / -2.0;
        assert_eq!(result, Vec3(-0.5, -1.0, -1.5));
    }

    #[test]
    fn add_assign() {
        let mut a = Vec3::origin();
        a += Vec3::new(1.0, 1.0, 1.0);
        a += Vec3::new(0.0, -2.0, 1.0);
        let result = a;
        assert_eq!(result, Vec3(1.0, -1.0, 2.0));
    }
    
    #[test]
    fn dot1() {
        let a = Vec3(1.0, 0.0, 0.0);
        let b = Vec3(0.0, 1.0, 0.0);
        let result = dot(a,b);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn dot2() {
        let a = Vec3(1.0, 1.0, 1.0);
        let b = Vec3(3.0, 3.0, 3.0);
        let result = dot(a,b);
        assert_eq!(result, 9.0)
    }

    #[test]
    fn dot3() {
        let a = Vec3(0.0, 1.0, 3.0);
        let b = Vec3(-1.0, 0.0, -2.0);
        let result = dot(a,b);
        assert_eq!(result < 0.0, true);
    }

    #[test]
    fn cross1() {
        let a = Vec3(1.0, 0.0, 0.0);
        let b = Vec3(0.0, 1.0, 0.0);
        let result = cross(a,b);
        assert_eq!(result, Vec3(0.0, 0.0, 1.0))
    }

    #[test]
    fn cross2() {
        let a = Vec3(1.0, 0.0, 0.0);
        let b = Vec3(0.0, 1.0, 0.0);
        let result = cross(b,a);
        assert_eq!(result, Vec3(0.0, 0.0, -1.0))
    }

    #[test]
    fn cross3() {
        let a = Vec3(1.0, 2.0, 3.0);
        let b = Vec3(0.5, 4.0, -2.0);
        let result = cross(a,b);
        assert_eq!(result, Vec3(-16.0, 3.5, 3.0))
    }

    #[test]
    fn mul_overload() {
        let a = Vec3(1.0, 0.0, 0.0);
        let b = Vec3(0.0, 1.0, 0.0);
        let result = b*a;
        assert_eq!(result, Vec3(0.0, 0.0, 0.0))
    }

    #[test]
    fn length() {
        let a = Vec3(2.0, 4.0, 8.0);
        let result = a.length();
        assert_eq!(result, f64::sqrt(84.0))
    }

    #[test]
    fn length_squared() {
        let a = Vec3(2.0, 4.0, 8.0);
        let result = a.length_squared();
        assert_eq!(result, 84.0)
    }

    #[test]
    fn unit_vec() {
        let a = Vec3(2.0, 3.0, 4.0);
        let result = unit_vector(a);
        let div = f64::sqrt(29.0);
        assert_eq!(result, Vec3(2.0 / div, 3.0 / div, 4.0 / div))
    }

    #[test]
    fn refract1() {
        let v_in = Vec3(1.0, 1.0, 10.0);
        let n = Vec3(0.0, 0.0, -1.0);
        let result = refract(v_in, n, 0.5);
        assert_eq!(result.0, 0.5);
    }

    #[test]
    fn refract2() {
        let v_in = Vec3(1.0, 2.0, 10.0);
        let n = Vec3(0.0, 0.0, -1.0);
        let result = refract(v_in, n, 0.5);
        assert_eq!(result.1, 1.0);
    }

    #[test]
    fn write_color1() {
        let a: Color = Vec3(0.5, 0.5 , 0.5);
        let result = color_to_string(a, 1.0, 1);
        assert_eq!(result, String::from("128 128 128"))
    }

    #[test]
    fn write_color2() {
        let a: Color = Vec3(0.5, -1.0 , 3.0);
        let result = color_to_string(a, 1.0, 1);
        assert_eq!(result, String::from("128 0 255"))
    }

    #[test]
    fn write_color3() {
        let a: Color = Vec3(1.0, 1.0 , 1.0);
        let result = color_to_string(a, 1.0, 1);
        assert_eq!(result, String::from("255 255 255"))
    }

    #[test]
    fn write_color4() {
        let a: Color = Vec3(1.0, 1.0 , 1.0);
        let b: Color = Vec3(1.0, 1.0 , 0.0);
        let c: Color = Vec3(1.0, 0.0 , 0.0);
        let final_res = a + b + c;
        let result = color_to_string(final_res, 2.0, 3);
        assert_eq!(result, String::from("255 209 147"))
    }
}