use spark_std::def::{CrystalDefault, CrystalInit};
use spark_std::align::Alignment;

#[test]
fn test_numeric_defaults() {
    assert_eq!(u32::crystal_default(), 0);
    assert_eq!(i64::crystal_default(), 0);
    assert_eq!(f32::crystal_default(), 0.0);
}

#[test]
fn test_array_defaults() {
    let arr: [i32; 4] = CrystalDefault::crystal_default();
    assert_eq!(arr, [0, 0, 0, 0]);
}

#[test]
fn test_container_defaults() {
    let vec: Vec<i32> = CrystalDefault::crystal_default();
    assert!(vec.is_empty());

    let string: String = CrystalDefault::crystal_default();
    assert!(string.is_empty());
}

#[test]
fn test_alignment_optimization() {
    assert_eq!(<[u32; 8]>::optimal_alignment(), Alignment::Vector32);
    assert_eq!(<u8>::optimal_alignment(), Alignment::Crystal16);
}

#[test]
fn test_zero_initialization() {
    let zero_i32 = i32::crystal_zeroed();
    assert_eq!(zero_i32, 0);

    let zero_arr: [i32; 4] = CrystalInit::crystal_zeroed();
    assert_eq!(zero_arr, [0, 0, 0, 0]);
}

#[test]
fn test_uninitialized() {
    let mut uninit = i32::crystal_uninit();
    unsafe {
        *uninit.as_mut_ptr() = 42;
        assert_eq!(uninit.assume_init(), 42);
    }
}

#[test]
fn test_bool_default() {
    assert_eq!(bool::crystal_default(), false);
    assert_eq!(bool::crystal_zeroed(), false);
}

#[test]
fn test_string_zero() {
    let zero_string = String::crystal_zeroed();
    assert!(zero_string.is_empty());
}
