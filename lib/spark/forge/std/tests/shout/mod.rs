use spark_std::shout::{Shout, ShoutResult};

#[test]
fn test_shout_creation() {
    let s = Shout::new(42);
    assert_eq!(s.get(), 42);
}

#[test]
fn test_shout_arithmetic() {
    let a = Shout::new(1_000_000_000);
    let b = Shout::new(2_000_000_000);

    assert_eq!((a + b).get(), 3_000_000_000);
    assert_eq!((b - a).get(), 1_000_000_000);
    assert_eq!((a * b).get(), 2_000_000_000_000_000_000);
    assert_eq!((b / a).get(), 2);
}

#[test]
fn test_shout_echo() {
    let a = Shout::new(1_000_000_000);
    let b = Shout::new(2_000_000_000);

    let c = a.resonate(&b).unwrap();
    assert_eq!(c.get(), 1_000_000_000);
    assert!(c.echo().intensity() > 0.0);
    assert!(c.echo().delay() > 0.0);
    assert!(c.echo().decay() > 0.0);
    assert!(c.echo().reflections() > 0);
}

#[test]
fn test_shout_amplification() {
    let a = Shout::new(1_000_000_000);
    let b = Shout::new(2_000_000_000);

    let c = a.checked_mul(b).unwrap();
    assert_eq!(c.get(), 2_000_000_000_000_000_000);
    assert!(c.amplifier().gain() > 0.0);
    assert!(c.amplifier().purity() > 0.0);
    assert!(c.amplifier().efficiency() > 0.0);
    assert!(c.amplifier().modes() > 0);
}

#[test]
fn test_shout_comparison() {
    let a = Shout::new(1_000_000_000);
    let b = Shout::new(2_000_000_000);
    let c = Shout::new(1_000_000_000);

    assert!(a < b);
    assert!(a <= c);
    assert!(b > a);
    assert!(a >= c);
    assert_eq!(a, c);
    assert_ne!(a, b);
}
