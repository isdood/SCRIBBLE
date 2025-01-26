//! Spell block parser implementation

use crate::weave::WeaveParser;

/// Parser for spell blocks
#[derive(Debug)]
pub struct SpellParser;

impl SpellParser {
    /// Parses weave declarations in spell blocks
    pub fn parse_weave(spell: &str) -> Option<u16> {
        spell.lines()
            .find(|line| line.contains("~weave~"))
            .and_then(WeaveParser::parse)
    }

    /// Validates a spell block
    pub fn validate(spell: &str) -> bool {
        spell.starts_with("@spell@") && spell.ends_with("@end@")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spell_parser() {
        let spell = "@spell@\n~weave~ = 500\n@end@";
        assert_eq!(SpellParser::parse_weave(spell), Some(500));
    }

    #[test]
    fn test_spell_validation() {
        assert!(SpellParser::validate("@spell@\n~weave~ = 500\n@end@"));
        assert!(!SpellParser::validate("invalid"));
    }
}
