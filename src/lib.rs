use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Point3,
}

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
    
    pub fn get_components(&self) -> (f64, f64, f64) {
        (self.0, self.1 , self.2)
    }

    pub fn length(self) -> f64 {
        let x = self.0;
        let y = self.1;
        let z = self.2;

        f64::sqrt(x*x + y*y + z*z)
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

pub fn dot(a: Vec3, b: Vec3) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    let i = a.1*b.2 - a.2*b.1;
    let j = a.0*b.2 - a.2*b.0;
    let k = a.0*b.1 - a.1*b.0;

    Vec3(i,j,k)
}

pub fn unit_vector(vector: Vec3) -> Vec3 {
    vector / vector.length()
}

impl Ray {
    pub fn new(origin: Point3, direction: Point3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}

pub fn clamp(value: f64, lower: f64, upper: f64) -> f64 {
    if value < lower {
        lower
    } else if value > upper {
        upper
    } else {
        value
    }
}

pub fn color_to_string(color: Color) -> String{
    let x = clamp(color.0, 0.0, 1.0);
    let y = clamp(color.1, 0.0, 1.0);
    let z = clamp(color.2, 0.0, 1.0);

    let r = (x * 255.99) as u8;
    let g = (y * 255.99) as u8;
    let b = (z * 255.99) as u8;

    format!("{r} {g} {b}")
}

// tests go here
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
    fn scalar_mul() {
        let a = Vec3(1.0,2.0,3.0);
        let result = a * 5.0;
        assert_eq!(result, Vec3(5.0, 10.0, 15.0));
    }

    #[test]
    fn scalar_div() {
        let a = Vec3(1.0,2.0,3.0);
        let result = a / -2.0;
        assert_eq!(result, Vec3(-0.5, -1.0, -1.5));
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
    fn clamp1() {
        let a = 3.4;
        let result = clamp(a, 0.0 , 1.0);
        assert_eq!(result, 1.0)
    }

    #[test]
    fn clamp2() {
        let a = 3.4;
        let result = clamp(a, 0.0 , 5.0);
        assert_eq!(result, 3.4)
    }
   
    #[test]
    fn clamp3() {
        let a = 3.4;
        let result = clamp(a, 4.0 , 5.0);
        assert_eq!(result, 4.0)
    }

    #[test]
    fn write_color1() {
        let a: Color = Vec3(0.5, 0.5 , 0.5);
        let result = color_to_string(a);
        assert_eq!(result, String::from("127 127 127"))
    }

    #[test]
    fn write_color2() {
        let a: Color = Vec3(0.5, -1.0 , 3.0);
        let result = color_to_string(a);
        assert_eq!(result, String::from("127 0 255"))
    }
}