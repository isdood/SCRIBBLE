use spark_std::voice::{Voice, VoiceResult};

#[test]
fn test_voice_creation() {
    let v = Voice::new(42);
    assert_eq!(v.get(), 42);
}

#[test]
fn test_voice_arithmetic() {
    let a = Voice::new(100_000);
    let b = Voice::new(200_000);

    assert_eq!((a + b).get(), 300_000);
    assert_eq!((b - a).get(), 100_000);
    assert_eq!((a * b).get(), 20_000_000_000);
    assert_eq!((b / a).get(), 2);
}

#[test]
fn test_voice_harmonics() {
    let a = Voice::new(100_000);
    let b = Voice::new(200_000);

    let c = a.harmonize(&b).unwrap();
    assert_eq!(c.get(), 100_000);
    assert!(c.harmonic().frequency() > 0.0);
    assert!(c.harmonic().amplitudes().iter().all(|&x| x > 0.0));
}

#[test]
fn test_voice_resonance() {
    let a = Voice::new(100_000);
    let b = Voice::new(200_000);

    let c = a.checked_mul(b).unwrap();
    assert_eq!(c.get(), 20_000_000_000);
    assert!(c.resonator().energy() > 0.0);
    assert!(c.resonator().coherence() > 0.0);
    assert!(c.resonator().quality() > 0.0);
}

#[test]
fn test_voice_comparison() {
    let a = Voice::new(100_000);
    let b = Voice::new(200_000);
    let c = Voice::new(100_000);

    assert!(a < b);
    assert!(a <= c);
    assert!(b > a);
    assert!(a >= c);
    assert_eq!(a, c);
    assert_ne!(a, b);
}
