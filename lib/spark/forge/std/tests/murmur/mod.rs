use spark_std::murmur::{Murmur, MurmurResult};

#[test]
fn test_murmur_creation() {
    let m = Murmur::new(42);
    assert_eq!(m.get(), 42);
}

#[test]
fn test_murmur_arithmetic() {
    let a = Murmur::new(1000);
    let b = Murmur::new(2000);

    assert_eq!((a + b).get(), 3000);
    assert_eq!((b - a).get(), 1000);
    assert_eq!((a * b).get(), 2_000_000);
    assert_eq!((b / a).get(), 2);
}

#[test]
fn test_murmur_wave_propagation() {
    let a = Murmur::new(1000);
    let b = Murmur::new(2000);

    let c = a.checked_add(b).unwrap();
    assert_eq!(c.get(), 3000);
    assert!(c.wave().amplitude() > 0.0);
    assert!(c.wave().frequency() > 0.0);
}

#[test]
fn test_murmur_resonance() {
    let a = Murmur::new(1000);
    let b = Murmur::new(2000);

    let c = a.checked_mul(b).unwrap();
    assert_eq!(c.get(), 2_000_000);
    assert!(c.resonance().energy() > 0.0);
    assert!(c.resonance().coherence() > 0.0);
}

#[test]
fn test_murmur_comparison() {
    let a = Murmur::new(1000);
    let b = Murmur::new(2000);
    let c = Murmur::new(1000);

    assert!(a < b);
    assert!(a <= c);
    assert!(b > a);
    assert!(a >= c);
    assert_eq!(a, c);
    assert_ne!(a, b);
}
