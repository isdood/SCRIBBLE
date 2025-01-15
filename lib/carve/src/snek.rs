/// Python (Snek) Translation Module
/// Last Updated: 2025-01-15 02:46:38 UTC
/// Author: isdood
/// Current User: isdood

use super::TranslationState;
use unstable_matter::{
    Quantum, ScribePrecision, Scribe, QuantumString,
    Vector3D, UnstableDescriptor, QuantumState,
    QUANTUM_COHERENCE_THRESHOLD
};

#[derive(Debug, PartialEq)]
pub enum SnekBlockState {
    Outside,    // Not in a Snek block
    Starting,   // Found opening !snek!
    Inside,     // Processing Snek code
    Ending,     // Found closing !snek!
}

#[derive(Debug)]
pub enum SnekState {
    Parsing,
    Translating,
    Optimizing,
    Verifying,
    Complete,
    Failed,
}

/// Main translator for Python code
#[derive(Debug)]
pub struct SnekTranslator {
    state: TranslationState,
    snek_state: SnekState,
    block_state: SnekBlockState,
    indentation_level: usize,
    quantum_stability: f64,
    current_block: Vec<String>,
}

impl SnekTranslator {
    pub fn new() -> Self {
        Self {
            state: TranslationState::new(),
            snek_state: SnekState::Parsing,
            block_state: SnekBlockState::Outside,
            indentation_level: 0,
            quantum_stability: 1.0,
            current_block: Vec::new(),
        }
    }

    /// Translate Python code to the target format
    pub fn translate(&mut self, source: &str) -> Result<String, &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state too unstable for translation");
        }

        self.snek_state = SnekState::Translating;
        self.process_code(source)
    }

    /// Process and transform Python code
    fn process_code(&mut self, source: &str) -> Result<String, &'static str> {
        let mut result = String::new();

        for line in source.lines() {
            match self.block_state {
                SnekBlockState::Outside => {
                    if line.trim() == "!snek!" {
                        self.block_state = SnekBlockState::Starting;
                        result.push_str("// Begin Snek Translation Block\n");
                    } else {
                        // Pass through non-Snek code unchanged
                        result.push_str(line);
                        result.push('\n');
                    }
                },
                SnekBlockState::Starting => {
                    self.block_state = SnekBlockState::Inside;
                    self.current_block.clear();
                    self.current_block.push(line.to_string());
                },
                SnekBlockState::Inside => {
                    if line.trim() == "!snek!" {
                        self.block_state = SnekBlockState::Ending;
                        // Process the collected block
                        let translated = self.process_snek_block()?;
                        result.push_str(&translated);
                        result.push_str("// End Snek Translation Block\n");
                        self.block_state = SnekBlockState::Outside;
                        self.current_block.clear();
                    } else {
                        self.current_block.push(line.to_string());
                    }
                },
                SnekBlockState::Ending => {
                    self.block_state = SnekBlockState::Outside;
                }
            }
        }

        if self.block_state != SnekBlockState::Outside {
            return Err("Unclosed Snek block - missing !snek! terminator");
        }

        self.snek_state = SnekState::Complete;
        Ok(result)
    }

    /// Process a complete Snek block
    fn process_snek_block(&mut self) -> Result<String, &'static str> {
        let mut result = String::new();
        let mut base_indent = None;

        // Calculate the base indentation level
        for line in &self.current_block {
            let indent_count = line.chars().take_while(|c| c.is_whitespace()).count();
            if !line.trim().is_empty() {
                base_indent = Some(base_indent.unwrap_or(indent_count).min(indent_count));
            }
        }

        // Process each line with respect to the base indentation
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

    /// Process a single line of Python code
    fn process_line(&mut self, line: &str) -> Result<String, &'static str> {
        // Skip empty lines and the !snek! markers
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed == "!snek!" {
            return Ok(String::new());
        }

        // Basic translation rules
        let translated = match trimmed {
            s if s.starts_with("def ") => self.translate_function(s),
            s if s.starts_with("class ") => self.translate_class(s),
            s if s.contains("=") => self.translate_assignment(s),
            s if s.starts_with("if ") => self.translate_if_statement(s),
            s if s.starts_with("for ") => self.translate_for_loop(s),
            s if s.starts_with("while ") => self.translate_while_loop(s),
            s if s.starts_with("return ") => self.translate_return(s),
            s if s.starts_with("#") => self.translate_comment(s),
            _ => Ok(String::from(trimmed)),
        }?;

        // Add proper indentation to the translated code
        Ok(format!("{}{}", "    ".repeat(self.indentation_level), translated))
    }

    // Translation helper methods
    fn translate_function(&self, line: &str) -> Result<String, &'static str> {
        let without_def = line.strip_prefix("def ").unwrap();
        let (name, params) = without_def.split_once('(').unwrap_or((without_def, ""));
        let params = params.trim_end_matches("):").trim();
        Ok(format!("pub fn {}({}) {{", name, params))
    }

    fn translate_class(&self, line: &str) -> Result<String, &'static str> {
        let without_class = line.strip_prefix("class ").unwrap();
        let name = without_class.trim_end_matches(':');
        Ok(format!("#[derive(Debug)]\npub struct {} {{", name))
    }

    fn translate_assignment(&self, line: &str) -> Result<String, &'static str> {
        if line.contains("+=") || line.contains("-=") || line.contains("*=") || line.contains("/=") {
            Ok(line.to_string())
        } else {
            Ok(format!("let {}", line))
        }
    }

    fn translate_if_statement(&self, line: &str) -> Result<String, &'static str> {
        let without_if = line.strip_prefix("if ").unwrap();
        let condition = without_if.trim_end_matches(':');
        Ok(format!("if {} {{", condition))
    }

    fn translate_for_loop(&self, line: &str) -> Result<String, &'static str> {
        let without_for = line.strip_prefix("for ").unwrap();
        Ok(format!("for {} {{", without_for))
    }

    fn translate_while_loop(&self, line: &str) -> Result<String, &'static str> {
        let without_while = line.strip_prefix("while ").unwrap();
        Ok(format!("while {} {{", without_while))
    }

    fn translate_return(&self, line: &str) -> Result<String, &'static str> {
        let without_return = line.strip_prefix("return ").unwrap();
        Ok(format!("return {};", without_return))
    }

    fn translate_comment(&self, line: &str) -> Result<String, &'static str> {
        Ok(format!("// {}", line.trim_start_matches('#').trim()))
    }
}

impl Quantum for SnekTranslator {
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

impl Scribe for SnekTranslator {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("SnekTranslator[");
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
    fn test_basic_translation() {
        let mut translator = SnekTranslator::new();
        let source = "!snek!\ndef hello_world():\n    print('Hello, World!')\n!snek!";
        let result = translator.translate(source).unwrap();
        assert!(result.contains("pub fn hello_world() {"));
    }

    #[test]
    fn test_quantum_stability() {
        let translator = SnekTranslator::new();
        assert!(translator.is_quantum_stable());
        assert!(translator.get_coherence() > QUANTUM_COHERENCE_THRESHOLD);
    }

    #[test]
    fn test_translation_state() {
        let mut translator = SnekTranslator::new();
        assert!(matches!(translator.snek_state, SnekState::Parsing));
        let _ = translator.translate("!snek!\nx = 1\n!snek!");
        assert!(matches!(translator.snek_state, SnekState::Complete));
    }

    #[test]
    fn test_unclosed_snek_block() {
        let mut translator = SnekTranslator::new();
        let source = "!snek!\ndef hello_world():\n    print('Hello, World!')";
        let result = translator.translate(source);
        assert!(result.is_err());
    }
}
