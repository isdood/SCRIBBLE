use spark_std::whisper::{Whisper, WhisperOps, CrystalResonance};

#[test]
fn test_whisper_creation() {
    let w = Whisper::new(42);
    assert_eq!(w.get(), 42);
}

#[test]
fn test_whisper_arithmetic() {
    let a = Whisper::new(10);
    let b = Whisper::new(20);

    assert_eq!((a + b).get(), 30);
    assert_eq!((b - a).get(), 10);
    assert_eq!((a * b).get(), -56); // Overflow handling
}

#[test]
fn test_whisper_resonance() {
    let a = Whisper::new(10);
    let b = Whisper::new(20);

    let c = a.resonate(&b).unwrap();
    assert_eq!(c.get(), 10);
    assert!(c.phase().value() >= 0.0);
}

#[test]
fn test_whisper_phase_shift() {
    let a = Whisper::new(10);
    let b = a.phase_shift(1.0).unwrap();

    assert_eq!(a.get(), b.get());
    assert!(b.phase().value() > a.phase().value());
}

#[test]
fn test_crystal_operations() {
    let a = Whisper::new(10);
    let b = Whisper::new(20);

    let c = a.align(&b).unwrap();
    assert_eq!(c.get(), 10);

    let d = a.entangle(&b).unwrap();
    assert_eq!(d.get(), 10);
}
