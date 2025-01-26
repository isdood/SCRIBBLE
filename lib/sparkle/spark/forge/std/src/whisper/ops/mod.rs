//! Operations module for Whisper type

use super::{Whisper, WhisperResult, WhisperError};

/// Trait for Whisper-specific operations
pub trait WhisperOps {
    /// Performs crystal resonance
    fn resonate(&self, other: &Self) -> WhisperResult<Whisper>;
    /// Performs quantum phase shift
    fn phase_shift(&self, phase: f64) -> WhisperResult<Whisper>;
}

impl WhisperOps for Whisper {
    fn resonate(&self, other: &Self) -> WhisperResult<Whisper> {
        let phase = self.phase().combine(other.phase());
        let state = self.state().resonate(other.state());

        Ok(Whisper {
            value: self.get(),
            phase,
            state,
        })
    }

    fn phase_shift(&self, phase: f64) -> WhisperResult<Whisper> {
        if !(-std::f64::consts::PI..=std::f64::consts::PI).contains(&phase) {
            return Err(WhisperError::QuantumError("Phase out of range".into()));
        }

        Ok(Whisper {
            value: self.get(),
            phase: self.phase().shift(phase),
            state: *self.state(),
        })
    }
}
