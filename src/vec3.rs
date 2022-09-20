use std::ops;
use raytracer::clamp;

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

        let i = a.1*b.2 - a.2*b.1;
        let j = a.0*b.2 - a.2*b.0;
        let k = a.0*b.1 - a.1*b.0;

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

fn cross(a: Vec3, b: Vec3) -> Vec3 {
    let i = a.1*b.2 - a.2*b.1;
    let j = a.0*b.2 - a.2*b.0;
    let k = a.0*b.1 - a.1*b.0;

    Vec3(i,j,k)
}

pub fn unit_vector(vector: Vec3) -> Vec3 {
    vector / vector.length()
}

pub fn color_to_string(color: Color, samples_per_pixel: u32) -> String{
    let x = color.0;
    let y = color.1;
    let z = color.2;

    let scale = 1.0 / (samples_per_pixel as f64);

    let r = x * scale;
    let g = y * scale;
    let b = z * scale;

    let r = (256.0 * clamp(r, 0.0, 0.999)) as u8;
    let g = (256.0 * clamp(g, 0.0, 0.999)) as u8;
    let b = (256.0 * clamp(b, 0.0, 0.999)) as u8;

    format!("{r} {g} {b}")
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
    fn cross_overload() {
        let a = Vec3(1.0, 0.0, 0.0);
        let b = Vec3(0.0, 1.0, 0.0);
        let result = b*a;
        assert_eq!(result, Vec3(0.0, 0.0, -1.0))
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
    fn write_color1() {
        let a: Color = Vec3(0.5, 0.5 , 0.5);
        let result = color_to_string(a, 1);
        assert_eq!(result, String::from("128 128 128"))
    }

    #[test]
    fn write_color2() {
        let a: Color = Vec3(0.5, -1.0 , 3.0);
        let result = color_to_string(a, 1);
        assert_eq!(result, String::from("128 0 255"))
    }

    #[test]
    fn write_color3() {
        let a: Color = Vec3(1.0, 1.0 , 1.0);
        let result = color_to_string(a, 1);
        assert_eq!(result, String::from("255 255 255"))
    }

    #[test]
    fn write_color4() {
        let a: Color = Vec3(1.0, 1.0 , 1.0);
        let b: Color = Vec3(1.0, 1.0 , 0.0);
        let c: Color = Vec3(1.0, 0.0 , 0.0);
        let final_res = a + b + c;
        let result = color_to_string(final_res, 3);
        assert_eq!(result, String::from("255 170 85"))
    }
}