use spark_std::coll::{CrystalVec, CollectionAlignment, CrystalCollection};

#[test]
fn test_crystal_vec_basic() {
    let mut vec = CrystalVec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    assert_eq!(vec.len(), 3);
    assert_eq!(vec.pop(), Some(3));
    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.pop(), Some(1));
    assert_eq!(vec.pop(), None);
}

#[test]
fn test_crystal_vec_drop() {
    use std::rc::Rc;
    use std::cell::Cell;

    let counter = Rc::new(Cell::new(0));
    {
        let mut vec = CrystalVec::new();
        let counter_clone = counter.clone();
        vec.push(DropCounter(counter_clone));
        assert_eq!(counter.get(), 0);
    }
    assert_eq!(counter.get(), 1);
}

#[test]
fn test_crystal_vec_alignment_behavior() {
    let vec = CrystalVec::<u8>::with_alignment(CollectionAlignment::Custom(128));
    assert_eq!(vec.alignment().as_bytes(), 128);

    let vec = CrystalVec::<u8>::new();
    assert_eq!(vec.alignment().as_bytes(), std::mem::size_of::<usize>());
}

#[test]
fn test_crystal_vec_as_slice() {
    let mut vec = CrystalVec::new();
    for i in 0..5 {
        vec.push(i);
    }

    let slice = vec.as_slice();
    assert_eq!(slice, &[0, 1, 2, 3, 4]);

    let mut_slice = vec.as_mut_slice();
    mut_slice[0] = 42;
    assert_eq!(vec.as_slice(), &[42, 1, 2, 3, 4]);
}

#[test]
fn test_crystal_vec_reserve() {
    let mut vec = CrystalVec::new();
    vec.reserve(100);
    for i in 0..50 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 50);
    assert_eq!(vec.as_slice()[49], 49);
}

#[test]
fn test_crystal_vec_zero_sized() {
    let mut vec = CrystalVec::<()>::new();
    vec.push(());
    vec.push(());
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.pop(), Some(()));
}

#[test]
fn test_crystal_vec_large_items() {
    #[derive(Debug, PartialEq)]
    struct LargeStruct {
        data: [u8; 1024],
    }

    let mut vec = CrystalVec::new();
    vec.push(LargeStruct { data: [42; 1024] });
    assert_eq!(vec.len(), 1);
    let item = vec.pop().unwrap();
    assert_eq!(item.data[0], 42);
}

#[test]
#[should_panic(expected = "Capacity overflow")]
fn test_crystal_vec_capacity_overflow() {
    let mut vec = CrystalVec::<i32>::new();
    vec.reserve(usize::MAX);
}

// Helper struct for drop tests
struct DropCounter(Rc<Cell<u32>>);

impl Drop for DropCounter {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

// Tests for thread safety
#[test]
fn test_crystal_vec_sync_send() {
    fn assert_sync<T: Sync>() {}
    fn assert_send<T: Send>() {}

    assert_send::<CrystalVec<i32>>();
    assert_sync::<CrystalVec<i32>>();
}

#[test]
fn test_crystal_collection_trait() {
    let mut vec = CrystalVec::new();
    vec.push(1);
    vec.push(2);

    assert!(!vec.is_empty());
    vec.clear();
    assert!(vec.is_empty());
}

#[test]
fn test_crystal_vec_with_custom_alignment() {
    for align in [2, 4, 8, 16, 32, 64, 128] {
        let vec = CrystalVec::<u8>::with_alignment(CollectionAlignment::Custom(align));
        assert_eq!(vec.alignment().as_bytes(), align);
    }
}
