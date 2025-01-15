use crate::quantum::{Quantum, QUANTUM_COHERENCE_THRESHOLD};
use crate::scribe::{Scribe, ScribePrecision, QuantumString};
use crate::state::TranslationState;
use crate::Translator;

/// Represents the current state of the Prolog translator
#[derive(Debug)]
pub enum PrologState {
    Parsing,
    Complete,
    Error,
}

/// Translator for converting Prolog code to Rust
pub struct PrologTranslator {
    state: TranslationState,
    prolog_state: PrologState,
    quantum_stability: f64,
    indentation_level: usize,
}

impl PrologTranslator {
    /// Creates a new instance of PrologTranslator
    pub fn new() -> Self {
        Self {
            state: TranslationState::new(),
            prolog_state: PrologState::Parsing,
            quantum_stability: 1.0,
            indentation_level: 0,
        }
    }

    /// Translates a block of Prolog code to Rust
    pub fn translate(&mut self, source: &str) -> Result<String, &'static str> {
        if !source.starts_with("!prolog!") || !source.ends_with("!prolog!") {
            self.prolog_state = PrologState::Error;
            return Err("Invalid Prolog code block markers");
        }

        // Extract the Prolog code between the markers
        let prolog_code = source
        .strip_prefix("!prolog!")
        .unwrap()
        .strip_suffix("!prolog!")
        .unwrap()
        .trim();

        let mut result = String::new();
        for line in prolog_code.lines() {
            let translated = self.process_line(line)?;
            result.push_str(&translated);
            result.push('\n');
        }

        self.prolog_state = PrologState::Complete;
        Ok(result)
    }

    fn process_line(&mut self, line: &str) -> Result<String, &'static str> {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            return Ok(String::new());
        }

        // Handle indentation changes
        if trimmed == ")" {
            self.indentation_level = self.indentation_level.saturating_sub(1);
        }

        let translated = if trimmed.starts_with("%") {
            self.translate_comment(trimmed)?
        } else if trimmed.ends_with(":-") {
            self.translate_rule_head(trimmed)?
        } else if trimmed.ends_with(".") {
            self.translate_fact(trimmed)?
        } else if trimmed.contains(":-") {
            self.translate_rule(trimmed)?
        } else {
            self.translate_query(trimmed)?
        };

        // Adjust indentation for nested rules
        if trimmed.contains(":-") {
            self.indentation_level += 1;
        }

        Ok(format!("{}{}", "    ".repeat(self.indentation_level), translated))
    }

    // Translation helper methods
    fn translate_comment(&self, line: &str) -> Result<String, &'static str> {
        Ok(format!("// {}", line.trim_start_matches('%').trim()))
    }

    fn translate_fact(&self, line: &str) -> Result<String, &'static str> {
        let fact = line.trim_end_matches('.');
        Ok(format!("fact!({});", fact))
    }

    fn translate_rule_head(&self, line: &str) -> Result<String, &'static str> {
        let head = line.trim_end_matches(":-");
        Ok(format!("rule!({}) {{", head))
    }

    fn translate_rule(&self, line: &str) -> Result<String, &'static str> {
        let parts: Vec<&str> = line.split(":-").collect();
        if parts.len() != 2 {
            return Err("Invalid rule format");
        }
        let head = parts[0].trim();
        let body = parts[1].trim().trim_end_matches('.');
        Ok(format!("rule!({}) {{ {}", head, body))
    }

    fn translate_query(&self, line: &str) -> Result<String, &'static str> {
        Ok(format!("query!({});", line))
    }
}

impl Quantum for PrologTranslator {
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

impl Scribe for PrologTranslator {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("PrologTranslator[");
        self.state.scribe(precision, output);
        output.push_str(", stability=");
        output.push_f64(self.quantum_stability, precision.decimal_places());
        output.push_char(']');
    }
}

impl Translator for PrologTranslator {
    fn translate_line(&mut self, line: &str) -> Result<String, &'static str> {
        self.process_line(line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_translation() {
        let mut translator = PrologTranslator::new();
        let source = "!prolog! parent(john, mary). !prolog!";
        let result = translator.translate(source).unwrap();
        assert!(result.contains("fact!(parent(john, mary));"));
    }

    #[test]
    fn test_rule_translation() {
        let mut translator = PrologTranslator::new();
        let source = "!prolog! grandparent(X, Y) :- parent(X, Z), parent(Z, Y). !prolog!";
        let result = translator.translate(source).unwrap();
        assert!(result.contains("rule!(grandparent(X, Y))"));
    }

    #[test]
    fn test_comment_translation() {
        let mut translator = PrologTranslator::new();
        let source = "!prolog! % This is a comment !prolog!";
        let result = translator.translate(source).unwrap();
        assert!(result.contains("// This is a comment"));
    }

    #[test]
    fn test_quantum_stability() {
        let translator = PrologTranslator::new();
        assert!(translator.is_quantum_stable());
        assert!(translator.get_coherence() > QUANTUM_COHERENCE_THRESHOLD);
    }

    #[test]
    fn test_translation_state() {
        let mut translator = PrologTranslator::new();
        assert!(matches!(translator.prolog_state, PrologState::Parsing));
        let _ = translator.translate("!prolog! fact. !prolog!");
        assert!(matches!(translator.prolog_state, PrologState::Complete));
    }

    #[test]
    fn test_unclosed_prolog_block() {
        let mut translator = PrologTranslator::new();
        let source = "!prolog! parent(john, mary).";
        let result = translator.translate(source);
        assert!(result.is_err());
    }
}
