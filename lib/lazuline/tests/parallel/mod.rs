use crate::common::{setup, create_test_data};

#[test]
fn test_parallel_processing() {
    let mut instance = setup();
    let data = create_test_data(1000);
    let result = instance.channel_compute(&data).unwrap();
    assert_eq!(result.len(), data.len());
}

#[test]
fn test_large_data_processing() {
    let mut instance = setup();
    let data = create_test_data(1_000_000);
    let result = instance.channel_compute(&data).unwrap();
    assert_eq!(result, vec![2.0f64; 1_000_000]);
}
