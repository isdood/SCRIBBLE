/// Bash Translation Module
/// Last Updated: 2025-01-15 03:02:21 UTC
/// Author: isdood
/// Current User: isdood

use super::TranslationState;
use unstable_matter::{
    Quantum, ScribePrecision, Scribe, QuantumString,
    Vector3D, UnstableDescriptor, QuantumState,
    QUANTUM_COHERENCE_THRESHOLD
};

#[derive(Debug, PartialEq)]
pub enum BashBlockState {
    Outside,    // Not in a Bash block
    Starting,   // Found opening !bash!
    Inside,     // Processing Bash code
    Ending,     // Found closing !bash!
}

#[derive(Debug)]
pub enum BashState {
    Parsing,
    Translating,
    Optimizing,
    Verifying,
    Complete,
    Failed,
}

/// Main translator for Bash code
#[derive(Debug)]
pub struct BashTranslator {
    state: TranslationState,
    bash_state: BashState,
    block_state: BashBlockState,
    indentation_level: usize,
    quantum_stability: f64,
    current_block: Vec<String>,
}

impl BashTranslator {
    pub fn new() -> Self {
        Self {
            state: TranslationState::new(),
            bash_state: BashState::Parsing,
            block_state: BashBlockState::Outside,
            indentation_level: 0,
            quantum_stability: 1.0,
            current_block: Vec::new(),
        }
    }

    /// Translate Bash code to the target format
    pub fn translate(&mut self, source: &str) -> Result<String, &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state too unstable for translation");
        }

        self.bash_state = BashState::Translating;
        self.process_code(source)
    }

    /// Process and transform Bash code
    fn process_code(&mut self, source: &str) -> Result<String, &'static str> {
        let mut result = String::new();

        for line in source.lines() {
            match self.block_state {
                BashBlockState::Outside => {
                    if line.trim() == "!bash!" {
                        self.block_state = BashBlockState::Starting;
                        result.push_str("// Begin Bash Translation Block\n");
                    } else {
                        // Pass through non-Bash code unchanged
                        result.push_str(line);
                        result.push('\n');
                    }
                },
                BashBlockState::Starting => {
                    self.block_state = BashBlockState::Inside;
                    self.current_block.clear();
                    self.current_block.push(line.to_string());
                },
                BashBlockState::Inside => {
                    if line.trim() == "!bash!" {
                        self.block_state = BashBlockState::Ending;
                        // Process the collected block
                        let translated = self.process_bash_block()?;
                        result.push_str(&translated);
                        result.push_str("// End Bash Translation Block\n");
                        self.block_state = BashBlockState::Outside;
                        self.current_block.clear();
                    } else {
                        self.current_block.push(line.to_string());
                    }
                },
                BashBlockState::Ending => {
                    self.block_state = BashBlockState::Outside;
                }
            }
        }

        if self.block_state != BashBlockState::Outside {
            return Err("Unclosed Bash block - missing !bash! terminator");
        }

        self.bash_state = BashState::Complete;
        Ok(result)
    }

    /// Process a complete Bash block
    fn process_bash_block(&mut self) -> Result<String, &'static str> {
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

    /// Process a single line of Bash code
    fn process_line(&mut self, line: &str) -> Result<String, &'static str> {
        // Skip empty lines and the !bash! markers
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed == "!bash!" {
            return Ok(String::new());
        }

        // Basic translation rules
        let translated = match trimmed {
            s if s.starts_with("echo ") => self.translate_echo(s),
            s if s.starts_with("if ") => self.translate_if_statement(s),
            s if s.starts_with("for ") => self.translate_for_loop(s),
            s if s.starts_with("while ") => self.translate_while_loop(s),
            s if s.starts_with("#") => self.translate_comment(s),
            _ => Ok(String::from(trimmed)),
        }?;

        // Add proper indentation to the translated code
        Ok(format!("{}{}", "    ".repeat(self.indentation_level), translated))
    }

    // Translation helper methods
    fn translate_echo(&self, line: &str) -> Result<String, &'static str> {
        let without_echo = line.strip_prefix("echo ").unwrap();
        Ok(format!("println!(\"{}\");", without_echo))
    }

    fn translate_if_statement(&self, line: &str) -> Result<String, &'static str> {
        let without_if = line.strip_prefix("if ").unwrap();
        let condition = without_if.trim_end_matches(';').trim();
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

    fn translate_comment(&self, line: &str) -> Result<String, &'static str> {
        Ok(format!("// {}", line.trim_start_matches('#').trim()))
    }
}

impl Quantum for BashTranslator {
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

impl Scribe for BashTranslator {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("BashTranslator[");
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
        let mut translator = BashTranslator::new();
        let source = "!bash! echo 'Hello, World!' !bash!";
        let result = translator.translate(source).unwrap();
        assert!(result.contains("println!(\"Hello, World!\");"));
    }

    #[test]
    fn test_quantum_stability() {
        let translator = BashTranslator::new();
        assert!(translator.is_quantum_stable());
        assert!(translator.get_coherence() > QUANTUM_COHERENCE_THRESHOLD);
    }

    #[test]
    fn test_translation_state() {
        let mut translator = BashTranslator::new();
        assert!(matches!(translator.bash_state, BashState::Parsing));
        let _ = translator.translate("!bash! echo 'Hello, World!' !bash!");
        assert!(matches!(translator.bash_state, BashState::Complete));
    }

    #[test]
    fn test_unclosed_bash_block() {
        let mut translator = BashTranslator::new();
        let source = "!bash! echo 'Hello, World!'";
        let result = translator.translate(source);
        assert!(result.is_err());
    }
}
