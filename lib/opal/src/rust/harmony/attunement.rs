pub struct Attunement {
    factor: f64,
}

impl Attunement {
    pub fn new(factor: f64) -> Self {
        Self { factor }
    }

    pub fn adjust(&mut self, delta: f64) {
        self.factor += delta;
    }

    pub fn optimize(&mut self) {
        self.factor *= 1.02;
    }
}
