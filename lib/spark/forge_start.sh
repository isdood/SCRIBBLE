#!/bin/bash

# Spark Safety Module Setup Script
# Author: isdood
# Created: 2025-01-25 17:21:35 UTC
# Repository: isdood/scribble
# Description: Sets up the Rust-based safety checker with three magic levels

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

check_rust() {
    if ! command -v rustc &> /dev/null; then
        print_purple "❌ Rust is not installed. Please install Rust first."
        exit 1
    fi

    rust_version=$(rustc --version)
    print_purple "✓ Found $rust_version"
}

create_directory_structure() {
    print_purple "🛡️ Creating Safety module structure..."
    mkdir -p forge/safety/src
    mkdir -p forge/safety/tests
}

setup_cargo() {
    cat > forge/safety/Cargo.toml << 'EOL'
[package]
name = "spark-safety"
version = "0.1.0"
edition = "2021"
authors = ["isdood"]
description = "Magical safety checker for the Spark language"

[dependencies]
thiserror = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[dev-dependencies]
criterion = "0.5"
EOL

    cat > forge/safety/src/lib.rs << 'EOL'
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
EOL

    cat > forge/safety/src/main.rs << 'EOL'
use spark_safety::{SafetyChecker, SafetyLevel};

fn main() {
    println!("🛡️ Spark Safety Checker v0.1.0");

    let mut checker = SafetyChecker::new(SafetyLevel::Balanced);

    if let Err(e) = checker.check_spell("unsafe magic") {
        println!("❌ Safety violation detected: {}", e);
    } else {
        println!("✨ Spell checked successfully!");
    }

    let stats = checker.get_stats();
    println!("📊 Safety Level: {}", stats.level);
}
EOL

    cat > forge/safety/tests/safety_tests.rs << 'EOL'
use spark_safety::{SafetyChecker, SafetyLevel, SafetyError};

#[test]
fn test_calm_mode() {
    let mut checker = SafetyChecker::new(SafetyLevel::Calm);
    assert!(checker.check_spell("safe magic").is_ok());
    assert!(checker.check_spell("unsafe magic").is_err());
}

#[test]
fn test_balanced_mode() {
    let mut checker = SafetyChecker::new(SafetyLevel::Balanced);
    assert!(checker.check_spell("safe magic").is_ok());
    assert!(checker.check_spell("unsafe magic with safe_guard").is_ok());
}

#[test]
fn test_wild_mode() {
    let mut checker = SafetyChecker::new(SafetyLevel::Wild);
    assert!(checker.check_spell("unsafe magic").is_ok());
    assert!(checker.check_spell("forbidden_magic").is_err());
}
EOL

    print_purple "✓ Created safety module files"
}

main() {
    print_purple "🛡️ Creating Spark Safety Module..."
    check_rust
    create_directory_structure
    setup_cargo
    print_purple "✨ Safety module created! Run 'cd forge/safety && cargo build' to get started."
}

main
