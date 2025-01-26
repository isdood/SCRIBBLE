use spark_std::clone::{CrystalClone, CloneStrategy, CloneError};
use std::mem;

#[test]
fn test_primitive_clone() {
    let x = 42i32;
    assert_eq!(x.crystal_clone().unwrap(), 42);
    assert_eq!(x.clone_strategy(), CloneStrategy::Memcopy);
}

#[test]
fn test_option_clone() {
    let x: Option<i32> = Some(42);
    assert_eq!(x.crystal_clone().unwrap(), Some(42));

    let x: Option<i32> = None;
    assert_eq!(x.crystal_clone().unwrap(), None);
}

#[test]
fn test_result_clone() {
    let x: Result<i32, &str> = Ok(42);
    assert_eq!(x.crystal_clone().unwrap(), Ok(42));

    let x: Result<i32, &str> = Err("error");
    assert_eq!(x.crystal_clone().unwrap(), Err("error"));
}

#[test]
fn test_clone_strategy() {
    let helper = spark_std::clone::CloneHelper::new();
    assert!(helper.should_memcopy::<i32>());
    assert!(helper.should_memcopy::<f64>());
    assert!(helper.should_memcopy::<char>());
}

#[test]
fn test_alignment() {
    let helper = spark_std::clone::CloneHelper::new();

    #[repr(align(32))]
    struct Aligned32([u8; 32]);

    let data = Aligned32([0; 32]);
    let ptr = &data as *const Aligned32;

    unsafe {
        let cloned = helper.clone_bytes(ptr, 1);
        assert_eq!(cloned as usize % 32, 0);
        std::alloc::dealloc(
            cloned as *mut u8,
            std::alloc::Layout::from_size_align(32, 32).unwrap()
        );
    }
}

#[test]
fn test_large_clone() {
    let data = vec![0u8; 1024];
    let helper = spark_std::clone::CloneHelper::new();

    unsafe {
        let cloned = helper.clone_bytes(data.as_ptr(), data.len());
        assert_eq!(std::slice::from_raw_parts(cloned, data.len()), &data[..]);
        std::alloc::dealloc(
            cloned as *mut u8,
            std::alloc::Layout::from_size_align(1024, helper.alignment.as_bytes()).unwrap()
        );
    }
}
