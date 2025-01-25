use crate::compiler::safety::SafetyLevel;
use std::path::Path;

pub struct SparkParser {
    safety: SafetyLevel,
}

impl SparkParser {
    pub fn new() -> Self {
        Self {
            safety: SafetyLevel::default(),
        }
    }

    pub fn parse_file(&mut self, path: &Path) -> Result<(), String> {
        if !path.exists() {
            return Err(format!("File not found: {}", path.display()));
        }
        println!("Parsing {:?} with {:?} safety", path, self.safety);
        Ok(())
    }
}
