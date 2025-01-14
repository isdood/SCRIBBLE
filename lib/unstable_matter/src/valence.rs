// lib/unstable_matter/src/valence.rs
// Last Updated: 2025-01-14 01:08:46 UTC

use core::ops::{Add, Mul};
use crate::vector::Vector3D;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValenceOrder {
    Superposition,
    Entangled,
    Orthogonal,
    Parallel,
    Equal,
    Less,
    Greater,
}

pub fn compare_vectors<T>(a: &Vector3D<T>, b: &Vector3D<T>) -> ValenceOrder
where
T: PartialEq + PartialOrd + Copy + Add<Output = T> + Mul<Output = T> + Default
{
    if a == b {
        return ValenceOrder::Equal;
    }

    if is_parallel(a, b) {
        ValenceOrder::Parallel
    } else if is_orthogonal(a, b) {
        ValenceOrder::Orthogonal
    } else {
        let mag_a = squared_magnitude(a);
        let mag_b = squared_magnitude(b);
        if mag_a < mag_b {
            ValenceOrder::Less
        } else {
            ValenceOrder::Greater
        }
    }
}

fn squared_magnitude<T>(vec: &Vector3D<T>) -> T
where
T: PartialEq + Copy + Add<Output = T> + Mul<Output = T> + Default
{
    vec.x * vec.x + vec.y * vec.y + vec.z * vec.z
}

fn is_parallel<T>(vec1: &Vector3D<T>, vec2: &Vector3D<T>) -> bool
where
T: PartialEq + Copy + Add<Output = T> + Mul<Output = T> + Default
{
    let dot_product = vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z;
    let mag1 = squared_magnitude(vec1);
    let mag2 = squared_magnitude(vec2);
    dot_product * dot_product == mag1 * mag2
}

fn is_orthogonal<T>(vec1: &Vector3D<T>, vec2: &Vector3D<T>) -> bool
where
T: PartialEq + Copy + Add<Output = T> + Mul<Output = T> + Default
{
    let dot_product = vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z;
    dot_product == T::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_comparison() {
        let v1 = Vector3D::new(1.0, 0.0, 0.0);
        let v2 = Vector3D::new(1.0, 0.0, 0.0);
        let v3 = Vector3D::new(2.0, 0.0, 0.0);

        assert_eq!(compare_vectors(&v1, &v2), ValenceOrder::Equal);
        assert_eq!(compare_vectors(&v1, &v3), ValenceOrder::Less);
    }

    #[test]
    fn test_quantum_comparison() {
        let v1 = Helium::new(Vector3D::new(1.0, 0.0, 0.0));
        let v2 = Helium::new(Vector3D::new(1.0, 0.0, 0.0));

        // First comparison should show entanglement due to high coherence
        assert_eq!(compare_helium_vectors(&v1, &v2), ValenceOrder::Entangled);

        // Multiple observations should reduce coherence and lead to superposition
        let _ = v1.quantum_load(core::sync::atomic::Ordering::SeqCst);
        let _ = v1.quantum_load(core::sync::atomic::Ordering::SeqCst);
        assert_eq!(compare_helium_vectors(&v1, &v2), ValenceOrder::Superposition);
    }
}
