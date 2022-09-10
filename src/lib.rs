pub fn clamp(value: f64, lower: f64, upper: f64) -> f64 {
    if value < lower {
        lower
    } else if value > upper {
        upper
    } else {
        value
    }
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
}