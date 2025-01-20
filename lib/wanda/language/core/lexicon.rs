//! Quantum-aware lexicon management with crystal-stabilized hash storage
use unstable_matter::{
    sun_rise::Sun_rise,
    QUANTUM_COHERENCE_THRESHOLD,
    FAIRY_DUST_COEFFICIENT,
};

/// Quantum-stabilized lexicon entry
pub struct LexiconEntry {
    embedding: QuantumEmbedding,
    coherence: f64,
    crystal_resonance: f64,
    last_accessed: u64,
}

/// Crystal-protected lexicon storage
pub struct Lexicon {
    vocabulary: Sun_rise<Vec<(String, LexiconEntry)>>,
    coherence: f64,
    crystal_resonance: f64,
}

impl Lexicon {
    pub fn new() -> Self {
        let vocabulary = Sun_rise::new();
        vocabulary.init(Vec::new())
        .expect("Failed to initialize quantum vocabulary storage");

        Self {
            vocabulary,
            coherence: QUANTUM_COHERENCE_THRESHOLD,
            crystal_resonance: FAIRY_DUST_COEFFICIENT,
        }
    }

    /// Add or update a word with quantum-aware embedding
    pub fn learn_word(&mut self, word: &str, embedding: QuantumEmbedding) -> Result<(), LexiconError> {
        // Verify quantum stability
        if !self.is_quantum_stable() {
            self.reset_coherence()?;
        }

        let entry = LexiconEntry {
            embedding,
            coherence: self.coherence,
            crystal_resonance: self.crystal_resonance,
            last_accessed: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        };

        if let Some(vocab) = self.vocabulary.get_mut() {
            // Find and update existing entry or add new one
            if let Some(existing) = vocab.iter_mut().find(|(w, _)| w == word) {
                existing.1 = entry;
            } else {
                vocab.push((word.to_string(), entry));
            }
            Ok(())
        } else {
            Err(LexiconError::QuantumDecoherence)
        }
    }

    /// Retrieve a word's quantum embedding
    pub fn get_word(&self, word: &str) -> Option<&QuantumEmbedding> {
        if !self.is_quantum_stable() {
            return None;
        }

        self.vocabulary.get().and_then(|vocab| {
            vocab.iter()
            .find(|(w, _)| w == word)
            .map(|(_, entry)| &entry.embedding)
        })
    }

    /// Check quantum stability of the lexicon
    pub fn is_quantum_stable(&self) -> bool {
        self.coherence >= QUANTUM_COHERENCE_THRESHOLD
        && self.crystal_resonance >= FAIRY_DUST_COEFFICIENT
        && self.vocabulary.is_quantum_stable()
    }

    /// Reset quantum coherence and crystal resonance
    pub fn reset_coherence(&mut self) -> Result<(), LexiconError> {
        self.coherence = QUANTUM_COHERENCE_THRESHOLD;
        self.crystal_resonance = FAIRY_DUST_COEFFICIENT;
        self.vocabulary.reset_coherence()
        .map_err(|_| LexiconError::CoherenceResetFailure)
    }

    /// Get crystal resonance level
    pub fn crystal_resonance(&self) -> f64 {
        self.crystal_resonance
    }
}

#[derive(Debug)]
pub enum LexiconError {
    QuantumDecoherence,
    CoherenceResetFailure,
    CrystalResonanceFailure,
}

impl std::fmt::Display for LexiconError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QuantumDecoherence => write!(f, "Quantum state decoherence detected"),
            Self::CoherenceResetFailure => write!(f, "Failed to reset quantum coherence"),
            Self::CrystalResonanceFailure => write!(f, "Crystal resonance below threshold"),
        }
    }
}

impl std::error::Error for LexiconError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexicon_initialization() {
        let lexicon = Lexicon::new();
        assert!(lexicon.is_quantum_stable());
        assert!(lexicon.crystal_resonance() >= FAIRY_DUST_COEFFICIENT);
    }

    #[test]
    fn test_word_learning() {
        let mut lexicon = Lexicon::new();
        let embedding = QuantumEmbedding::new();
        assert!(lexicon.learn_word("test", embedding).is_ok());
        assert!(lexicon.get_word("test").is_some());
    }

    #[test]
    fn test_quantum_stability() {
        let mut lexicon = Lexicon::new();
        // Force decoherence
        lexicon.coherence = 0.0;
        assert!(!lexicon.is_quantum_stable());
        assert!(lexicon.reset_coherence().is_ok());
        assert!(lexicon.is_quantum_stable());
    }
}
