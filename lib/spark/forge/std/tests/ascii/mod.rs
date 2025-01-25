use spark_std::ascii::{CrystalStr, AsciiSet};

#[test]
fn test_crystal_str_creation() {
    let s = CrystalStr::from("Hello, World!");
    assert_eq!(s.len(), 13);
    assert!(!s.is_empty());
}

#[test]
fn test_crystal_str_case() {
    let mut s = CrystalStr::from("Hello");
    assert!(!s.is_ascii_lowercase());
    assert!(!s.is_ascii_uppercase());

    s.make_ascii_lowercase();
    assert_eq!(s.to_string(), "hello");
    assert!(s.is_ascii_lowercase());

    s.make_ascii_uppercase();
    assert_eq!(s.to_string(), "HELLO");
    assert!(s.is_ascii_uppercase());
}

#[test]
fn test_crystal_str_validation() {
    assert!(CrystalStr::from_bytes(b"Hello").is_some());
    assert!(CrystalStr::from_bytes(&[0x80]).is_none());
}

#[test]
fn test_ascii_set() {
    let mut set = AsciiSet::new();
    assert!(set.is_empty());

    set.insert(b'a');
    set.insert(b'b');
    set.insert(b'c');

    assert_eq!(set.len(), 3);
    assert!(set.contains(b'a'));
    assert!(set.contains(b'b'));
    assert!(set.contains(b'c'));
    assert!(!set.contains(b'd'));

    set.remove(b'b');
    assert_eq!(set.len(), 2);
    assert!(!set.contains(b'b'));
}

#[test]
fn test_crystal_str_set_matching() {
    let s = CrystalStr::from("abc123");
    let mut digits = AsciiSet::new();
    let mut letters = AsciiSet::new();

    for b in b'0'..=b'9' {
        digits.insert(b);
    }

    for b in b'a'..=b'z' {
        letters.insert(b);
    }

    assert!(s.contains_set(&digits));
    assert!(s.contains_set(&letters));
    assert!(!s.matches_set(&digits));
    assert!(!s.matches_set(&letters));
}

#[test]
fn test_crystal_str_conversions() {
    let s = CrystalStr::from("Hello");
    let cow: std::borrow::Cow<str> = s.clone().into();
    assert_eq!(cow, "Hello");
    assert_eq!(s.as_bytes(), b"Hello");
}
