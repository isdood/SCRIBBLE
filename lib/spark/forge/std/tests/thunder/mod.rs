use spark_std::thunder::Thunder;
use std::f64;

#[test]
fn test_basic_arithmetic() {
    let a = Thunder::new(2.0);
    let b = Thunder::new(3.0);

    assert_eq!(Thunder::new(5.0), a + b);
    assert_eq!(Thunder::new(-1.0), a - b);
    assert_eq!(Thunder::new(6.0), a * b);
    assert_eq!(Thunder::new(2.0/3.0), a / b);
}

#[test]
fn test_constants() {
    assert!(Thunder::INFINITY.is_infinite());
    assert!(Thunder::NEG_INFINITY.is_infinite());
    assert!(Thunder::NAN.is_nan());
    assert!(!Thunder::MAX.is_infinite());
    assert!(!Thunder::MIN.is_infinite());
}

#[test]
fn test_methods() {
    let x = Thunder::new(4.0);
    assert_eq!(Thunder::new(2.0), x.sqrt());
    assert_eq!(Thunder::new(4.0), x.abs());
    assert_eq!(Thunder::new(4.0), x.floor());
    assert_eq!(Thunder::new(4.0), x.ceil());
}

#[test]
fn test_comparison() {
    let a = Thunder::new(2.0);
    let b = Thunder::new(3.0);

    assert!(a < b);
    assert!(b > a);
    assert_ne!(a, b);
    assert_eq!(a, a);
}

#[test]
fn test_special_values() {
    assert!(Thunder::new(0.0).is_zero());
    assert!(Thunder::INFINITY.is_infinite());
    assert!(Thunder::NAN.is_nan());
    assert!(Thunder::MAX.is_finite());
}

#[test]
fn test_conversions() {
    let x = 2.5f64;
    let t = Thunder::from(x);
    let y: f64 = t.into();
    assert_eq!(x, y);
}

#[test]
fn test_bit_patterns() {
    let bits = 0x4000000000000000u64; // 2.0 in IEEE 754
    let t = Thunder::from_bits(bits);
    assert_eq!(t, Thunder::new(2.0));
    assert_eq!(t.to_bits(), bits);
}
