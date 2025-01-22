#[derive(Debug, Clone, Copy)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub quantum_coherence: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            quantum_coherence: 1.0
        }
    }

    pub fn dot(&self, other: &Vector3D) -> f64 {
        // Apply quantum coherence to dot product
        let classical_dot = self.x * other.x + self.y * other.y + self.z * other.z;
        let coherence = self.quantum_coherence.min(other.quantum_coherence);
        classical_dot * coherence
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt() * self.quantum_coherence
    }

    pub fn set_coherence(&mut self, coherence: f64) {
        self.quantum_coherence = coherence.clamp(0.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
        assert_eq!(v.quantum_coherence, 1.0);
    }

    #[test]
    fn test_dot_product() {
        let mut v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2), 32.0); // Full coherence

        v1.set_coherence(0.5);
        assert_eq!(v1.dot(&v2), 16.0); // Half coherence
    }
}
