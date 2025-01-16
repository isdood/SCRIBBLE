use crate::scribe::{Scribe, ScribePrecision, QuantumString};
use crate::unstable_matter::{PLANCK_LENGTH, QUANTUM_THRESHOLD};
use std::f64::consts::PI;

/// Metadata for the cereal format
pub const CEREAL_VERSION: u16 = 1;
pub const CEREAL_MAGIC: [u8; 4] = [0xCE, 0xAF, 0x25, 0x01]; // CE(real) AF(fixed) 2025 01(version)
pub const QUANTUM_PRECISION: usize = 35; // Planck scale precision

/// Native quantum serialization error types
#[derive(Debug)]
pub enum CerealError {
    BufferOverflow,
    InvalidMagic,
    QuantumDecoherence,
    StateCollapse,
    InvalidChecksum,
    CoherenceLoss,
}

/// Quantum-safe result type
pub type CerealResult<T> = Result<T, CerealError>;

/// Quantum binary buffer with coherence tracking
#[derive(Debug)]
pub struct QuantumBuffer {
    data: Vec<u8>,
    coherence: f64,
    quantum_state: u8,
    checksum: u64,
    created: u64,  // UTC timestamp
    author: [u8; 32], // Fixed size for username
}

impl QuantumBuffer {
    pub fn new() -> Self {
        let mut buffer = Self {
            data: Vec::with_capacity(1024),
            coherence: 1.0,
            quantum_state: 0,
            checksum: 0,
            created: 1705371857, // 2025-01-16 02:24:17 UTC
            author: [0; 32],
        };

        // Set author (isdood)
        let author = b"isdood";
        buffer.author[..author.len()].copy_from_slice(author);

        // Write header
        buffer.write_magic();
        buffer
    }

    fn write_magic(&mut self) {
        self.data.extend_from_slice(&CEREAL_MAGIC);
        self.data.extend_from_slice(&CEREAL_VERSION.to_le_bytes());
        self.update_quantum_state();
    }

    pub fn write_f64(&mut self, value: f64) -> CerealResult<()> {
        self.data.extend_from_slice(&value.to_le_bytes());
        self.update_quantum_state();
        self.verify_coherence()?;
        Ok(())
    }

    pub fn read_f64(&mut self, pos: &mut usize) -> CerealResult<f64> {
        if *pos + 8 > self.data.len() {
            return Err(CerealError::BufferOverflow);
        }
        let bytes = self.data[*pos..*pos + 8].try_into()
        .map_err(|_| CerealError::InvalidMagic)?;
        *pos += 8;
        self.verify_coherence()?;
        Ok(f64::from_le_bytes(bytes))
    }

    fn update_quantum_state(&mut self) {
        self.coherence *= 0.99999; // Slight decay per operation
        self.quantum_state = ((self.data.len() as f64 * PI) % 255.0) as u8;
        self.update_checksum();
    }

    fn update_checksum(&mut self) {
        self.checksum = self.data.iter()
        .enumerate()
        .fold(0, |acc, (i, &byte)| {
            acc.wrapping_add((byte as u64).wrapping_mul(i as u64))
        });
    }

    fn verify_coherence(&self) -> CerealResult<()> {
        if self.coherence < QUANTUM_THRESHOLD {
            Err(CerealError::CoherenceLoss)
        } else {
            Ok(())
        }
    }
}

/// Quantum-aware 3D vector with serialization
#[derive(Debug, Clone)]
pub struct QuantumVector3D {
    x: f64,
    y: f64,
    z: f64,
    phase: f64,     // Quantum phase
    coherence: f64,  // Coherence factor
}

/// Native serialization trait
pub trait Cereal: Sized {
    fn cerealize(&self, buffer: &mut QuantumBuffer) -> CerealResult<()>;
    fn decerealize(buffer: &mut QuantumBuffer, pos: &mut usize) -> CerealResult<Self>;
}

impl QuantumVector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            phase: 0.0,
            coherence: 1.0,
        }
    }

    pub fn with_quantum(x: f64, y: f64, z: f64, phase: f64, coherence: f64) -> Self {
        Self {
            x,
            y,
            z,
            phase: phase % (2.0 * PI),
            coherence: coherence.clamp(0.0, 1.0),
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag > PLANCK_LENGTH {
            self.x /= mag;
            self.y /= mag;
            self.z /= mag;
        }
    }
}

impl Cereal for QuantumVector3D {
    fn cerealize(&self, buffer: &mut QuantumBuffer) -> CerealResult<()> {
        buffer.write_f64(self.x)?;
        buffer.write_f64(self.y)?;
        buffer.write_f64(self.z)?;
        buffer.write_f64(self.phase)?;
        buffer.write_f64(self.coherence)?;
        Ok(())
    }

    fn decerealize(buffer: &mut QuantumBuffer, pos: &mut usize) -> CerealResult<Self> {
        Ok(Self {
            x: buffer.read_f64(pos)?,
           y: buffer.read_f64(pos)?,
           z: buffer.read_f64(pos)?,
           phase: buffer.read_f64(pos)?,
           coherence: buffer.read_f64(pos)?,
        })
    }
}

impl Scribe for QuantumVector3D {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("⟨");
        self.x.scribe(precision, output);
        output.push_str(", ");
        self.y.scribe(precision, output);
        output.push_str(", ");
        self.z.scribe(precision, output);
        output.push_str("⟩[φ=");
        self.phase.scribe(precision, output);
        output.push_str(", c=");
        self.coherence.scribe(precision, output);
        output.push_char(']');
    }
}

/// Quantum memory pool for vectors
#[derive(Debug)]
pub struct VectorMemoryPool {
    vectors: Vec<QuantumVector3D>,
    timestamp: u64,
    coherence: f64,
}

impl VectorMemoryPool {
    pub fn new() -> Self {
        Self {
            vectors: Vec::new(),
            timestamp: 1705371857, // 2025-01-16 02:24:17 UTC
            coherence: 1.0,
        }
    }

    pub fn add_vector(&mut self, vector: QuantumVector3D) {
        self.vectors.push(vector);
        self.coherence *= 0.99999;
    }

    pub fn cerealize(&self) -> CerealResult<QuantumBuffer> {
        let mut buffer = QuantumBuffer::new();
        buffer.write_f64(self.coherence)?;

        // Write vector count
        buffer.data.extend_from_slice(&(self.vectors.len() as u32).to_le_bytes());

        // Write each vector
        for vector in &self.vectors {
            vector.cerealize(&mut buffer)?;
        }

        Ok(buffer)
    }
}

impl Scribe for VectorMemoryPool {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("VectorPool[size=");
        self.vectors.len().scribe(precision, output);
        output.push_str(", coherence=");
        self.coherence.scribe(precision, output);
        output.push_str(", vectors=[");

        let mut first = true;
        for vector in &self.vectors {
            if !first {
                output.push_str(", ");
            }
            vector.scribe(precision, output);
            first = false;
        }

        output.push_char(']');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_buffer() {
        let mut buffer = QuantumBuffer::new();
        assert!(buffer.coherence > QUANTUM_THRESHOLD);
        assert_eq!(&buffer.data[..4], &CEREAL_MAGIC);
    }

    #[test]
    fn test_vector_cerealization() {
        let vector = QuantumVector3D::with_quantum(1.234, -5.678, 9.012, PI/4.0, 0.95);
        let mut buffer = QuantumBuffer::new();

        assert!(vector.cerealize(&mut buffer).is_ok());

        let mut pos = 6; // Skip magic + version
        let decoded = QuantumVector3D::decerealize(&mut buffer, &mut pos).unwrap();

        assert!((vector.x - decoded.x).abs() < PLANCK_LENGTH);
        assert!((vector.y - decoded.y).abs() < PLANCK_LENGTH);
        assert!((vector.z - decoded.z).abs() < PLANCK_LENGTH);
    }

    #[test]
    fn test_memory_pool() {
        let mut pool = VectorMemoryPool::new();
        pool.add_vector(QuantumVector3D::new(1.0, 2.0, 3.0));
        pool.add_vector(QuantumVector3D::new(-1.0, -2.0, -3.0));

        let buffer = pool.cerealize().unwrap();
        assert!(buffer.coherence > QUANTUM_THRESHOLD);

        let mut output = QuantumString::new();
        pool.scribe(ScribePrecision::Standard, &mut output);
        assert!(output.as_str().contains("size=2"));
    }

    #[test]
    fn test_coherence_decay() {
        let mut buffer = QuantumBuffer::new();
        let initial_coherence = buffer.coherence;

        // Perform multiple operations
        for _ in 0..1000 {
            let _ = buffer.write_f64(1.0);
        }

        assert!(buffer.coherence < initial_coherence);
        assert!(buffer.coherence > QUANTUM_THRESHOLD);
    }
}
