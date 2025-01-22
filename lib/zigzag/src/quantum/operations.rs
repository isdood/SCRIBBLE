//! Quantum gate implementations

use super::{QuantumState, QuantumOp};
use crate::core::SIMDValue;
use std::f64::consts::PI;
use std::arch::x86_64::*;

pub struct HadamardGate;
pub struct CNOTGate;
pub struct SWAPGate;
pub struct ControlledPhaseGate {
    angle: f64,
}

// Implementation of HadamardGate
impl<T: SIMDValue> QuantumOp<T> for HadamardGate {
    fn apply(&self, _state: &QuantumState, data: &[T]) -> Vec<T> {
        let factor = T::from(1.0f64 / 2.0f64.sqrt()).unwrap();
        let mut result = Vec::with_capacity(data.len());

        unsafe {
            if is_x86_feature_detected!("avx512f") {
                for chunk in data.chunks(16) {
                    if chunk.len() == 16 {
                        let input = _mm512_loadu_ps(chunk.as_ptr() as *const f32);
                        let factor_vec = _mm512_set1_ps(factor.to_f32().unwrap());
                        let output = _mm512_mul_ps(input, factor_vec);
                        let mut buffer = vec![0.0f32; 16];
                        _mm512_storeu_ps(buffer.as_mut_ptr(), output);
                        result.extend(buffer.iter().map(|&x| T::from(x).unwrap()));
                    }
                }
            } else {
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

// Rest of the implementations follow...
