// magicmath/src/floor.rs

/// Floor function for floating-point numbers
pub fn floor(x: f64) -> f64 {
    x.floor()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floor() {
        assert_eq!(floor(3.7), 3.0);
        assert_eq!(floor(-3.7), -4.0);
        assert_eq!(floor(0.0), 0.0);
        assert_eq!(floor(2.0), 2.0);
    }
}
