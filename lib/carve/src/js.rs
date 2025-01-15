/// JavaScript Translation Module
/// Last Updated: 2025-01-15 04:09:40 UTC
/// Author: isdood
/// Current User: isdood

use super::TranslationState;
use unstable_matter::{
    Quantum, ScribePrecision, Scribe, QuantumString,
    Vector3D, UnstableDescriptor, QuantumState,
    QUANTUM_COHERENCE_THRESHOLD
};

#[derive(Debug, PartialEq)]
pub enum JsBlockState {
    Outside,    // Not in a JS block
    Starting,   // Found opening !js!
    Inside,     // Processing JS code
    Ending,     // Found closing !js!
}

#[derive(Debug)]
pub enum JsState {
    Parsing,
    Translating,
    Optimizing,
    Verifying,
    Complete,
    Failed,
}

/// Main translator for JavaScript code
#[derive(Debug)]
pub struct JsTranslator {
    state: TranslationState,
    js_state: JsState,
    block_state: JsBlockState,
    indentation_level: usize,
    quantum_stability: f64,
    current_block: Vec<String>,
}

impl JsTranslator {
    pub fn new() -> Self {
        Self {
            state: TranslationState::new(),
            js_state: JsState::Parsing,
            block_state: JsBlockState::Outside,
            indentation_level: 0,
            quantum_stability: 1.0,
            current_block: Vec::new(),
        }
    }

    /// Translate JavaScript code to the target format
    pub fn translate(&mut self, source: &str) -> Result<String, &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state too unstable for translation");
        }

        self.js_state = JsState::Translating;
        self.process_code(source)
    }

    /// Process and transform JavaScript code
    fn process_code(&mut self, source: &str) -> Result<String, &'static str> {
        let mut result = String::new();

        for line in source.lines() {
            match self.block_state {
                JsBlockState::Outside => {
                    if line.trim() == "!js!" {
                        self.block_state = JsBlockState::Starting;
                        result.push_str("// Begin JavaScript Translation Block\n");
                    } else {
                        result.push_str(line);
                        result.push('\n');
                    }
                },
                JsBlockState::Starting => {
                    self.block_state = JsBlockState::Inside;
                    self.current_block.clear();
                    self.current_block.push(line.to_string());
                },
                JsBlockState::Inside => {
                    if line.trim() == "!js!" {
                        self.block_state = JsBlockState::Ending;
                        let translated = self.process_js_block()?;
                        result.push_str(&translated);
                        result.push_str("// End JavaScript Translation Block\n");
                        self.block_state = JsBlockState::Outside;
                        self.current_block.clear();
                    } else {
                        self.current_block.push(line.to_string());
                    }
                },
                JsBlockState::Ending => {
                    self.block_state = JsBlockState::Outside;
                }
            }
        }

        if self.block_state != JsBlockState::Outside {
            return Err("Unclosed JavaScript block - missing !js! terminator");
        }

        self.js_state = JsState::Complete;
        Ok(result)
    }

    /// Process a complete JavaScript block
    fn process_js_block(&mut self) -> Result<String, &'static str> {
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

    /// Process a single line of JavaScript code
    fn process_line(&mut self, line: &str) -> Result<String, &'static str> {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed == "!js!" {
            return Ok(String::new());
        }

        let translated = match trimmed {
            s if s.starts_with("function ") => self.translate_function(s),
            s if s.starts_with("const ") => self.translate_const(s),
            s if s.starts_with("let ") => self.translate_let(s),
            s if s.starts_with("//") => self.translate_comment(s),
            _ => Ok(String::from(trimmed)),
        }?;

        Ok(format!("{}{}", "    ".repeat(self.indentation_level), translated))
    }

    fn translate_function(&self, line: &str) -> Result<String, &'static str> {
        Ok(format!("fn {} {{", line.trim_start_matches("function ")))
    }

    fn translate_const(&self, line: &str) -> Result<String, &'static str> {
        Ok(format!("const {}", line.trim_start_matches("const ")))
    }

    fn translate_let(&self, line: &str) -> Result<String, &'static str> {
        Ok(format!("let {}", line.trim_start_matches("let ")))
    }

    fn translate_comment(&self, line: &str) -> Result<String, &'static str> {
        Ok(String::from(line))
    }
}

impl Quantum for JsTranslator {
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

impl Scribe for JsTranslator {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("JsTranslator[");
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
    fn test_basic_function_translation() {
        let mut translator = JsTranslator::new();
        let source = "!js! function hello() { } !js!";
        let result = translator.translate(source).unwrap();
        assert!(result.contains("fn hello() {"));
    }

    #[test]
    fn test_unclosed_js_block() {
        let mut translator = JsTranslator::new();
        let source = "!js! function example() {";
        let result = translator.translate(source);
        assert!(result.is_err());
    }
}
