use lazuline::Lazuline;

pub fn setup() -> Lazuline {
    Lazuline::new().unwrap()
}

pub fn teardown(_instance: Lazuline) {
    // Cleanup code here if needed
}

pub fn create_test_data(size: usize) -> Vec<f64> {
    vec![1.0f64; size]
}
