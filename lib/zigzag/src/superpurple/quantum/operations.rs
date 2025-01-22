//! Quantum gate operations
//! Created: 2025-01-22 00:35:31
//! Author: isdood

use super::{QuantumState, QuantumOp};
use crate::superpurple::core::SIMDValue;
use std::f64::consts::PI;
use std::arch::x86_64::*;

pub struct HadamardGate;
pub struct PauliXGate;
pub struct PauliYGate;
pub struct PauliZGate;

impl<T: SIMDValue> QuantumOp<T> for HadamardGate {
    fn apply(&self, state: &QuantumState, data: &[T]) -> Vec<T> {
        let factor = T::from(1.0f64 / 2.0f64.sqrt()).unwrap();
        let mut result = Vec::with_capacity(data.len());

        unsafe {
            if is_x86_feature_detected!("avx512f") {
                // AVX-512 implementation
                let factor_vec = _mm512_set1_ps(factor.to_f32().unwrap());
                for chunk in data.chunks(16) {
                    if chunk.len() == 16 {
                        let input = _mm512_loadu_ps(chunk.as_ptr() as *const f32);
                        let output = _mm512_mul_ps(input, factor_vec);
                        let mut buffer = vec![0.0f32; 16];
                        _mm512_storeu_ps(buffer.as_mut_ptr(), output);
                        result.extend(buffer.iter().map(|&x| T::from(x).unwrap()));
                    }
                }
            } else {
                // Fallback implementation
                for &x in data {
                    result.push(x * factor);
                }
            }
        }

        result
    }

    fn is_unitary(&self) -> bool {
        true
    }
}

impl<T: SIMDValue> QuantumOp<T> for PauliXGate {
    fn apply(&self, _state: &QuantumState, data: &[T]) -> Vec<T> {
        let mut result = Vec::with_capacity(data.len());

        unsafe {
            if is_x86_feature_detected!("avx512f") {
                // AVX-512 implementation
                for chunk in data.chunks(16) {
                    if chunk.len() == 16 {
                        let input = _mm512_loadu_ps(chunk.as_ptr() as *const f32);
                        let ones = _mm512_set1_ps(1.0);
                        let output = _mm512_sub_ps(ones, input);
                        let mut buffer = vec![0.0f32; 16];
                        _mm512_storeu_ps(buffer.as_mut_ptr(), output);
                        result.extend(buffer.iter().map(|&x| T::from(x).unwrap()));
                    }
                }
            } else {
                // Fallback implementation
                for &x in data {
                    result.push(T::one() - x);
                }
            }
        }

        result
    }

    fn is_unitary(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hadamard_gate() {
        let gate = HadamardGate;
        let state = QuantumState::new(1.0);
        let data = vec![1.0f32, 0.0];
        let result = gate.apply(&state, &data);

        let expected_val = 1.0 / 2.0f32.sqrt();
        assert!((result[0] - expected_val).abs() < 1e-6);
        assert!((result[1] - expected_val).abs() < 1e-6);
    }

    #[test]
    fn test_pauli_x_gate() {
        let gate = PauliXGate;
        let state = QuantumState::new(1.0);
        let data = vec![1.0f32, 0.0];
        let result = gate.apply(&state, &data);

        assert!((result[0] - 0.0).abs() < 1e-6);
        assert!((result[1] - 1.0).abs() < 1e-6);
    }
}
pub struct CNOTGate;
pub struct ToffoliGate;
pub struct PhaseGate {
    angle: f64,
}

impl PhaseGate {
    pub fn new(angle: f64) -> Self {
        Self { angle }
    }
}

impl<T: SIMDValue> QuantumOp<T> for CNOTGate {
    fn apply(&self, _state: &QuantumState, data: &[T]) -> Vec<T> {
        assert!(data.len() % 2 == 0, "CNOT requires pairs of qubits");
        let mut result = Vec::with_capacity(data.len());

        unsafe {
            if is_x86_feature_detected!("avx512f") {
                for chunk in data.chunks(32) {
                    if chunk.len() == 32 {
                        let control = _mm512_loadu_ps(chunk.as_ptr() as *const f32);
                        let target = _mm512_loadu_ps(chunk[16..].as_ptr() as *const f32);
                        let ones = _mm512_set1_ps(1.0);
                        let mask = _mm512_cmp_ps_mask(control, _mm512_setzero_ps(), _CMP_GT_OS);
                        let flipped = _mm512_mask_sub_ps(target, mask, ones, target);

                        let mut buffer = vec![0.0f32; 32];
                        _mm512_storeu_ps(buffer.as_mut_ptr(), control);
                        _mm512_storeu_ps(buffer[16..].as_mut_ptr(), flipped);
                        result.extend(buffer.iter().map(|&x| T::from(x).unwrap()));
                    }
                }
            } else {
                // Fallback implementation
                for chunk in data.chunks(2) {
                    if chunk.len() == 2 {
                        let control = chunk[0];
                        let target = chunk[1];
                        result.push(control);
                        result.push(if control > T::zero() { T::one() - target } else { target });
                    }
                }
            }
        }

        result
    }

    fn is_unitary(&self) -> bool {
        true
    }
}

impl<T: SIMDValue> QuantumOp<T> for ToffoliGate {
    fn apply(&self, _state: &QuantumState, data: &[T]) -> Vec<T> {
        assert!(data.len() % 3 == 0, "Toffoli requires triplets of qubits");
        let mut result = Vec::with_capacity(data.len());

        unsafe {
            if is_x86_feature_detected!("avx512f") {
                for chunk in data.chunks(48) {
                    if chunk.len() == 48 {
                        let control1 = _mm512_loadu_ps(chunk.as_ptr() as *const f32);
                        let control2 = _mm512_loadu_ps(chunk[16..].as_ptr() as *const f32);
                        let target = _mm512_loadu_ps(chunk[32..].as_ptr() as *const f32);

                        let ones = _mm512_set1_ps(1.0);
                        let zero = _mm512_setzero_ps();
                        let mask1 = _mm512_cmp_ps_mask(control1, zero, _CMP_GT_OS);
                        let mask2 = _mm512_cmp_ps_mask(control2, zero, _CMP_GT_OS);
                        let combined_mask = mask1 & mask2;
                        let flipped = _mm512_mask_sub_ps(target, combined_mask, ones, target);

                        let mut buffer = vec![0.0f32; 48];
                        _mm512_storeu_ps(buffer.as_mut_ptr(), control1);
                        _mm512_storeu_ps(buffer[16..].as_mut_ptr(), control2);
                        _mm512_storeu_ps(buffer[32..].as_mut_ptr(), flipped);
                        result.extend(buffer.iter().map(|&x| T::from(x).unwrap()));
                    }
                }
            } else {
                // Fallback implementation
                for chunk in data.chunks(3) {
                    if chunk.len() == 3 {
                        let control1 = chunk[0];
                        let control2 = chunk[1];
                        let target = chunk[2];
                        result.push(control1);
                        result.push(control2);
                        result.push(if control1 > T::zero() && control2 > T::zero() {
                            T::one() - target
                        } else {
                            target
                        });
                    }
                }
            }
        }

        result
    }

    fn is_unitary(&self) -> bool {
        true
    }
}

impl<T: SIMDValue> QuantumOp<T> for PhaseGate {
    fn apply(&self, _state: &QuantumState, data: &[T]) -> Vec<T> {
        let phase = T::from(self.angle.cos()).unwrap();
        let mut result = Vec::with_capacity(data.len());

        unsafe {
            if is_x86_feature_detected!("avx512f") {
                let phase_vec = _mm512_set1_ps(phase.to_f32().unwrap());
                for chunk in data.chunks(16) {
                    if chunk.len() == 16 {
                        let input = _mm512_loadu_ps(chunk.as_ptr() as *const f32);
                        let output = _mm512_mul_ps(input, phase_vec);
                        let mut buffer = vec![0.0f32; 16];
                        _mm512_storeu_ps(buffer.as_mut_ptr(), output);
                        result.extend(buffer.iter().map(|&x| T::from(x).unwrap()));
                    }
                }
            } else {
                // Fallback implementation
                for &x in data {
                    result.push(x * phase);
                }
            }
        }

        result
    }

    fn is_unitary(&self) -> bool {
        true
    }
}
pub struct SWAPGate;
pub struct ControlledPhaseGate {
    angle: f64,
}
pub struct SqrtNOTGate;
pub struct ControlledHadamardGate;

impl ControlledPhaseGate {
    pub fn new(angle: f64) -> Self {
        Self { angle }
    }
}

impl<T: SIMDValue> QuantumOp<T> for SWAPGate {
    fn apply(&self, _state: &QuantumState, data: &[T]) -> Vec<T> {
        assert!(data.len() % 2 == 0, "SWAP requires pairs of qubits");
        let mut result = Vec::with_capacity(data.len());

        unsafe {
            if is_x86_feature_detected!("avx512f") {
                for chunk in data.chunks(32) {
                    if chunk.len() == 32 {
                        let first = _mm512_loadu_ps(chunk.as_ptr() as *const f32);
                        let second = _mm512_loadu_ps(chunk[16..].as_ptr() as *const f32);

                        let mut buffer = vec![0.0f32; 32];
                        _mm512_storeu_ps(buffer.as_mut_ptr(), second);
                        _mm512_storeu_ps(buffer[16..].as_mut_ptr(), first);
                        result.extend(buffer.iter().map(|&x| T::from(x).unwrap()));
                    }
                }
            } else {
                for chunk in data.chunks(2) {
                    if chunk.len() == 2 {
                        result.push(chunk[1]);
                        result.push(chunk[0]);
                    }
                }
            }
        }

        result
    }

    fn is_unitary(&self) -> bool {
        true
    }
}

impl<T: SIMDValue> QuantumOp<T> for ControlledPhaseGate {
    fn apply(&self, _state: &QuantumState, data: &[T]) -> Vec<T> {
        assert!(data.len() % 2 == 0, "Controlled-Phase requires pairs of qubits");
        let phase = T::from(self.angle.cos()).unwrap();
        let mut result = Vec::with_capacity(data.len());

        unsafe {
            if is_x86_feature_detected!("avx512f") {
                let phase_vec = _mm512_set1_ps(phase.to_f32().unwrap());
                for chunk in data.chunks(32) {
                    if chunk.len() == 32 {
                        let control = _mm512_loadu_ps(chunk.as_ptr() as *const f32);
                        let target = _mm512_loadu_ps(chunk[16..].as_ptr() as *const f32);

                        let mask = _mm512_cmp_ps_mask(control, _mm512_setzero_ps(), _CMP_GT_OS);
                        let phased = _mm512_mask_mul_ps(target, mask, target, phase_vec);

                        let mut buffer = vec![0.0f32; 32];
                        _mm512_storeu_ps(buffer.as_mut_ptr(), control);
                        _mm512_storeu_ps(buffer[16..].as_mut_ptr(), phased);
                        result.extend(buffer.iter().map(|&x| T::from(x).unwrap()));
                    }
                }
            } else {
                for chunk in data.chunks(2) {
                    if chunk.len() == 2 {
                        result.push(chunk[0]);
                        result.push(if chunk[0] > T::zero() {
                            chunk[1] * phase
                        } else {
                            chunk[1]
                        });
                    }
                }
            }
        }

        result
    }

    fn is_unitary(&self) -> bool {
        true
    }
}

impl<T: SIMDValue> QuantumOp<T> for SqrtNOTGate {
    fn apply(&self, _state: &QuantumState, data: &[T]) -> Vec<T> {
        let factor = T::from(0.5).unwrap();
        let mut result = Vec::with_capacity(data.len());

        unsafe {
            if is_x86_feature_detected!("avx512f") {
                let factor_vec = _mm512_set1_ps(factor.to_f32().unwrap());
                let one_vec = _mm512_set1_ps(1.0);

                for chunk in data.chunks(16) {
                    if chunk.len() == 16 {
                        let input = _mm512_loadu_ps(chunk.as_ptr() as *const f32);
                        let complement = _mm512_sub_ps(one_vec, input);
                        let output = _mm512_add_ps(
                            input,
                            _mm512_mul_ps(complement, factor_vec)
                        );

                        let mut buffer = vec![0.0f32; 16];
                        _mm512_storeu_ps(buffer.as_mut_ptr(), output);
                        result.extend(buffer.iter().map(|&x| T::from(x).unwrap()));
                    }
                }
            } else {
                for &x in data {
                    result.push(x + (T::one() - x) * factor);
                }
            }
        }

        result
    }

    fn is_unitary(&self) -> bool {
        true
    }
}

impl<T: SIMDValue> QuantumOp<T> for ControlledHadamardGate {
    fn apply(&self, _state: &QuantumState, data: &[T]) -> Vec<T> {
        assert!(data.len() % 2 == 0, "Controlled-Hadamard requires pairs of qubits");
        let factor = T::from(1.0f64 / 2.0f64.sqrt()).unwrap();
        let mut result = Vec::with_capacity(data.len());

        unsafe {
            if is_x86_feature_detected!("avx512f") {
                let factor_vec = _mm512_set1_ps(factor.to_f32().unwrap());

                for chunk in data.chunks(32) {
                    if chunk.len() == 32 {
                        let control = _mm512_loadu_ps(chunk.as_ptr() as *const f32);
                        let target = _mm512_loadu_ps(chunk[16..].as_ptr() as *const f32);

                        let mask = _mm512_cmp_ps_mask(control, _mm512_setzero_ps(), _CMP_GT_OS);
                        let hadamard = _mm512_mul_ps(target, factor_vec);
                        let transformed = _mm512_mask_blend_ps(mask, target, hadamard);

                        let mut buffer = vec![0.0f32; 32];
                        _mm512_storeu_ps(buffer.as_mut_ptr(), control);
                        _mm512_storeu_ps(buffer[16..].as_mut_ptr(), transformed);
                        result.extend(buffer.iter().map(|&x| T::from(x).unwrap()));
                    }
                }
            } else {
                for chunk in data.chunks(2) {
                    if chunk.len() == 2 {
                        result.push(chunk[0]);
                        result.push(if chunk[0] > T::zero() {
                            chunk[1] * factor
                        } else {
                            chunk[1]
                        });
                    }
                }
            }
        }

        result
    }

    fn is_unitary(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_gate() {
        let gate = SWAPGate;
        let state = QuantumState::new(1.0);
        let data = vec![1.0f32, 0.0];
        let result = gate.apply(&state, &data);
        assert_eq!(result, vec![0.0, 1.0]);
    }

    #[test]
    fn test_controlled_phase_gate() {
        let gate = ControlledPhaseGate::new(std::f64::consts::PI / 2.0);
        let state = QuantumState::new(1.0);
        let data = vec![1.0f32, 1.0];
        let result = gate.apply(&state, &data);
        assert!((result[0] - 1.0).abs() < 1e-6);
        assert!((result[1] - 0.0).abs() < 1e-6);
    }
}
