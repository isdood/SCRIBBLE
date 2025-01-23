use num_complex::Complex64;

pub struct QuantumState {
    amplitude: Complex64,
}

impl QuantumState {
    pub fn new(amplitude: Complex64) -> Self {
        QuantumState { amplitude }
    }

    pub fn get_probability(&self) -> f64 {
        self.amplitude.norm_sqr()
    }
}
