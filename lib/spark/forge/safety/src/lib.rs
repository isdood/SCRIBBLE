use thiserror::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetyLevel {
    Calm,    // Most restrictive, perfect for production
    Balanced, // Good mix of safety and flexibility
    Wild,     // Minimal restrictions, use with caution
}

impl fmt::Display for SafetyLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SafetyLevel::Calm => write!(f, "calm"),
            SafetyLevel::Balanced => write!(f, "balanced"),
            SafetyLevel::Wild => write!(f, "wild"),
        }
    }
}

#[derive(Error, Debug)]
pub enum SafetyError {
    #[error("Dangerous magic detected: {0}")]
    DangerousMagic(String),
    #[error("Unstable spell combination: {0}")]
    UnstableSpell(String),
    #[error("Forbidden enchantment: {0}")]
    ForbiddenEnchantment(String),
}

pub struct SafetyChecker {
    level: SafetyLevel,
    enchantments_count: usize,
    wild_magic_detected: bool,
}

impl SafetyChecker {
    pub fn new(level: SafetyLevel) -> Self {
        Self {
            level,
            enchantments_count: 0,
            wild_magic_detected: false,
        }
    }

    pub fn check_spell(&mut self, spell: &str) -> Result<(), SafetyError> {
        match self.level {
            SafetyLevel::Calm => self.check_calm(spell),
            SafetyLevel::Balanced => self.check_balanced(spell),
            SafetyLevel::Wild => self.check_wild(spell),
        }
    }

    fn check_calm(&mut self, spell: &str) -> Result<(), SafetyError> {
        // Most restrictive checks
        if spell.contains("unsafe") {
            return Err(SafetyError::DangerousMagic(
                "Unsafe magic not allowed in calm mode".into()
            ));
        }
        if spell.contains("**") && self.enchantments_count > 100 {
            return Err(SafetyError::UnstableSpell(
                "Too many path operations in calm mode".into()
            ));
        }
        self.enchantments_count += 1;
        Ok(())
    }

    fn check_balanced(&mut self, spell: &str) -> Result<(), SafetyError> {
        // Medium restrictions
        if spell.contains("unsafe") && !spell.contains("safe_guard") {
            return Err(SafetyError::DangerousMagic(
                "Unsafe magic requires safe_guard in balanced mode".into()
            ));
        }
        if spell.contains("**") && self.enchantments_count > 1000 {
            return Err(SafetyError::UnstableSpell(
                "Too many path operations in balanced mode".into()
            ));
        }
        self.enchantments_count += 1;
        Ok(())
    }

    fn check_wild(&mut self, spell: &str) -> Result<(), SafetyError> {
        // Minimal restrictions
        if spell.contains("forbidden_magic") {
            return Err(SafetyError::ForbiddenEnchantment(
                "Even wild mode has some limits!".into()
            ));
        }
        self.wild_magic_detected = spell.contains("unsafe");
        self.enchantments_count += 1;
        Ok(())
    }

    pub fn get_stats(&self) -> SafetyStats {
        SafetyStats {
            level: self.level,
            enchantments_count: self.enchantments_count,
            wild_magic_detected: self.wild_magic_detected,
        }
    }
}

#[derive(Debug)]
pub struct SafetyStats {
    pub level: SafetyLevel,
    pub enchantments_count: usize,
    pub wild_magic_detected: bool,
}
