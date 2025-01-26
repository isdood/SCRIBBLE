use spark_std::comp::{CrystalComp, CompResult, CompOrd};

#[test]
fn test_primitive_comparison() {
    assert_eq!(42i32.crystal_cmp(&24), CompResult::Greater);
    assert_eq!(24i32.crystal_cmp(&42), CompResult::Less);
    assert_eq!(42i32.crystal_cmp(&42), CompResult::Equal);
}

#[test]
fn test_float_comparison() {
    assert!(3.14f64.crystal_gt(&2.71));
    assert!(2.71f64.crystal_lt(&3.14));
    assert!(1.0f64.crystal_eq(&1.0));
}

#[test]
fn test_slice_comparison() {
    let a = vec![1, 2, 3];
    let b = vec![1, 2, 4];
    let c = vec![1, 2, 3];

    assert!(a.crystal_lt(&b));
    assert!(b.crystal_gt(&a));
    assert!(a.crystal_eq(&c));
}

#[test]
fn test_string_comparison() {
    let a = "hello";
    let b = "world";
    let c = "hello";

    assert!(a.crystal_lt(&b));
    assert!(b.crystal_gt(&a));
    assert!(a.crystal_eq(&c));
}

#[test]
fn test_ordering() {
    let asc = CompOrd::Ascending;
    let desc = CompOrd::Descending;

    assert_eq!(asc.apply(CompResult::Less), CompResult::Less);
    assert_eq!(desc.apply(CompResult::Less), CompResult::Greater);
    assert_eq!(asc.apply(CompResult::Equal), CompResult::Equal);
    assert_eq!(desc.apply(CompResult::Equal), CompResult::Equal);
}

#[test]
fn test_incomparable() {
    assert_eq!(CompResult::Incomparable.as_ordering(), None);
    assert!(CompResult::Incomparable.is_incomparable());
}

#[test]
fn test_comparison_methods() {
    let x = 42i32;
    let y = 24i32;

    assert!(x.crystal_gt(&y));
    assert!(x.crystal_ge(&y));
    assert!(y.crystal_lt(&x));
    assert!(y.crystal_le(&x));
    assert!(x.crystal_eq(&x));
}

#[test]
fn test_ord_reverse() {
    assert_eq!(CompOrd::Ascending.reverse(), CompOrd::Descending);
    assert_eq!(CompOrd::Descending.reverse(), CompOrd::Ascending);
}
