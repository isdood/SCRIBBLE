use spark_std::cubed::{Cubed, CubedAlloc};
use spark_std::align::Alignment;
use std::mem;

#[test]
fn test_basic_allocation() {
    let cubed = Cubed::new(42);
    assert_eq!(*cubed, 42);
}

#[test]
fn test_custom_alignment() {
    let alloc = CubedAlloc::new(Alignment::Vector64);
    let cubed = Cubed::with_allocator(42, alloc);
    assert_eq!(cubed.alignment(), Alignment::Vector64);
    assert!(cubed.is_simd_aligned());
}

#[test]
fn test_optimal_alignment() {
    let cubed = Cubed::new(42);
    assert!(matches!(
        cubed.alignment(),
        Alignment::Crystal16 | Alignment::Vector32 | Alignment::Vector64
    ));
}

#[test]
fn test_into_inner() {
    let cubed = Cubed::new(String::from("test"));
    let value = cubed.into_inner();
    assert_eq!(value, "test");
}

#[test]
fn test_debug_impl() {
    let cubed = Cubed::new(42);
    let debug_str = format!("{:?}", cubed);
    assert!(debug_str.contains("42"));
    assert!(debug_str.contains("alignment"));
}

#[test]
fn test_drop() {
    struct DropCounter<'a> {
        counter: &'a mut i32,
    }

    impl<'a> Drop for DropCounter<'a> {
        fn drop(&mut self) {
            *self.counter += 1;
        }
    }

    let mut counter = 0;
    {
        let _cubed = Cubed::new(DropCounter { counter: &mut counter });
    }
    assert_eq!(counter, 1);
}

#[test]
fn test_zero_sized_type() {
    let cubed: Cubed<()> = Cubed::new(());
    assert_eq!(mem::size_of_val(&*cubed), 0);
}

#[test]
fn test_large_alignment() {
    #[repr(align(64))]
    struct Aligned64(u64);

    let cubed = Cubed::new(Aligned64(42));
    assert!(cubed.is_simd_aligned());
}
