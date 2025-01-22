//! Pattern Recognizer Implementation
//! Created: 2025-01-22
//! Author: isdood

pub struct PatternRecognizer {
    patterns: Vec<String>,
}

impl PatternRecognizer {
    pub fn new() -> Self {
        PatternRecognizer {
            patterns: Vec::new(),
        }
    }

    pub fn recognize(&mut self, pattern: String) {
        self.patterns.push(pattern);
    }

    pub fn get_patterns(&self) -> &Vec<String> {
        &self.patterns
    }
}
