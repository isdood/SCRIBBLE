//! Weave module for DNA-inspired thread distribution
//! Implements Z-X & Q-W base pairing for optimal threading

use crate::waves::WaveError;

/// Thread distribution factor (1-1000)
#[derive(Debug, Clone, Copy)]
pub struct WeavePattern {
    /// Distribution factor
    factor: u16,
    /// Z-X pair ratio
    zx_ratio: f64,
    /// Q-W pair ratio
    qw_ratio: f64,
}

/// DNA-inspired base pair for thread optimization
#[derive(Debug)]
pub enum BasePair {
    /// Z-X pair (compute intensive)
    ZX(f64),
    /// Q-W pair (memory intensive)
    QW(f64),
}

impl WeavePattern {
    /// Creates a new weave pattern with the specified distribution factor
    pub fn new(factor: u16) -> Result<Self, WaveError> {
        if !(1..=1000).contains(&factor) {
            return Err(WaveError::ResonanceError("Weave factor must be between 1 and 1000".into()));
        }

        // Calculate optimal ratios based on factor
        let zx_ratio = (factor as f64 / 1000.0).sqrt();
        let qw_ratio = 1.0 - zx_ratio;

        Ok(Self {
            factor,
            zx_ratio,
            qw_ratio,
        })
    }

    /// Creates thread distribution based on weave pattern
    pub fn distribute(&self) -> Vec<BasePair> {
        let thread_count = num_cpus::get();
        let mut pairs = Vec::with_capacity(thread_count);

        for i in 0..thread_count {
            let ratio = i as f64 / thread_count as f64;
            if ratio < self.zx_ratio {
                pairs.push(BasePair::ZX(self.zx_ratio));
            } else {
                pairs.push(BasePair::QW(self.qw_ratio));
            }
        }

        pairs
    }

    /// Optimizes thread distribution for resonance
    pub fn optimize_resonance(&self) -> Result<Vec<BasePair>, WaveError> {
        let pairs = self.distribute();
        let total_pairs = pairs.len();

        // Ensure even distribution
        if total_pairs % 2 != 0 {
            return Err(WaveError::ResonanceError("Uneven thread distribution".into()));
        }

        Ok(pairs)
    }
}

/// Parser for weave declarations in .spk files
pub struct WeaveParser;

impl WeaveParser {
    /// Parses weave declaration from string
    pub fn parse(input: &str) -> Option<u16> {
        let weave_pattern = input.trim();
        if !weave_pattern.starts_with("~weave~") {
            return None;
        }

        weave_pattern
            .split('=')
            .nth(1)
            .and_then(|s| s.trim().parse().ok())
            .filter(|&n| (1..=1000).contains(&n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weave_pattern_creation() {
        assert!(WeavePattern::new(0).is_err());
        assert!(WeavePattern::new(1001).is_err());
        assert!(WeavePattern::new(500).is_ok());
    }

    #[test]
    fn test_weave_distribution() {
        let pattern = WeavePattern::new(500).unwrap();
        let pairs = pattern.distribute();
        assert!(!pairs.is_empty());
    }

    #[test]
    fn test_weave_parser() {
        assert_eq!(WeaveParser::parse("~weave~ = 500"), Some(500));
        assert_eq!(WeaveParser::parse("~weave~=1"), Some(1));
        assert_eq!(WeaveParser::parse("~weave~=1001"), None);
        assert_eq!(WeaveParser::parse("invalid"), None);
    }
}

pub mod integration;
