/// Go Translation Module
/// Last Updated: 2025-01-15 04:28:36 UTC
/// Author: isdood
/// Current User: isdood

use super::TranslationState;
use unstable_matter::{
    Quantum, ScribePrecision, Scribe, QuantumString,
    Vector3D, UnstableDescriptor, QuantumState,
    QUANTUM_COHERENCE_THRESHOLD
};

#[derive(Debug, PartialEq)]
pub enum GoBlockState {
    Outside,    // Not in a Go block
    Starting,   // Found opening !go!
    Inside,     // Processing Go code
    Ending,     // Found closing !go!
}

#[derive(Debug)]
pub enum GoState {
    Parsing,
    Translating,
    Optimizing,
    Verifying,
    Complete,
    Failed,
}

#[derive(Debug)]
pub struct PackageInfo {
    name: String,
    imports: Vec<String>,
}

/// Main translator for Go code
#[derive(Debug)]
pub struct GoTranslator {
    state: TranslationState,
    go_state: GoState,
    block_state: GoBlockState,
    indentation_level: usize,
    quantum_stability: f64,
    current_block: Vec<String>,
    package_info: PackageInfo,
    in_struct_def: bool,
    in_interface_def: bool,
}

impl GoTranslator {
    pub fn new() -> Self {
        Self {
            state: TranslationState::new(),
            go_state: GoState::Parsing,
            block_state: GoBlockState::Outside,
            indentation_level: 0,
            quantum_stability: 1.0,
            current_block: Vec::new(),
            package_info: PackageInfo {
                name: String::new(),
                imports: Vec::new(),
            },
            in_struct_def: false,
            in_interface_def: false,
        }
    }

    /// Translate Go code to the target format
    pub fn translate(&mut self, source: &str) -> Result<String, &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state too unstable for translation");
        }

        self.go_state = GoState::Translating;
        self.process_code(source)
    }

    /// Process and transform Go code
    fn process_code(&mut self, source: &str) -> Result<String, &'static str> {
        let mut result = String::new();

        for line in source.lines() {
            match self.block_state {
                GoBlockState::Outside => {
                    if line.trim() == "!go!" {
                        self.block_state = GoBlockState::Starting;
                        result.push_str("// Begin Go Translation Block\n");
                    } else {
                        result.push_str(line);
                        result.push('\n');
                    }
                },
                GoBlockState::Starting => {
                    self.block_state = GoBlockState::Inside;
                    self.current_block.clear();
                    self.current_block.push(line.to_string());
                },
                GoBlockState::Inside => {
                    if line.trim() == "!go!" {
                        self.block_state = GoBlockState::Ending;
                        let translated = self.process_go_block()?;
                        result.push_str(&translated);
                        result.push_str("// End Go Translation Block\n");
                        self.block_state = GoBlockState::Outside;
                        self.current_block.clear();
                    } else {
                        self.current_block.push(line.to_string());
                    }
                },
                GoBlockState::Ending => {
                    self.block_state = GoBlockState::Outside;
                }
            }
        }

        if self.block_state != GoBlockState::Outside {
            return Err("Unclosed Go block - missing !go! terminator");
        }

        self.go_state = GoState::Complete;
        Ok(result)
    }

    /// Process a complete Go block
    fn process_go_block(&mut self) -> Result<String, &'static str> {
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

    /// Process a single line of Go code
    fn process_line(&mut self, line: &str) -> Result<String, &'static str> {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed == "!go!" {
            return Ok(String::new());
        }

        // Update state tracking
        if trimmed.starts_with("type") && trimmed.contains("struct") {
            self.in_struct_def = true;
        } else if trimmed.starts_with("type") && trimmed.contains("interface") {
            self.in_interface_def = true;
        } else if trimmed == "}" {
            self.in_struct_def = false;
            self.in_interface_def = false;
        }

        let translated = match trimmed {
            s if s.starts_with("package ") => self.translate_package(s),
            s if s.starts_with("import ") => self.translate_import(s),
            s if s.starts_with("func ") => self.translate_function(s),
            s if s.starts_with("type ") => self.translate_type(s),
            s if s.starts_with("var ") => self.translate_var(s),
            s if s.starts_with("const ") => self.translate_const(s),
            s if s.starts_with("//") => self.translate_comment(s),
            _ => Ok(String::from(trimmed)),
        }?;

        Ok(format!("{}{}", "    ".repeat(self.indentation_level), translated))
    }

    fn translate_package(&mut self, line: &str) -> Result<String, &'static str> {
        let pkg_name = line.trim_start_matches("package ").trim();
        self.package_info.name = pkg_name.to_string();
        Ok(format!("mod {} {{", pkg_name))
    }

    fn translate_import(&mut self, line: &str) -> Result<String, &'static str> {
        let import = line.trim_start_matches("import ").trim_matches('"');
        self.package_info.imports.push(import.to_string());
        Ok(format!("use {};", import))
    }

    fn translate_function(&mut self, line: &str) -> Result<String, &'static str> {
        let fn_def = line.trim_start_matches("func ").trim_end_matches('{').trim();
        if fn_def.contains(")") {
            // Method
            let parts: Vec<&str> = fn_def.splitn(2, ")").collect();
            if parts.len() == 2 {
                Ok(format!("impl {} {{ fn {} {{", parts[0].trim_start_matches("("), parts[1]))
            } else {
                Ok(format!("fn {} {{", fn_def))
            }
        } else {
            // Regular function
            Ok(format!("fn {} {{", fn_def))
        }
    }

    fn translate_type(&mut self, line: &str) -> Result<String, &'static str> {
        let type_def = line.trim_start_matches("type ").trim();
        if type_def.contains("struct") {
            Ok(format!("pub struct {} {{", type_def.replace("struct", "")))
        } else if type_def.contains("interface") {
            Ok(format!("pub trait {} {{", type_def.replace("interface", "")))
        } else {
            Ok(format!("type {};", type_def))
        }
    }

    fn translate_var(&mut self, line: &str) -> Result<String, &'static str> {
        Ok(format!("let {};", line.trim_start_matches("var ")))
    }

    fn translate_const(&mut self, line: &str) -> Result<String, &'static str> {
        Ok(format!("const {};", line.trim_start_matches("const ")))
    }

    fn translate_comment(&mut self, line: &str) -> Result<String, &'static str> {
        Ok(String::from(line))
    }
}

impl Quantum for GoTranslator {
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

impl Scribe for GoTranslator {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("GoTranslator[");
        self.state.scribe(precision, output);
        output.push_str(", package=");
        output.push_str(&self.package_info.name);
        output.push_str(", stability=");
        output.push_f64(self.quantum_stability, precision.decimal_places());
        output.push_char(']');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_go_translation() {
        let mut translator = GoTranslator::new();
        let source = "!go!\npackage main\n\nfunc main() { }\n!go!";
        let result = translator.translate(source).unwrap();
        assert!(result.contains("mod main"));
        assert!(result.contains("fn main()"));
    }

    #[test]
    fn test_unclosed_go_block() {
        let mut translator = GoTranslator::new();
        let source = "!go!\npackage main\n\nfunc main() {";
        let result = translator.translate(source);
        assert!(result.is_err());
    }
}
