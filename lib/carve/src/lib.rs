/// Carve Translation Library
/// Last Updated: 2025-01-15 03:08:03 UTC
/// Author: isdood
/// Current User: isdood

// Hashbrown space hashing
pub mod spacemap;

pub mod snek;
pub mod bash;
pub mod fish;
pub mod cplus;
pub mod prolog;

// Re-exports
pub use snek::SnekTranslator;
pub use bash::BashTranslator;
pub use fish::FishTranslator;
pub use cplus::CPlusTranslator;
pub use spacemap::SpaceMap;

use unstable_matter::{
    Quantum, ScribePrecision, Scribe, QuantumString,
    Vector3D, UnstableDescriptor, QuantumState
};

/// Represents a translation state in quantum space
#[derive(Debug)]
pub struct TranslationState {
    quantum_descriptor: UnstableDescriptor,
    source_coherence: f64,
    target_coherence: f64,
    translation_vector: Vector3D<f64>,
}

impl TranslationState {
    pub fn new() -> Self {
        Self {
            quantum_descriptor: UnstableDescriptor::new(),
            source_coherence: 1.0,
            target_coherence: 1.0,
            translation_vector: Vector3D::new(0.0, 0.0, 0.0),
        }
    }

    pub fn get_quantum_state(&self) -> QuantumState {
        *self.quantum_descriptor.state.get()
    }

    pub fn get_coherence(&self) -> f64 {
        (self.source_coherence + self.target_coherence) / 2.0
    }
}

impl Quantum for TranslationState {
    fn is_quantum_stable(&self) -> bool {
        self.quantum_descriptor.is_stable() &&
        self.source_coherence > 0.5 &&
        self.target_coherence > 0.5
    }

    fn get_coherence(&self) -> f64 {
        self.get_coherence()
    }

    fn decay_coherence(&self) {
        self.quantum_descriptor.decay_coherence();
    }

    fn reset_coherence(&self) {
        self.quantum_descriptor.reset();
    }
}

impl Scribe for TranslationState {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("TranslationState[");
        output.push_str("coherence=(");
        output.push_f64(self.source_coherence, precision.decimal_places());
        output.push_str(", ");
        output.push_f64(self.target_coherence, precision.decimal_places());
        output.push_str(")]");
    }
}

/// Unified Translator that handles different markers
pub struct UnifiedTranslator {
    translators: SpaceMap<String, Box<dyn Translator>>,
}

impl UnifiedTranslator {
    pub fn new() -> Self {
        let mut translators: SpaceMap<String, Box<dyn Translator>> = SpaceMap::new();
        translators.insert("snek".to_string(), Box::new(SnekTranslator::new()));
        translators.insert("bash".to_string(), Box::new(BashTranslator::new()));
        translators.insert("fish".to_string(), Box::new(FishTranslator::new()));
        translators.insert("cplus".to_string(), Box::new(CPlusTranslator::new()));

        Self { translators }
    }

    /// Translate code based on markers
    pub fn translate(&mut self, source: &str) -> Result<String, &'static str> {
        let mut result = String::new();
        let mut buffer = String::new();

        let mut current_translator: Option<&mut Box<dyn Translator>> = None;
        let parts: Vec<&str> = source.split('!').collect();

        for part in parts {
            if let Some(translator_name) = self.translators.get(part) {
                if current_translator.is_some() {
                    // End of current translation block
                    result.push_str("// End Translation Block\n");
                    let translated_content = current_translator.unwrap().translate_line(&buffer)?;
                    result.push_str(&translated_content);
                    buffer.clear();
                    current_translator = None;
                } else {
                    // Start of new translation block
                    result.push_str("// Begin Translation Block\n");
                    current_translator = Some(translator_name);
                }
            } else if let Some(translator) = &mut current_translator {
                buffer.push_str(part);
            } else {
                // Pass through non-translated content
                result.push_str(part);
            }
        }

        if current_translator.is_some() {
            return Err("Unclosed translation block - missing end marker");
        }

        Ok(result)
    }
}

/// Translator trait to be implemented by specific translators
pub trait Translator {
    fn translate_line(&mut self, line: &str) -> Result<String, &'static str>;
}

// Implement Translator for SnekTranslator
impl Translator for SnekTranslator {
    fn translate_line(&mut self, line: &str) -> Result<String, &'static str> {
        self.process_line(line)
    }
}

// Implement Translator for BashTranslator
impl Translator for BashTranslator {
    fn translate_line(&mut self, line: &str) -> Result<String, &'static str> {
        self.process_line(line)
    }
}

// Implement Translator for FishTranslator
impl Translator for FishTranslator {
    fn translate_line(&mut self, line: &str) -> Result<String, &'static str> {
        self.process_line(line)
    }
}

// Implement Translator for CPlusTranslator
impl Translator for CPlusTranslator {
    fn translate_line(&mut self, line: &str) -> Result<String, &'static str> {
        self.process_line(line)
    }
}
