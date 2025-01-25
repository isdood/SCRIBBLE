use spark_std::rune::{Rune, RuneError};
use std::str::FromStr;

#[test]
fn test_basic_rune() {
    let r = Rune::new(0x41).unwrap();
    assert_eq!(r.as_u32(), 0x41);
    assert_eq!(format!("{}", r), "A");
}

#[test]
fn test_invalid_rune() {
    assert!(Rune::new(0x110000).is_none());
    assert!(Rune::new(0xD800).is_none());
}

#[test]
fn test_ascii() {
    let r = Rune::from('A');
    assert!(r.is_ascii());

    let r = Rune::from('λ');
    assert!(!r.is_ascii());
}

#[test]
fn test_utf8_encoding() {
    let r = Rune::from('λ');
    assert_eq!(r.encode_utf8(), vec![0xCE, 0xBB]);
}

#[test]
fn test_from_str() {
    assert_eq!(Rune::from_str("A").unwrap(), Rune::from('A'));
    assert!(Rune::from_str("AB").is_err());
}

#[test]
fn test_classification() {
    let r = Rune::from('A');
    assert!(r.is_alphabetic());
    assert!(!r.is_numeric());
    assert!(r.is_alphanumeric());
    assert!(!r.is_whitespace());
    assert!(!r.is_control());
}

#[test]
fn test_arithmetic() {
    let r = Rune::from('A');
    assert_eq!(r + 1, Some(Rune::from('B')));
    assert_eq!(r - 1, Some(Rune::from('@')));
}

#[test]
fn test_display() {
    let r = Rune::from('λ');
    assert_eq!(format!("{}", r), "λ");
    assert_eq!(format!("{:?}", r), "Rune('λ')");
}

#[test]
fn test_ordering() {
    let a = Rune::from('A');
    let b = Rune::from('B');
    assert!(a < b);
}

#[test]
fn test_replacement() {
    assert_eq!(format!("{}", Rune::REPLACEMENT), "�");
}
