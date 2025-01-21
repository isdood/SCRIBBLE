#![cfg(test)]

use crate::common::{setup, create_test_data, teardown};

mod common;
mod integration;
mod parallel;

#[test]
fn test_create_instance() {
    let instance = setup();
    assert!(instance.is_initialized());
    teardown(instance);
}

#[test]
fn test_multiple_computations() {
    let mut instance = setup();

    // First computation
    let data1 = create_test_data(5);
    let result1 = instance.channel_compute(&data1).unwrap();
    assert_eq!(result1, vec![2.0f64; 5]);

    // Second computation
    let data2 = vec![2.0f64; 3];
    let result2 = instance.channel_compute(&data2).unwrap();
    assert_eq!(result2, vec![4.0f64; 3]);

    teardown(instance);
}
