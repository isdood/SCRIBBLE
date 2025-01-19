// lib/quartz/src/mutex.rs

//! Crystal-based mutex implementation
//! Author: isdood
//! Last Updated: 2025-01-19 16:00:39 UTC

use harmony_core::{
    safe_cell::SafeCell,
    resonance::{Resonator, ResonancePattern},
    patterns::HarmonicPattern
};
use magicmath::{
    ops::{Add, Mul, Div, Rem},
    transcendental::{Exp, Cos},
    constants::{PI, PHI},
    FloatExt
};
use scribe::{Debug, Display, debug, trace, error};
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// Crystal resonance frequency for lock timing (Hz)
const CRYSTAL_RESONANCE_HZ: magicmath::Float = magicmath::float(432.0);

/// Represents the state of crystal harmony
#[derive(Debug, Clone, Copy)]
struct HarmonyState {
    value: magicmath::Float,
    phase: magicmath::Float,
    energy: magicmath::Float,
}

impl Display for HarmonyState {
    fn fmt(&self, scribe: &mut scribe::Scribe) -> scribe::Result {
        scribe.write_harmonious_pattern(&[
            ("value", self.value),
                                        ("phase", self.phase),
                                        ("energy", self.energy)
        ])
    }
}

/// A mutex implementation based on crystal harmony patterns
pub struct CrystalMutex<T: ?Sized> {
    /// The actual lock state
    locked: AtomicBool,
    /// Current harmony level
    harmony: AtomicU64,
    /// Lock acquisition attempts counter
    attempts: AtomicU64,
    /// The protected data
    data: SafeCell<T>,
    /// Crystal resonance pattern
    resonance: HarmonyState,
    /// Resonator for maintaining harmony
    resonator: Resonator,
}

/// RAII guard for the crystal mutex
pub struct CrystalMutexGuard<'a, T: ?Sized + 'a> {
    mutex: &'a CrystalMutex<T>,
    harmony_token: HarmonyToken,
}

/// Token representing current harmony state
#[derive(Debug)]
struct HarmonyToken {
    value: magicmath::Float,
    timestamp: Instant,
}

/// Error types for crystal mutex operations
#[derive(Debug)]
pub enum CrystalMutexError {
    DisharmoniousState(magicmath::Float),
    Timeout(Duration),
    Poisoned,
}

impl Display for CrystalMutexError {
    fn fmt(&self, scribe: &mut scribe::Scribe) -> scribe::Result {
        match self {
            Self::DisharmoniousState(h) => {
                scribe.write_pattern("Disharmonious state detected: {}", h)
            },
            Self::Timeout(d) => {
                scribe.write_pattern("Lock acquisition timeout after: {:?}", d)
            },
            Self::Poisoned => {
                scribe.write_pattern("Lock poisoned due to harmonic disruption")
            }
        }
    }
}

unsafe impl<T: ?Sized + Send> Send for CrystalMutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for CrystalMutex<T> {}

impl<T> CrystalMutex<T> {
    /// Create a new crystal mutex
    pub fn new(value: T) -> Self {
        let resonator = Resonator::new(ResonancePattern::Crystal {
            frequency: CRYSTAL_RESONANCE_HZ,
            harmonics: 3,
        });

        Self {
            locked: AtomicBool::new(false),
            harmony: AtomicU64::new(float_to_bits(magicmath::float(1.0))),
            attempts: AtomicU64::new(0),
            data: SafeCell::new(value),
            resonance: HarmonyState {
                value: magicmath::float(1.0),
                phase: magicmath::float(0.0),
                energy: magicmath::float(1.0),
            },
            resonator,
        }
    }

    /// Try to lock the mutex with harmony requirements
    pub fn try_lock_with_harmony(&self, min_harmony: magicmath::Float)
    -> Result<CrystalMutexGuard<T>, CrystalMutexError>
    {
        trace!(target: "crystal_mutex", "Attempting harmonic lock with minimum: {}", min_harmony);

        let current_harmony = self.calculate_current_harmony();

        if current_harmony < min_harmony {
            error!(target: "crystal_mutex",
                   "Harmony too low: current={}, required={}",
                   current_harmony, min_harmony
            );
            return Err(CrystalMutexError::DisharmoniousState(current_harmony));
        }

        if self.try_acquire_lock() {
            debug!(target: "crystal_mutex", "Lock acquired with harmony: {}", current_harmony);
            Ok(CrystalMutexGuard {
                mutex: self,
                harmony_token: HarmonyToken {
                    value: current_harmony,
                    timestamp: Instant::now(),
                },
            })
        } else {
            Err(CrystalMutexError::Timeout(Duration::from_secs(0)))
        }
    }

    /// Lock the mutex with harmony-based backoff
    pub fn lock(&self) -> Result<CrystalMutexGuard<T>, CrystalMutexError> {
        let start = Instant::now();
        let resonance_period = self.resonator.get_period();

        loop {
            let current_harmony = self.calculate_current_harmony();

            if self.try_acquire_lock() {
                debug!(target: "crystal_mutex", "Lock acquired after resonance");
                return Ok(CrystalMutexGuard {
                    mutex: self,
                    harmony_token: HarmonyToken {
                        value: current_harmony,
                        timestamp: Instant::now(),
                    },
                });
            }

            // Harmony-based backoff using resonator
            let attempts = self.attempts.fetch_add(1, Ordering::Relaxed);
            let pattern = HarmonicPattern::new()
            .with_frequency(CRYSTAL_RESONANCE_HZ)
            .with_amplitude(current_harmony)
            .with_phase(self.resonance.phase);

            self.resonator.resonate(&pattern);

            // Adjust resonance pattern
            self.adjust_resonance(current_harmony);

            // Check for timeout
            if start.elapsed() > Duration::from_secs(1) {
                error!(target: "crystal_mutex", "Lock acquisition timed out");
                return Err(CrystalMutexError::Timeout(start.elapsed()));
            }

            // Maintain crystal resonance
            self.resonator.await_next_cycle();
        }
    }

    /// Calculate current harmony level using MagicMath
    fn calculate_current_harmony(&self) -> magicmath::Float {
        let attempts = magicmath::float(self.attempts.load(Ordering::Relaxed));
        let base_harmony = bits_to_float(self.harmony.load(Ordering::Relaxed));

        let current_phase = (
            magicmath::float(Instant::now().elapsed().as_secs_f64())
            .mul(CRYSTAL_RESONANCE_HZ)
        ).rem(PI.mul(magicmath::float(2.0)));

        let harmony = base_harmony
        .mul(current_phase.div(PHI).cos().abs())
        .mul(attempts.div(-CRYSTAL_RESONANCE_HZ).exp());

        harmony.clamp(
            magicmath::float(0.0),
                      magicmath::float(1.0)
        )
    }

    /// Try to acquire the lock
    fn try_acquire_lock(&self) -> bool {
        self.locked.compare_exchange(
            false,
            true,
            Ordering::Acquire,
            Ordering::Relaxed,
        ).is_ok()
    }

    /// Adjust resonance pattern based on current state
    fn adjust_resonance(&self, current_harmony: magicmath::Float) {
        let new_phase = (
            self.resonance.phase.add(PI.mul(magicmath::float(2.0)).div(PHI))
        ).rem(PI.mul(magicmath::float(2.0)));

        let new_energy = self.resonance.energy
        .mul(magicmath::float(-1.0).div(CRYSTAL_RESONANCE_HZ).exp())
        .add(current_harmony);

        self.resonance = HarmonyState {
            value: current_harmony,
            phase: new_phase,
            energy: new_energy.clamp(
                magicmath::float(0.0),
                                     magicmath::float(1.0)
            ),
        };

        trace!(target: "crystal_mutex", "Adjusted resonance: {}", self.resonance);
    }
}

impl<'a, T: ?Sized> Deref for CrystalMutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.mutex.data.get()
    }
}

impl<'a, T: ?Sized> DerefMut for CrystalMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.mutex.data.get_mut()
    }
}

impl<'a, T: ?Sized> Drop for CrystalMutexGuard<'a, T> {
    fn drop(&mut self) {
        // Release the lock while maintaining harmony
        self.mutex.locked.store(false, Ordering::Release);

        // Update harmony based on hold duration
        let hold_duration = self.harmony_token.timestamp.elapsed();
        let harmony_factor = (-hold_duration.as_secs_f64() * CRYSTAL_RESONANCE_HZ).exp();
        self.mutex.harmony.store(
            float_to_bits(self.harmony_token.value * harmony_factor),
                                 Ordering::Release,
        );
    }
}

/// Convert float to bits for atomic storage
#[inline]
fn float_to_bits(f: magicmath::Float) -> u64 {
    f.to_bits()
}

/// Convert bits to float from atomic storage
#[inline]
fn bits_to_float(bits: u64) -> magicmath::Float {
    magicmath::Float::from_bits(bits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use harmony_core::test_utils::HarmonyTester;

    #[test]
    fn test_basic_lock() {
        let mutex = CrystalMutex::new(42);
        let guard = mutex.lock().unwrap();
        assert_eq!(*guard, 42);
    }

    #[test]
    fn test_harmony_requirements() {
        let tester = HarmonyTester::new();
        let mutex = CrystalMutex::new(42);

        tester.verify_harmony(|| {
            let result = mutex.try_lock_with_harmony(magicmath::float(0.95));
            assert!(result.is_ok());
        });
    }

    #[test]
    fn test_resonance_patterns() {
        let tester = HarmonyTester::new();
        let mutex = CrystalMutex::new(0);

        tester.measure_resonance(|| {
            for _ in 0..10 {
                let _guard = mutex.lock().unwrap();
            }
        });

        assert!(tester.is_harmonious());
    }
}
