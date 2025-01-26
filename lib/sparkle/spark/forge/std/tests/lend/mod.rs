use spark_std::lend::{CrystalLend, CrystalLendMut, Lender, LendError};
use std::sync::Arc;
use std::cell::Cell;

#[test]
fn test_basic_lending() {
    let mut value = 42;

    // Test immutable lend
    let lend = value.lend();
    assert_eq!(*lend, 42);

    // Test mutable lend
    let mut lend_mut = value.lend_mut();
    *lend_mut = 84;
    assert_eq!(value, 84);
}

#[test]
fn test_multiple_immutable_lends() {
    let value = String::from("test");

    let lend1 = value.lend();
    let lend2 = value.lend();

    assert_eq!(&*lend1, "test");
    assert_eq!(&*lend2, "test");
}

#[test]
fn test_lend_arc() {
    let arc = Arc::new(42);
    let lend = arc.lend();
    assert_eq!(*lend, 42);
}

#[test]
fn test_crystal_lend_debug() {
    let value = 42;
    let lend = value.lend();
    let debug_str = format!("{:?}", lend);
    assert!(debug_str.contains("42"));
}

#[test]
fn test_crystal_lend_mut_debug() {
    let mut value = 42;
    let lend = value.lend_mut();
    let debug_str = format!("{:?}", lend);
    assert!(debug_str.contains("42"));
}

#[test]
fn test_lend_cell() {
    let cell = Cell::new(42);
    let lend = cell.lend();
    assert_eq!(lend.get(), 42);
}

#[test]
fn test_try_lend() {
    let mut value = 42;

    // Test successful immutable lend
    let lend = value.try_lend().unwrap();
    assert_eq!(*lend, 42);

    // Test successful mutable lend
    let mut lend_mut = value.try_lend_mut().unwrap();
    *lend_mut = 84;
    assert_eq!(value, 84);
}
