/// Protected Trait Definition
/// Last Updated: 2025-01-15 01:59:16 UTC
/// Author: isdood
/// Current User: isdood

use crate::helium::HeliumOrdering;

pub trait Protected {
    fn protect(&self) -> Result<(), &'static str>;
    fn unprotect(&self) -> Result<(), &'static str>;
    fn get_coherence(&self) -> f64;
    fn is_quantum_stable(&self) -> bool;
    fn is_protected(&self) -> bool;
}
