use super::{QuantumState, QuantumOp};
use crate::core::SIMDValue;

#[derive(Debug, Clone)]
pub struct HadamardGate;

#[derive(Debug, Clone)]
pub struct CNOTGate;

#[derive(Debug, Clone)]
pub struct SWAPGate;

#[derive(Debug, Clone)]
pub struct ControlledPhaseGate {
    pub angle: f64,
}

#[derive(Debug, Clone)]
pub struct SqrtNOTGate;

impl ControlledPhaseGate {
    #[inline]
    pub fn new(angle: f64) -> Self {
        Self { angle }
    }
}

impl<T: SIMDValue> QuantumOp<T> for HadamardGate {
    #[inline]
    fn apply(&self, _state: &QuantumState, data: &[T]) -> Vec<T> {
        let factor = T::from(1.0f64 / 2.0f64.sqrt()).unwrap();
        data.iter().map(|&x| x * factor).collect()
    }

    fn is_unitary(&self) -> bool {
        true
    }
}

impl<T: SIMDValue> QuantumOp<T> for CNOTGate {
    #[inline]
    fn apply(&self, _state: &QuantumState, data: &[T]) -> Vec<T> {
        assert!(data.len() % 2 == 0, "CNOT requires pairs of qubits");
        let mut result = Vec::with_capacity(data.len());

        for chunk in data.chunks(2) {
            let control = chunk[0];
            let target = chunk[1];
            result.push(control);
            result.push(if control > T::zero() { T::one() - target } else { target });
        }

        result
    }

    fn is_unitary(&self) -> bool {
        true
    }
}

impl<T: SIMDValue> QuantumOp<T> for SWAPGate {
    #[inline]
    fn apply(&self, _state: &QuantumState, data: &[T]) -> Vec<T> {
        assert!(data.len() % 2 == 0, "SWAP requires pairs of qubits");
        let mut result = Vec::with_capacity(data.len());

        for chunk in data.chunks(2) {
            result.push(chunk[1]);
            result.push(chunk[0]);
        }

        result
    }

    fn is_unitary(&self) -> bool {
        true
    }
}

impl<T: SIMDValue> QuantumOp<T> for ControlledPhaseGate {
    #[inline]
    fn apply(&self, _state: &QuantumState, data: &[T]) -> Vec<T> {
        assert!(data.len() % 2 == 0, "Controlled-Phase requires pairs of qubits");
        let phase = T::from(self.angle.cos()).unwrap();
        let mut result = Vec::with_capacity(data.len());

        for chunk in data.chunks(2) {
            let control = chunk[0];
            let target = chunk[1];
            result.push(control);
            result.push(if control > T::zero() { target * phase } else { target });
        }

        result
    }

    fn is_unitary(&self) -> bool {
        true
    }
}

impl<T: SIMDValue> QuantumOp<T> for SqrtNOTGate {
    #[inline]
    fn apply(&self, _state: &QuantumState, data: &[T]) -> Vec<T> {
        let factor = T::from(0.5f64).unwrap();
        data.iter().map(|&x| x + (T::one() - x) * factor).collect()
    }

    fn is_unitary(&self) -> bool {
        true
    }
}
