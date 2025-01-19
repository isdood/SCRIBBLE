use super::lexer::{Lexer, Token};
use super::ast::*;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::Eof,
            peek_token: Token::Eof,
        };

        // Prime the parser
        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn parse_module(&mut self) -> Result<QuartzModule, ParseError> {
        let mut module = QuartzModule {
            global_imports: vec![],
            functions: vec![],
        };

        while !self.current_token_is(Token::Eof) {
            match self.current_token {
                Token::LeftBracket => {
                    match self.peek_token {
                        Token::Global => {
                            module.global_imports = self.parse_global_block()?;
                        }
                        Token::Function(_) => {
                            module.functions.push(self.parse_function()?);
                        }
                        _ => return Err(ParseError::UnexpectedToken),
                    }
                }
                _ => return Err(ParseError::UnexpectedToken),
            }
        }

        Ok(module)
    }

    fn parse_function(&mut self) -> Result<Function, ParseError> {
        // Parse function header
        let (name, is_global) = self.parse_function_header()?;

        // Parse imports
        let imports = self.parse_imports()?;

        // Parse parameters
        let parameters = self.parse_parameters()?;

        // Parse return type
        let return_type = self.parse_return_type()?;

        // Parse function body
        let body = self.parse_block()?;

        Ok(Function {
            name,
            is_global,
            imports,
            parameters,
            return_type,
            body,
        })
    }

    // Additional parsing methods...
}
