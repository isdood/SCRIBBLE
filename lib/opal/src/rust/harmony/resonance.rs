pub struct Resonance {
    level: f64,
}

impl Resonance {
    pub fn new(level: f64) -> Self {
        Self { level }
    }

    pub fn amplify(&mut self, factor: f64) {
        self.level *= factor;
    }

    pub fn optimize(&mut self) {
        self.level *= 1.01;
    }
}
