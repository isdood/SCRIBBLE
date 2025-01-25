use std::path::Path;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetyLevel {
    Calm,      // Maximum safety - strict compile-time checks
    Balanced,  // Mixed safety - some runtime checks allowed
    Wild,      // Minimal safety - mostly runtime checks
}

impl Default for SafetyLevel {
    fn default() -> Self {
        SafetyLevel::Calm  // Default to maximum safety
    }
}

impl FromStr for SafetyLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "calm" => Ok(SafetyLevel::Calm),
            "balanced" => Ok(SafetyLevel::Balanced),
            "wild" => Ok(SafetyLevel::Wild),
            _ => Err(format!("Unknown safety level: {}. Use calm, balanced, or wild", s))
        }
    }
}

pub struct ForgeFeatures {
    file_safety: SafetyLevel,
    module_path: String,
    function_safety: HashMap<String, SafetyLevel>,
}

impl ForgeFeatures {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            file_safety: SafetyLevel::default(),
            module_path: path.as_ref().to_string_lossy().into_owned(),
            function_safety: HashMap::new(),
        }
    }

    pub fn parse_features(&mut self, source: &str) -> Result<(), String> {
        for line in source.lines() {
            let line = line.trim();
            
            if line.starts_with("~forge~") {
                self.parse_safety_level(line, None)?;
            } else if line.contains("~forge~") && line.contains("fn") {
                if let Some(fn_name) = self.extract_function_name(line) {
                    self.parse_safety_level(line, Some(fn_name))?;
                }
            }
        }
        Ok(())
    }

    fn extract_function_name(&self, line: &str) -> Option<String> {
        line.split("fn")
            .nth(1)?
            .split('(')
            .next()
            .map(|s| s.trim().to_string())
    }

    fn parse_safety_level(&mut self, line: &str, fn_name: Option<String>) -> Result<(), String> {
        let level = line.split('=')
            .nth(1)
            .ok_or_else(|| "Invalid forge feature syntax".to_string())?
            .trim()
            .parse()?;

        match fn_name {
            Some(name) => { self.function_safety.insert(name, level); }
            None => self.file_safety = level,
        }
        Ok(())
    }

    pub fn get_safety_level(&self, fn_name: &str) -> SafetyLevel {
        self.function_safety
            .get(fn_name)
            .copied()
            .unwrap_or(self.file_safety)
    }
}
