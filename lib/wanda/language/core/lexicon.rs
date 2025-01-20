//! Crystal-Harmonized Lexicon Management System
//! Last Updated: 2025-01-20 01:40:06 UTC
//! Current User: isdood
//!
//! A specialized lexicon system utilizing crystal resonance for:
//! - Language pattern crystallization
//! - Aetheric meaning preservation
//! - Harmonic semantic relationships
//! - Dream-state language processing

use harmony_core::hashbrown::{BrownTable, BrownEntry, HarmonicState, CrystalPattern};
use unstable_matter::{
    grav::GravitationalConstants,
    sunrise::Sunrise,
};
use magicmath::vector3d::Vector3D;
use scribe::{Scribe, ScribePrecision, CrystalString};
use crate::aether::Aether;

/// Crystal-resonant lexical entry
pub struct LexiconEntry {
    embedding: CrystalEmbedding,
    harmonic_state: HarmonicState,
    crystal_pattern: CrystalPattern,
    last_accessed: Aether<u64>,
}

impl Scribe for LexiconEntry {
    fn scribe(&self, precision: ScribePrecision, output: &mut CrystalString) {
        output.push_str("LexiconEntry{");
        output.push_str("embedding=");
        self.embedding.scribe(precision, output);
        output.push_str(", harmonic_state=");
        self.harmonic_state.scribe(precision, output);
        output.push_str(", crystal_pattern=");
        self.crystal_pattern.scribe(precision, output);
        output.push_str(", last_accessed=");
        output.push_f64(
            self.last_accessed.get_state().unwrap() as f64,
                        precision.decimal_places()
        );
        output.push_char('}');
    }
}

/// Crystal-attuned lexicon using harmonized BrownTable
pub struct Lexicon {
    vocabulary: BrownTable<LexiconEntry>,
    resonance: Aether<f64>,
    resonance_threshold: f64,
    grav_constants: GravitationalConstants,
}

impl Lexicon {
    pub fn new(initial_capacity: usize) -> Self {
        Self {
            vocabulary: BrownTable::new(initial_capacity),
            resonance: Aether::new(1.0),
            resonance_threshold: 0.87, // CRYSTAL_RESONANCE_THRESHOLD
            grav_constants: GravitationalConstants::new(),
        }
    }

    /// Add or update a word with crystal-resonant embedding
    pub fn learn_word(&mut self, word: &str, embedding: CrystalEmbedding) -> Result<(), LexiconError> {
        if !self.check_resonance() {
            self.harmonize_resonance()?;
        }

        // Convert word to aetheric coordinates
        let coords = self.word_to_aetheric_coords(word);

        let entry = LexiconEntry {
            embedding,
            harmonic_state: HarmonicState::new(),
            crystal_pattern: CrystalPattern::new(),
            last_accessed: Aether::new(
                std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
            ),
        };

        self.vocabulary.insert(coords, entry);
        self.maintain_crystal_resonance()?;
        Ok(())
    }

    /// Retrieve a word's crystal embedding
    pub fn get_word(&self, word: &str) -> Option<&CrystalEmbedding> {
        if !self.check_resonance() {
            return None;
        }

        let coords = self.word_to_aetheric_coords(word);
        self.vocabulary.get(&coords).map(|entry| &entry.embedding)
    }

    /// Convert word to aetheric spatial coordinates
    fn word_to_aetheric_coords(&self, word: &str) -> Vector3D {
        let mut hasher = CrystalBrownHasher::new();
        let hash = word.chars().fold(0, |acc, c| {
            hasher.update(&[c as u8]);
            acc + hasher.state.get_state().unwrap()
        });

        // Map hash to 3D crystal space
        Vector3D::new(
            (hash % 1000) as f64 / 1000.0,
                      ((hash / 1000) % 1000) as f64 / 1000.0,
                      ((hash / 1000000) % 1000) as f64 / 1000.0,
        )
    }

    /// Check crystal resonance levels
    fn check_resonance(&self) -> bool {
        self.resonance.get_state().unwrap() >= self.resonance_threshold
    }

    /// Harmonize crystal resonance
    fn harmonize_resonance(&mut self) -> Result<(), LexiconError> {
        self.resonance = Aether::new(1.0);

        // Apply gravitational corrections
        let grav_factor = self.grav_constants.get_local_factor();
        self.resonance.modify_state(|r| r * grav_factor)
        .map_err(|_| LexiconError::ResonanceHarmonizationFailure)?;

        Ok(())
    }

    /// Maintain crystal lattice resonance
    fn maintain_crystal_resonance(&mut self) -> Result<(), LexiconError> {
        // Apply dream variance correction
        let dream_factor = self.vocabulary
        .get_crystal_pattern()
        .dream_variance
        .get_state()
        .unwrap();

        self.resonance.modify_state(|r| r * (1.0 - dream_factor * 0.1))
        .map_err(|_| LexiconError::CrystalResonanceDecay)?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum LexiconError {
    ResonanceHarmonizationFailure,
    CrystalResonanceDecay,
    AethericMappingFailure,
    DreamStateInstability,
}

impl std::fmt::Display for LexiconError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ResonanceHarmonizationFailure => write!(f, "Failed to harmonize crystal resonance"),
            Self::CrystalResonanceDecay => write!(f, "Crystal resonance decay detected"),
            Self::AethericMappingFailure => write!(f, "Failed to map word to aetheric coordinates"),
            Self::DreamStateInstability => write!(f, "Dream state instability detected"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexicon_initialization() {
        let lexicon = Lexicon::new(16);
        assert!(lexicon.check_resonance());
    }

    #[test]
    fn test_word_learning() {
        let mut lexicon = Lexicon::new(16);
        let embedding = CrystalEmbedding::new();
        assert!(lexicon.learn_word("crystal", embedding).is_ok());
        assert!(lexicon.get_word("crystal").is_some());
    }

    #[test]
    fn test_resonance_maintenance() {
        let mut lexicon = Lexicon::new(16);

        // Force resonance decay
        lexicon.resonance = Aether::new(0.5);
        assert!(!lexicon.check_resonance());

        // Test resonance recovery
        assert!(lexicon.harmonize_resonance().is_ok());
        assert!(lexicon.check_resonance());
    }

    #[test]
    fn test_aetheric_coordinate_mapping() {
        let lexicon = Lexicon::new(16);
        let coords = lexicon.word_to_aetheric_coords("crystal");
        assert!(coords.x >= 0.0 && coords.x <= 1.0);
        assert!(coords.y >= 0.0 && coords.y <= 1.0);
        assert!(coords.z >= 0.0 && coords.z <= 1.0);
    }
}
