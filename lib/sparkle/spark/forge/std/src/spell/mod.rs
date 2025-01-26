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
