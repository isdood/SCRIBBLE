/// HTML Translation Module
/// Last Updated: 2025-01-15 04:15:47 UTC
/// Author: isdood
/// Current User: isdood

use super::TranslationState;
use unstable_matter::{
    Quantum, ScribePrecision, Scribe, QuantumString,
    Vector3D, UnstableDescriptor, QuantumState,
    QUANTUM_COHERENCE_THRESHOLD
};

#[derive(Debug, PartialEq)]
pub enum HtmlBlockState {
    Outside,    // Not in an HTML block
    Starting,   // Found opening !html!
    Inside,     // Processing HTML code
    Ending,     // Found closing !html!
}

#[derive(Debug)]
pub enum HtmlState {
    Parsing,
    Translating,
    Optimizing,
    Verifying,
    Complete,
    Failed,
}

/// Main translator for HTML code
#[derive(Debug)]
pub struct HtmlTranslator {
    state: TranslationState,
    html_state: HtmlState,
    block_state: HtmlBlockState,
    indentation_level: usize,
    quantum_stability: f64,
    current_block: Vec<String>,
    in_script_tag: bool,
    in_style_tag: bool,
}

impl HtmlTranslator {
    pub fn new() -> Self {
        Self {
            state: TranslationState::new(),
            html_state: HtmlState::Parsing,
            block_state: HtmlBlockState::Outside,
            indentation_level: 0,
            quantum_stability: 1.0,
            current_block: Vec::new(),
            in_script_tag: false,
            in_style_tag: false,
        }
    }

    /// Translate HTML code to the target format
    pub fn translate(&mut self, source: &str) -> Result<String, &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state too unstable for translation");
        }

        self.html_state = HtmlState::Translating;
        self.process_code(source)
    }

    /// Process and transform HTML code
    fn process_code(&mut self, source: &str) -> Result<String, &'static str> {
        let mut result = String::new();

        for line in source.lines() {
            match self.block_state {
                HtmlBlockState::Outside => {
                    if line.trim() == "!html!" {
                        self.block_state = HtmlBlockState::Starting;
                        result.push_str("// Begin HTML Translation Block\n");
                    } else {
                        result.push_str(line);
                        result.push('\n');
                    }
                },
                HtmlBlockState::Starting => {
                    self.block_state = HtmlBlockState::Inside;
                    self.current_block.clear();
                    self.current_block.push(line.to_string());
                },
                HtmlBlockState::Inside => {
                    if line.trim() == "!html!" {
                        self.block_state = HtmlBlockState::Ending;
                        let translated = self.process_html_block()?;
                        result.push_str(&translated);
                        result.push_str("// End HTML Translation Block\n");
                        self.block_state = HtmlBlockState::Outside;
                        self.current_block.clear();
                    } else {
                        self.current_block.push(line.to_string());
                    }
                },
                HtmlBlockState::Ending => {
                    self.block_state = HtmlBlockState::Outside;
                }
            }
        }

        if self.block_state != HtmlBlockState::Outside {
            return Err("Unclosed HTML block - missing !html! terminator");
        }

        self.html_state = HtmlState::Complete;
        Ok(result)
    }

    /// Process a complete HTML block
    fn process_html_block(&mut self) -> Result<String, &'static str> {
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

    /// Process a single line of HTML code
    fn process_line(&mut self, line: &str) -> Result<String, &'static str> {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed == "!html!" {
            return Ok(String::new());
        }

        // Update tag tracking
        if trimmed.contains("<script") {
            self.in_script_tag = true;
        } else if trimmed.contains("</script>") {
            self.in_script_tag = false;
        } else if trimmed.contains("<style") {
            self.in_style_tag = true;
        } else if trimmed.contains("</style>") {
            self.in_style_tag = false;
        }

        let translated = match trimmed {
            s if s.starts_with("<!--") => self.translate_comment(s),
            s if self.in_script_tag => self.translate_script(s),
            s if self.in_style_tag => self.translate_style(s),
            s if s.starts_with("<") => self.translate_tag(s),
            _ => Ok(String::from(trimmed)),
        }?;

        Ok(format!("{}{}", "    ".repeat(self.indentation_level), translated))
    }

    fn translate_comment(&self, line: &str) -> Result<String, &'static str> {
        Ok(line.replace("<!--", "//").replace("-->", ""))
    }

    fn translate_script(&self, line: &str) -> Result<String, &'static str> {
        Ok(format!("js!{{ {} }}", line))
    }

    fn translate_style(&self, line: &str) -> Result<String, &'static str> {
        Ok(format!("css!{{ {} }}", line))
    }

    fn translate_tag(&self, line: &str) -> Result<String, &'static str> {
        Ok(format!("html!{{ {} }}", line))
    }
}

impl Quantum for HtmlTranslator {
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

impl Scribe for HtmlTranslator {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("HtmlTranslator[");
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
    fn test_basic_html_translation() {
        let mut translator = HtmlTranslator::new();
        let source = "!html! <div>Hello World</div> !html!";
        let result = translator.translate(source).unwrap();
        assert!(result.contains("html!{ <div>Hello World</div> }"));
    }

    #[test]
    fn test_unclosed_html_block() {
        let mut translator = HtmlTranslator::new();
        let source = "!html! <div>Hello World";
        let result = translator.translate(source);
        assert!(result.is_err());
    }
}
