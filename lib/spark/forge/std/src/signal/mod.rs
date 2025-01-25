//! Native IO signal type with crystal resonance.
//!
//! This module provides a high-performance IO signal type optimized for
//! crystal-space quantum tunneling and signal propagation.

pub mod pulse;
pub mod crystal;

use std::io::{self, Read, Write};
use std::ops::{BitAnd, BitOr, BitXor, Not};
use pulse::Pulse;
use crystal::Tunnel;

/// Result type for signal operations
pub type SignalResult<T> = Result<T, SignalError>;

/// Error type for signal operations
#[derive(Debug)]
pub enum SignalError {
    /// IO error
    Io(io::Error),
    /// Signal interference
    Interference(String),
    /// Quantum decoherence
    Decoherence(String),
    /// Crystal misalignment
    Misalignment(String),
}

impl From<io::Error> for SignalError {
    fn from(error: io::Error) -> Self {
        SignalError::Io(error)
    }
}

impl From<String> for SignalError {
    fn from(error: String) -> Self {
        SignalError::Misalignment(error)
    }
}

/// Native IO signal with crystal resonance
#[derive(Debug, Clone)]
pub struct Signal {
    /// Signal buffer
    buffer: Vec<u8>,
    /// Quantum pulse state
    pulse: Pulse,
    /// Crystal tunnel
    tunnel: Tunnel,
}

impl Signal {
    /// Creates a new signal
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            pulse: Pulse::default(),
            tunnel: Tunnel::default(),
        }
    }

    /// Creates a signal from bytes
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            buffer: bytes.to_vec(),
            pulse: Pulse::from_bytes(bytes),
            tunnel: Tunnel::from_bytes(bytes),
        }
    }

    /// Gets the signal buffer
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    /// Gets the quantum pulse
    pub fn pulse(&self) -> &Pulse {
        &self.pulse
    }

    /// Gets the crystal tunnel
    pub fn tunnel(&self) -> &Tunnel {
        &self.tunnel
    }

    /// Reads from a source with quantum tunneling
    pub fn read_tunneled<R: Read>(&mut self, reader: &mut R) -> SignalResult<usize> {
        let mut buf = [0u8; 1024];
        let n = reader.read(&mut buf)?;

        self.buffer.extend_from_slice(&buf[..n]);
        self.pulse.update(&buf[..n]);
        self.tunnel.tunnel(&buf[..n]);

        Ok(n)
    }

    /// Writes to a sink with quantum tunneling
    pub fn write_tunneled<W: Write>(&self, writer: &mut W) -> SignalResult<usize> {
        let n = writer.write(&self.buffer)?;
        self.tunnel.verify_transmission(&self.buffer[..n])?;
        Ok(n)
    }

    /// Resonates with another signal
    pub fn resonate(&mut self, other: &Signal) -> SignalResult<()> {
        self.pulse.resonate(&other.pulse);
        self.tunnel.synchronize(&other.tunnel)?;

        // Combine buffers with quantum interference
        let combined: Vec<u8> = self.buffer
            .iter()
            .zip(other.buffer.iter().cycle())
            .map(|(&a, &b)| a ^ b)
            .collect();

        self.buffer = combined;
        Ok(())
    }

    /// Amplifies the signal
    pub fn amplify(&mut self, gain: f64) -> SignalResult<()> {
        self.pulse.amplify(gain);
        self.tunnel.amplify(gain)?;

        // Apply quantum amplification to buffer
        for byte in &mut self.buffer {
            *byte = (*byte as f64 * gain).min(255.0) as u8;
        }

        Ok(())
    }
}

impl Default for Signal {
    fn default() -> Self {
        Self::new()
    }
}

impl Read for Signal {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = std::cmp::min(buf.len(), self.buffer.len());
        buf[..n].copy_from_slice(&self.buffer[..n]);
        self.buffer.drain(..n);
        Ok(n)
    }
}

impl Write for Signal {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buffer.clear();
        Ok(())
    }
}

impl BitAnd for Signal {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let buffer: Vec<u8> = self.buffer
            .iter()
            .zip(rhs.buffer.iter().cycle())
            .map(|(&a, &b)| a & b)
            .collect();

        Self {
            buffer,
            pulse: self.pulse & rhs.pulse,
            tunnel: self.tunnel & rhs.tunnel,
        }
    }
}

impl BitOr for Signal {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let buffer: Vec<u8> = self.buffer
            .iter()
            .zip(rhs.buffer.iter().cycle())
            .map(|(&a, &b)| a | b)
            .collect();

        Self {
            buffer,
            pulse: self.pulse | rhs.pulse,
            tunnel: self.tunnel | rhs.tunnel,
        }
    }
}

impl BitXor for Signal {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let buffer: Vec<u8> = self.buffer
            .iter()
            .zip(rhs.buffer.iter().cycle())
            .map(|(&a, &b)| a ^ b)
            .collect();

        Self {
            buffer,
            pulse: self.pulse ^ rhs.pulse,
            tunnel: self.tunnel ^ rhs.tunnel,
        }
    }
}

impl Not for Signal {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self {
            buffer: self.buffer.iter().map(|&b| !b).collect(),
            pulse: !self.pulse,
            tunnel: !self.tunnel,
        }
    }
}
