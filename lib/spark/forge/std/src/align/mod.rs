//! Alignment utilities for crystal-space operations.

pub mod space;

use std::fmt;

/// Represents memory alignment requirements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    /// 16-byte crystal alignment
    Crystal16,
    /// 32-byte vector alignment
    Vector32,
    /// 64-byte vector alignment
    Vector64,
    /// 128-byte parallel alignment
    Parallel128,
    /// 256-byte parallel alignment
    Parallel256,
    /// Custom alignment in bytes (must be power of 2)
    Custom(usize),
}

impl Alignment {
    /// Returns the alignment in bytes
    pub fn as_bytes(self) -> usize {
        match self {
            Self::Crystal16 => 16,
            Self::Vector32 => 32,
            Self::Vector64 => 64,
            Self::Parallel128 => 128,
            Self::Parallel256 => 256,
            Self::Custom(bytes) => bytes,
        }
    }

    /// Creates an Alignment from the specified number of bytes
    pub fn from_bytes(bytes: usize) -> Self {
        if !bytes.is_power_of_two() {
            return Self::Crystal16;
        }
        match bytes {
            16 => Self::Crystal16,
            32 => Self::Vector32,
            64 => Self::Vector64,
            128 => Self::Parallel128,
            256 => Self::Parallel256,
            _ => Self::Custom(bytes),
        }
    }
}

impl fmt::Display for Alignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Crystal16 => write!(f, "16-byte crystal alignment"),
            Self::Vector32 => write!(f, "32-byte vector alignment"),
            Self::Vector64 => write!(f, "64-byte vector alignment"),
            Self::Parallel128 => write!(f, "128-byte parallel alignment"),
            Self::Parallel256 => write!(f, "256-byte parallel alignment"),
            Self::Custom(bytes) => write!(f, "{}-byte custom alignment", bytes),
        }
    }
}
