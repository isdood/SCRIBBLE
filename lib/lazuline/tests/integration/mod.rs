use crate::common::{setup, create_test_data, teardown};

#[test]
fn test_basic_initialization() {
    let instance = setup();
    assert!(instance.is_initialized());
    teardown(instance);
}

#[test]
fn test_channel_compute() {
    let mut instance = setup();
    let data = create_test_data(10);
    let result = instance.channel_compute(&data).unwrap();
    assert_eq!(result, vec![2.0f64; 10]);
    teardown(instance);
}

#[test]
fn test_empty_data() {
    let mut instance = setup();
    let data = Vec::<f64>::new();
    let result = instance.channel_compute(&data).unwrap();
    assert!(result.is_empty());
    teardown(instance);
}
