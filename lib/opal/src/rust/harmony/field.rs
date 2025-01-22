pub struct Field {
    strength: f64,
}

impl Field {
    pub fn new(strength: f64) -> Self {
        Self { strength }
    }

    pub fn strengthen(&mut self, increment: f64) {
        self.strength += increment;
    }

    pub fn optimize(&mut self) {
        self.strength *= 1.03;
    }
}
