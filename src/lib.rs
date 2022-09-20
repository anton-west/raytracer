use rand::Rng;

//handy rng helpers
pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    Rng::gen_range(&mut rng, 0.0..1.0)
}

//return random float in range [min, max)
pub fn random_in_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    Rng::gen_range(&mut rng, min..max)
}

//handy math helpers
pub const PI: f64 = std::f64::consts::PI;

pub const INFINITY: f64 = f64::INFINITY;

pub fn clamp(value: f64, lower: f64, upper: f64) -> f64 {
    if value < lower {
        lower
    } else if value > upper {
        upper
    } else {
        value
    }
}

pub fn deg_to_rad(deg: f64) -> f64 {
    deg *  PI / 180.0
}

// tests go here
#[cfg(test)]
mod tests {
    use super::*;

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
    fn deg_to_rad1() {
        let deg = 90.0;
        let result = deg_to_rad(deg);
        assert_eq!(result, PI / 2.0)
    }
}