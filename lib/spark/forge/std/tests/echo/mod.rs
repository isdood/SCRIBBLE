use spark_std::echo::{CrystalEcho, EchoFmt, Hex, Binary, Octal};
use std::borrow::Cow;

#[test]
fn test_static_echo() {
    let e = CrystalEcho::new("test");
    assert_eq!(e.as_str(), "test");
    assert!(e.is_static());
}

#[test]
fn test_owned_echo() {
    let e = CrystalEcho::owned(String::from("test"));
    assert_eq!(e.as_str(), "test");
    assert!(!e.is_static());
}

#[test]
fn test_formatted_echo() {
    let e = CrystalEcho::fmt(42);
    assert_eq!(e.as_str(), "42");
}

#[test]
fn test_echo_macro() {
    let x = 42;
    let e = echo!("value: {}", x);
    assert_eq!(e.as_str(), "value: 42");
}

#[test]
fn test_numeric_formats() {
    let n = 42u32;
    assert_eq!(CrystalEcho::fmt(Hex(n)).as_str(), "0x2a");
    assert_eq!(CrystalEcho::fmt(Binary(n)).as_str(), "0b101010");
    assert_eq!(CrystalEcho::fmt(Octal(n)).as_str(), "0o52");
}

#[test]
fn test_capacity_hints() {
    let s = "test";
    assert_eq!(CrystalEcho::new(s).capacity_hint(), 4);

    let n = 42i32;
    let e = CrystalEcho::fmt(n);
    assert!(e.capacity_hint() >= 2);
}

#[test]
fn test_custom_echo_fmt() {
    struct Custom(i32);

    impl EchoFmt for Custom {
        fn format(&self) -> Cow<'_, str> {
            Cow::Owned(format!("Custom({})", self.0))
        }
    }

    let c = Custom(42);
    let e = CrystalEcho::fmt(c);
    assert_eq!(e.as_str(), "Custom(42)");
}

#[test]
fn test_char_echo() {
    let c = 'x';
    let e = CrystalEcho::fmt(c);
    assert_eq!(e.as_str(), "x");
}
