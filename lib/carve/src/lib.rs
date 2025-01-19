/// Carve Translation Library
/// Last Updated: 2025-01-19 08:15:40 UTC
/// Author: isdood
/// Current User: isdood

// Hashbrown space hashing
pub mod spacemap;

pub mod assy;
pub mod html;
pub mod php;
pub mod snek;
pub mod bash;
pub mod fish;
pub mod cplus;
pub mod prolog;
pub mod sql;
pub mod java;
pub mod js;
pub mod pwr;
pub mod go;
pub mod zig;

// Re-exports
pub use html::HtmlTranslator;
pub use php::PhpTranslator;
pub use snek::SnekTranslator;
pub use bash::BashTranslator;
pub use fish::FishTranslator;
pub use cplus::CPlusTranslator;
pub use prolog::PrologTranslator;
pub use sql::SqlTranslator;
pub use java::JavaTranslator;
pub use js::JsTranslator;
pub use pwr::PwrTranslator;
pub use assy::AssyTranslator;
pub use go::GoTranslator;
pub use zig::ZigTranslator;
pub use spacemap::SpaceMap;

use harmony_core::{
    Quantum, ScribePrecision, Scribe, QuantumString,
    Vector3D, cube::CrystalCube, harmony::MeshValue
};

/// Represents a translation state in quantum space
#[derive(Debug)]
pub struct TranslationState {
    crystal_cube: CrystalCube<MeshValue>,
    source_coherence: f64,
    target_coherence: f64,
    translation_vector: Vector3D<f64>,
}

impl TranslationState {
    pub fn new() -> Self {
        Self {
            crystal_cube: CrystalCube::new(),
            source_coherence: 1.0,
            target_coherence: 1.0,
            translation_vector: Vector3D::new(0.0, 0.0, 0.0),
        }
    }

    pub fn get_quantum_state(&self) -> bool {
        self.crystal_cube.is_stable()
    }

    pub fn get_coherence(&self) -> f64 {
        (self.source_coherence + self.target_coherence) / 2.0
    }
}

impl Quantum for TranslationState {
    fn is_quantum_stable(&self) -> bool {
        self.crystal_cube.is_stable() &&
        self.source_coherence > 0.5 &&
        self.target_coherence > 0.5
    }

    fn get_coherence(&self) -> f64 {
        self.get_coherence()
    }

    fn decay_coherence(&self) {
        self.crystal_cube.decay();
    }

    fn reset_coherence(&self) {
        self.crystal_cube.reset();
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
        translators.insert("prolog".to_string(), Box::new(PrologTranslator::new()));
        translators.insert("sql".to_string(), Box::new(SqlTranslator::new()));
        translators.insert("java".to_string(), Box::new(JavaTranslator::new()));
        translators.insert("js".to_string(), Box::new(JsTranslator::new()));
        translators.insert("pwr".to_string(), Box::new(PwrTranslator::new()));
        translators.insert("html".to_string(), Box::new(HtmlTranslator::new()));
        translators.insert("assy".to_string(), Box::new(AssyTranslator::new()));
        translators.insert("php".to_string(), Box::new(PhpTranslator::new()));
        translators.insert("go".to_string(), Box::new(GoTranslator::new()));
        translators.insert("zig".to_string(), Box::new(ZigTranslator::new()));

        Self { translators }
    }

    /// Translate code based on markers
    pub fn translate(&mut self, source: &str) -> Result<String, &'static str> {
        let mut result = String::new();
        let mut buffer = String::new();
        let mut inline_mode = false;
        let mut current_translator: Option<(&str, &mut Box<dyn Translator>)> = None;

        let parts: Vec<&str> = source.split('!').collect();

        for (i, part) in parts.iter().enumerate() {
            if let Some(translator_name) = self.translators.get(part.trim()) {
                if current_translator.is_some() {
                    // End of current translation block
                    let (lang, translator) = current_translator.take().unwrap();
                    let translated_content = translator.translate_line(&buffer)?;

                    if inline_mode {
                        // Verify spaces around markers for inline mode
                        if !buffer.starts_with(' ') || !buffer.ends_with(' ') {
                            return Err("Inline translation blocks must have spaces before and after the content");
                        }
                        let trimmed_content = buffer.trim();
                        result.push_str(&format!("inline_{}!({});", lang, trimmed_content));
                    } else {
                        result.push_str("// End Translation Block\n");
                        result.push_str(&translated_content);
                    }

                    buffer.clear();
                } else {
                    // Start of new translation block
                    // Check if we're in inline mode (part of a larger line)
                    inline_mode = i > 0 && !parts[i-1].trim().is_empty();
                    if !inline_mode {
                        result.push_str("// Begin Translation Block\n");
                    }
                    current_translator = Some((part.trim(), translator_name));
                }
            } else if let Some((_, translator)) = &mut current_translator {
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

// Macro to implement Translator trait
macro_rules! impl_translator {
    ($($t:ty),*) => {
        $(
            impl Translator for $t {
                fn translate_line(&mut self, line: &str) -> Result<String, &'static str> {
                    self.process_line(line)
                }
            }
        )*
    };
}

// Implement Translator for all translators
impl_translator!(
    SnekTranslator,
    BashTranslator,
    FishTranslator,
    CPlusTranslator,
    PrologTranslator,
    SqlTranslator,
    JavaTranslator,
    JsTranslator,
    PwrTranslator,
    HtmlTranslator,
    PhpTranslator,
    AssyTranslator,
    GoTranslator,
    ZigTranslator
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inline_translation_with_spaces() {
        let mut translator = UnifiedTranslator::new();
        let source = "Let's run !sql! SELECT * FROM users !sql! and then !bash! echo 'Done' !bash!";
        let result = translator.translate(source).unwrap();
        assert!(result.contains("inline_sql!(SELECT * FROM users);"));
        assert!(result.contains("inline_bash!(echo 'Done');"));
    }

    #[test]
    fn test_inline_translation_without_spaces() {
        let mut translator = UnifiedTranslator::new();
        let source = "Let's run !sql!SELECT * FROM users!sql! and continue...";
        let result = translator.translate(source);
        assert!(result.is_err());
    }

    #[test]
    fn test_block_translation() {
        let mut translator = UnifiedTranslator::new();
        let source = "!sql!\nSELECT * FROM users;\n!sql!";
        let result = translator.translate(source).unwrap();
        assert!(result.contains("// Begin Translation Block"));
        assert!(result.contains("// End Translation Block"));
    }

    #[test]
    fn test_mixed_translations() {
        let mut translator = UnifiedTranslator::new();
        let source = "First !bash! echo 'hello' !bash!\n!sql!\nSELECT * FROM users;\n!sql!";
        let result = translator.translate(source).unwrap();
        assert!(result.contains("inline_bash!"));
        assert!(result.contains("// Begin Translation Block"));
    }

    #[test]
    fn test_unclosed_block() {
        let mut translator = UnifiedTranslator::new();
        let source = "!sql! SELECT * FROM users";
        let result = translator.translate(source);
        assert!(result.is_err());
    }

    #[test]
    fn test_zig_translation() {
        let mut translator = UnifiedTranslator::new();
        let source = "!zig! fn main() => void { const x := 42; } !zig!";
        let result = translator.translate(source).unwrap();
        assert!(result.contains("pub fn main() void {"));
        assert!(result.contains("const x = 42;"));
    }
}
