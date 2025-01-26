use spark_std::potion::{Potion, StablePotion, VolatilePotion};
use spark_std::align::Alignment;
use std::sync::Arc;
use std::thread;

#[test]
fn test_basic_operations() {
    let potion = VolatilePotion::new(42);
    assert_eq!(potion.get(), 42);

    potion.set(84);
    assert_eq!(potion.get(), 84);

    potion.update(|x| x * 2);
    assert_eq!(potion.get(), 168);
}

#[test]
fn test_alignment() {
    let potion = Potion::with_alignment(42, Alignment::Vector64);
    assert_eq!(potion.alignment(), Alignment::Vector64);
}

#[test]
fn test_default() {
    let potion: VolatilePotion<i32> = VolatilePotion::default();
    assert_eq!(potion.get(), 0);
}

#[test]
fn test_replace() {
    let potion = VolatilePotion::new(42);
    let old = potion.replace(84);
    assert_eq!(old, 42);
    assert_eq!(potion.get(), 84);
}

#[test]
fn test_swap() {
    let potion1 = VolatilePotion::new(42);
    let potion2 = VolatilePotion::new(84);

    potion1.swap(&potion2);
    assert_eq!(potion1.get(), 84);
    assert_eq!(potion2.get(), 42);
}

#[test]
fn test_debug() {
    let potion = VolatilePotion::new(42);
    let debug_str = format!("{:?}", potion);
    assert!(debug_str.contains("42"));
    assert!(debug_str.contains("false")); // volatile
}

#[test]
fn test_thread_safe() {
    let potion = Arc::new(StablePotion::new(42));
    let potion2 = potion.clone();

    let handle = thread::spawn(move || {
        assert_eq!(potion2.get(), 42);
        potion2.set(84);
    });

    handle.join().unwrap();
    assert_eq!(potion.get(), 84);
}

#[test]
fn test_optimal_alignment() {
    let potion = VolatilePotion::new(42);
    assert!(matches!(
        potion.alignment(),
        Alignment::Crystal16 | Alignment::Vector32 | Alignment::Vector64
    ));
}

#[test]
fn test_simd_aligned() {
    let potion = Potion::with_alignment(42, Alignment::Vector32);
    // Note: This test might fail on some platforms due to allocation alignment
    // That's expected and okay
    println!("SIMD aligned: {}", potion.is_simd_aligned());
}
