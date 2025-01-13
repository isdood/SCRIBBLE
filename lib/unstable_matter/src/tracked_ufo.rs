#[derive(Debug, Clone)]
pub struct TrackedUFO {
    origin: usize,
    boundary: usize,
}

impl TrackedUFO {
    pub fn with_boundary(origin: usize, boundary: usize) -> Self {
        Self { origin, boundary }
    }

    pub fn track(&self) -> bool {
        true // Placeholder implementation
    }
}
