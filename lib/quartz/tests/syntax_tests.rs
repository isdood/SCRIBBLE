#[cfg(test)]
mod tests {
    use super::lexer::Lexer;
    use super::parser::Parser;

    #[test]
    fn test_basic_syntax() {
        let input = "[global] need quantum::core [end global]";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let module = parser.parse_module().expect("Failed to parse module");

        assert_eq!(module.global_imports.len(), 1);
    }
}
