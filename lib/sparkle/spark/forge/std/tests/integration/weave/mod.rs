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
