// lib/magicmath/src/exp.rs

//! Exponential function implementation for Crystal Lattice HPC Systems
//! =======================================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 16:22:10 UTC
//! Version: 0.1.0
//! License: MIT

/// Calculates the exponential function e^x using a series expansion (Taylor series).
///
/// # Arguments
///
/// * `x` - The exponent to which e is raised.
///
/// # Returns
///
/// * The calculated value of e^x.
pub fn exp(x: f64) -> f64 {
    const TERMS: usize = 20; // Number of terms in the series expansion
    let mut result = 1.0;
    let mut term = 1.0;

    for n in 1..TERMS {
        term *= x / n as f64;
        result += term;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exp() {
        let tolerance = 1e-10;

        // Test known values
        assert!((exp(0.0) - 1.0).abs() < tolerance);
        assert!((exp(1.0) - std::f64::consts::E).abs() < tolerance);
        assert!((exp(-1.0) - 1.0 / std::f64::consts::E).abs() < tolerance);

        // Test larger values
        assert!((exp(2.0) - 7.38905609893065).abs() < tolerance);
        assert!((exp(-2.0) - 0.1353352832366127).abs() < tolerance);
    }
}
