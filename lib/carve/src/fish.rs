/// Fish Translation Module
/// Last Updated: 2025-01-15 03:50:37 UTC
/// Author: isdood
/// Current User: isdood

use super::TranslationState;
use unstable_matter::{
    Quantum, ScribePrecision, Scribe, QuantumString,
    Vector3D, UnstableDescriptor, QuantumState,
    QUANTUM_COHERENCE_THRESHOLD
};

#[derive(Debug, PartialEq)]
pub enum FishBlockState {
    Outside,    // Not in a Fish block
    Starting,   // Found opening !fish!
    Inside,     // Processing Fish code
    Ending,     // Found closing !fish!
}

#[derive(Debug)]
pub enum FishState {
    Parsing,
    Translating,
    Optimizing,
    Verifying,
    Complete,
    Failed,
}

/// Main translator for Fish code
#[derive(Debug)]
pub struct FishTranslator {
    state: TranslationState,
    fish_state: FishState,
    block_state: FishBlockState,
    indentation_level: usize,
    quantum_stability: f64,
    current_block: Vec<String>,
}

impl FishTranslator {
    pub fn new() -> Self {
        Self {
            state: TranslationState::new(),
            fish_state: FishState::Parsing,
            block_state: FishBlockState::Outside,
            indentation_level: 0,
            quantum_stability: 1.0,
            current_block: Vec::new(),
        }
    }

    /// Translate Fish code to the target format
    pub fn translate(&mut self, source: &str) -> Result<String, &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state too unstable for translation");
        }

        self.fish_state = FishState::Translating;
        self.process_code(source)
    }

    /// Process and transform Fish code
    fn process_code(&mut self, source: &str) -> Result<String, &'static str> {
        let mut result = String::new();

        for line in source.lines() {
            match self.block_state {
                FishBlockState::Outside => {
                    if line.trim() == "!fish!" {
                        self.block_state = FishBlockState::Starting;
                        result.push_str("// Begin Fish Translation Block\n");
                    } else {
                        // Pass through non-Fish code unchanged
                        result.push_str(line);
                        result.push('\n');
                    }
                },
                FishBlockState::Starting => {
                    self.block_state = FishBlockState::Inside;
                    self.current_block.clear();
                    self.current_block.push(line.to_string());
                },
                FishBlockState::Inside => {
                    if line.trim() == "!fish!" {
                        self.block_state = FishBlockState::Ending;
                        // Process the collected block
                        let translated = self.process_fish_block()?;
                        result.push_str(&translated);
                        result.push_str("// End Fish Translation Block\n");
                        self.block_state = FishBlockState::Outside;
                        self.current_block.clear();
                    } else {
                        self.current_block.push(line.to_string());
                    }
                },
                FishBlockState::Ending => {
                    self.block_state = FishBlockState::Outside;
                }
            }
        }

        if self.block_state != FishBlockState::Outside {
            return Err("Unclosed Fish block - missing !fish! terminator");
        }

        self.fish_state = FishState::Complete;
        Ok(result)
    }

    /// Process a complete Fish block
    fn process_fish_block(&mut self) -> Result<String, &'static str> {
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

    /// Process a single line of Fish code
    fn process_line(&mut self, line: &str) -> Result<String, &'static str> {
        // Skip empty lines and the !fish! markers
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed == "!fish!" {
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

impl Quantum for FishTranslator {
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

impl Scribe for FishTranslator {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("FishTranslator[");
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
        let mut translator = FishTranslator::new();
        let source = "!fish! echo 'Hello, World!' !fish!";
        let result = translator.translate(source).unwrap();
        assert!(result.contains("println!(\"Hello, World!\");"));
    }

    #[test]
    fn test_quantum_stability() {
        let translator = FishTranslator::new();
        assert!(translator.is_quantum_stable());
        assert!(translator.get_coherence() > QUANTUM_COHERENCE_THRESHOLD);
    }

    #[test]
    fn test_translation_state() {
        let mut translator = FishTranslator::new();
        assert!(matches!(translator.fish_state, FishState::Parsing));
        let _ = translator.translate("!fish! echo 'Hello, World!' !fish!");
        assert!(matches!(translator.fish_state, FishState::Complete));
    }

    #[test]
    fn test_unclosed_fish_block() {
        let mut translator = FishTranslator::new();
        let source = "!fish! echo 'Hello, World!'";
        let result = translator.translate(source);
        assert!(result.is_err());
    }
}
