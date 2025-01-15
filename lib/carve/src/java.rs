/// Java Translation Module
/// Last Updated: 2025-01-15 04:01:38 UTC
/// Author: isdood
/// Current User: isdood

use super::TranslationState;
use unstable_matter::{
    Quantum, ScribePrecision, Scribe, QuantumString,
    Vector3D, UnstableDescriptor, QuantumState,
    QUANTUM_COHERENCE_THRESHOLD
};

#[derive(Debug, PartialEq)]
pub enum JavaBlockState {
    Outside,    // Not in a Java block
    Starting,   // Found opening !java!
    Inside,     // Processing Java code
    Ending,     // Found closing !java!
}

#[derive(Debug)]
pub enum JavaState {
    Parsing,
    Translating,
    Optimizing,
    Verifying,
    Complete,
    Failed,
}

/// Main translator for Java code
#[derive(Debug)]
pub struct JavaTranslator {
    state: TranslationState,
    java_state: JavaState,
    block_state: JavaBlockState,
    indentation_level: usize,
    quantum_stability: f64,
    current_block: Vec<String>,
}

impl JavaTranslator {
    pub fn new() -> Self {
        Self {
            state: TranslationState::new(),
            java_state: JavaState::Parsing,
            block_state: JavaBlockState::Outside,
            indentation_level: 0,
            quantum_stability: 1.0,
            current_block: Vec::new(),
        }
    }

    /// Translate Java code to the target format
    pub fn translate(&mut self, source: &str) -> Result<String, &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state too unstable for translation");
        }

        self.java_state = JavaState::Translating;
        self.process_code(source)
    }

    /// Process and transform Java code
    fn process_code(&mut self, source: &str) -> Result<String, &'static str> {
        let mut result = String::new();

        for line in source.lines() {
            match self.block_state {
                JavaBlockState::Outside => {
                    if line.trim() == "!java!" {
                        self.block_state = JavaBlockState::Starting;
                        result.push_str("// Begin Java Translation Block\n");
                    } else {
                        result.push_str(line);
                        result.push('\n');
                    }
                },
                JavaBlockState::Starting => {
                    self.block_state = JavaBlockState::Inside;
                    self.current_block.clear();
                    self.current_block.push(line.to_string());
                },
                JavaBlockState::Inside => {
                    if line.trim() == "!java!" {
                        self.block_state = JavaBlockState::Ending;
                        let translated = self.process_java_block()?;
                        result.push_str(&translated);
                        result.push_str("// End Java Translation Block\n");
                        self.block_state = JavaBlockState::Outside;
                        self.current_block.clear();
                    } else {
                        self.current_block.push(line.to_string());
                    }
                },
                JavaBlockState::Ending => {
                    self.block_state = JavaBlockState::Outside;
                }
            }
        }

        if self.block_state != JavaBlockState::Outside {
            return Err("Unclosed Java block - missing !java! terminator");
        }

        self.java_state = JavaState::Complete;
        Ok(result)
    }

    /// Process a complete Java block
    fn process_java_block(&mut self) -> Result<String, &'static str> {
        let mut result = String::new();
        let mut base_indent = None;

        for line in &self.current_block {
            let indent_count = line.chars().take_while(|c| c.is_whitespace()).count();
            if !line.trim().is_empty() {
                base_indent = Some(base_indent.unwrap_or(indent_count).min(indent_count));
            }
        }

        for line in &self.current_block {
            let indent_count = line.chars().take_while(|c| c.is_whitespace()).count();
            if let Some(base) = base_indent {
                self.indentation_level = (indent_count.saturating_sub(base)) / 4;
            }

            let processed = self.process_line(line)?;
            if !processed.is_empty() {
                result.push_str(&processed);
                result.push('\n');
            }
        }

        Ok(result)
    }

    /// Process a single line of Java code
    fn process_line(&mut self, line: &str) -> Result<String, &'static str> {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed == "!java!" {
            return Ok(String::new());
        }

        let translated = match trimmed {
            s if s.starts_with("public class ") => self.translate_class(s),
            s if s.starts_with("//") => self.translate_comment(s),
            _ => Ok(String::from(trimmed)),
        }?;

        Ok(format!("{}{}", "    ".repeat(self.indentation_level), translated))
    }

    fn translate_class(&self, line: &str) -> Result<String, &'static str> {
        let class_def = line.trim_end_matches('{').trim();
        Ok(format!("pub struct {} {{", class_def.replace("public class ", "")))
    }

    fn translate_comment(&self, line: &str) -> Result<String, &'static str> {
        Ok(String::from(line))
    }
}

impl Quantum for JavaTranslator {
    fn is_quantum_stable(&self) -> bool {
        self.state.is_quantum_stable() &&
        self.quantum_stability > QUANTUM_COHERENCE_THRESHOLD
    }

    fn get_coherence(&self) -> f64 {
        self.state.get_coherence() * self.quantum_stability
    }

    fn decay_coherence(&self) {
        self.state.decay_coherence();
        self.quantum_stability *= 0.99;
    }

    fn reset_coherence(&self) {
        self.state.reset_coherence();
        self.quantum_stability = 1.0;
    }
}

impl Scribe for JavaTranslator {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("JavaTranslator[");
        self.state.scribe(precision, output);
        output.push_str(", stability=");
        output.push_f64(self.quantum_stability, precision.decimal_places());
        output.push_char(']');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_class_translation() {
        let mut translator = JavaTranslator::new();
        let source = "!java! public class Example { } !java!";
        let result = translator.translate(source).unwrap();
        assert!(result.contains("pub struct Example {"));
    }

    #[test]
    fn test_unclosed_java_block() {
        let mut translator = JavaTranslator::new();
        let source = "!java! public class Example {";
        let result = translator.translate(source);
        assert!(result.is_err());
    }
}
