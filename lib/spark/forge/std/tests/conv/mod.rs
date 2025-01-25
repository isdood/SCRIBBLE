use spark_std::conv::{CrystalFrom, CrystalInto, CrystalTryFrom, CrystalTryInto};
use spark_std::align::Alignment;

#[test]
fn test_numeric_conversions() {
    let x: u8 = 42;
    let y: u16 = x.crystal_into();
    assert_eq!(y, 42u16);

    let z: u32 = u16::crystal_from(y);
    assert_eq!(z, 42u32);
}

#[test]
fn test_array_conversions() {
    let arr: [u8; 4] = [1, 2, 3, 4];
    let converted: [u16; 4] = arr.crystal_try_into().unwrap();
    assert_eq!(converted, [1, 2, 3, 4]);
}

#[test]
fn test_string_conversions() {
    let s = String::from("Hello");
    let bytes: Vec<u8> = s.crystal_into();
    let back: String = bytes.crystal_try_into().unwrap();
    assert_eq!(back, "Hello");
}

#[test]
fn test_alignment_optimization() {
    assert_eq!(<[u32; 8]>::optimal_alignment(), Alignment::Vector32);
    assert_eq!(<[u8; 4]>::optimal_alignment(), Alignment::Crystal16);
}

#[test]
fn test_conversion_error() {
    let invalid_utf8: Vec<u8> = vec![0xFF, 0xFF];
    let result: Result<String, _> = invalid_utf8.crystal_try_into();
    assert!(result.is_err());
}

#[test]
fn test_numeric_conversion_alignment() {
    assert_eq!(<u128>::optimal_alignment(), Alignment::Vector32);
    assert_eq!(<u8>::optimal_alignment(), Alignment::Crystal16);
}
