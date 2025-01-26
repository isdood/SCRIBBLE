#!/bin/bash

# Weave Feature Fix Script v2
# Author: isdood
# Created: 2025-01-25 21:11:21 UTC
# Repository: isdood/scribble
# Description: Fixes module loading and test issues in weave feature

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

fix_weave_feature() {
    cd forge/std || exit 1

    # Update integration tests to fix unused import and failing test
    cat > "tests/integration/weave/mod.rs" << 'EOL'
//! Integration tests for weave module

use spark_std::weave::WeavePattern;
use spark_std::waves::Wave;
use spark_std::weave::integration::WeaveOptimized;
use spark_std::spell::{SpellBlock, SpellParser};

#[test]
fn test_weave_wave_integration() {
    let wave = Wave::new(&[1, 2, 3]);
    let pattern = WeavePattern::new(500).unwrap();
    assert!(wave.apply_weave(&pattern).is_ok());
}

#[test]
fn test_spell_weave_parsing() {
    let spell = "@spell@\n~weave~ = 750\n@end@";
    assert!(SpellParser::validate(spell));
    assert_eq!(SpellParser::parse_weave(spell), Some(750));
}

#[test]
fn test_weave_pattern_validation() {
    // Test boundary conditions
    assert!(WeavePattern::new(1).is_ok());
    assert!(WeavePattern::new(1000).is_ok());
    assert!(WeavePattern::new(0).is_err());
    assert!(WeavePattern::new(1001).is_err());

    // Test pattern distribution
    let pattern = WeavePattern::new(500).unwrap();
    let distribution = pattern.distribute();
    assert!(!distribution.is_empty());
}

#[test]
fn test_spell_block_parsing() {
    // Test valid spell blocks
    let valid_spell = "@spell@\n~weave~ = 500\n@end@";
    let spell_block = SpellBlock::new(valid_spell);
    assert!(SpellParser::validate(valid_spell));
    assert_eq!(spell_block.weave_factor, Some(500));

    // Test invalid spell blocks
    let invalid_spell = "~weave~ = 500";
    let invalid_block = SpellBlock::new(invalid_spell);
    assert!(!SpellParser::validate(invalid_spell));
    assert_eq!(invalid_block.weave_factor, None);
}

#[test]
fn test_spell_block_creation() {
    let spell_content = "@spell@\n~weave~ = 800\nsome code\n@end@";
    let block = SpellBlock::new(spell_content);
    assert_eq!(block.weave_factor, Some(800));
    assert!(block.content.contains("some code"));
}
EOL

    # Update spell/mod.rs to improve SpellBlock implementation
    cat > "src/spell/mod.rs" << 'EOL'
//! Spell parsing and execution module

mod parser;
pub use parser::SpellParser;

/// Represents a parsed spell block
#[derive(Debug)]
pub struct SpellBlock {
    /// Weave factor if specified
    pub weave_factor: Option<u16>,
    /// Raw spell content
    pub content: String,
}

impl SpellBlock {
    /// Creates a new spell block from content
    pub fn new(content: impl Into<String>) -> Self {
        let content = content.into();
        let weave_factor = if SpellParser::validate(&content) {
            SpellParser::parse_weave(&content)
        } else {
            None
        };

        Self {
            weave_factor,
            content,
        }
    }

    /// Checks if this is a valid spell block
    pub fn is_valid(&self) -> bool {
        SpellParser::validate(&self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spell_block_creation() {
        let valid = SpellBlock::new("@spell@\n~weave~ = 500\n@end@");
        assert_eq!(valid.weave_factor, Some(500));
        assert!(valid.is_valid());

        let invalid = SpellBlock::new("invalid");
        assert_eq!(invalid.weave_factor, None);
        assert!(!invalid.is_valid());
    }
}
EOL

    print_purple "✓ Fixed integration tests and SpellBlock implementation"
}

main() {
    print_purple "🧬 Fixing Weave Feature..."
    fix_weave_feature
    print_purple "✨ Weave fixes applied:

Changes:
- Removed unused WeaveParser import
- Fixed spell block parsing test
- Added SpellBlock validation
- Improved SpellBlock creation
- Added more test cases
- Enhanced error handling
- Added block validation method

Run 'cargo test' to verify the fixes!"
}

main
