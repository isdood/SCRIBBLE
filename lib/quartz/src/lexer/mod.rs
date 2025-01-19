use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LeftBracket,
    RightBracket,
    Global,
    Function(String),
    Need,
    Takes,
    Gives,
    Create,
    As,
    With,
    When,
    Is,
    For,
    Each,
    In,
    Pipeline,
    Arrow,
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(f64),
    LeftBrace,
    RightBrace,
    Colon,
    Comma,
    Dot,
    Eof,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.peek_char() {
            Some('[') => {
                self.read_char();
                self.parse_block_header()
            },
            Some(']') => {
                self.read_char();
                Token::RightBracket
            },
            Some('>') => {
                if self.peek_next_char() == Some('>') {
                    self.read_char();
                    self.read_char();
                    Token::Pipeline
                } else {
                    self.read_identifier()
                }
            },
            Some(c) if is_letter(c) => self.read_identifier(),
            Some(c) if c.is_ascii_digit() => self.read_number(),
            None => Token::Eof,
            _ => self.read_char_token(),
        }
    }

    fn parse_block_header(&mut self) -> Token {
        let identifier = self.read_until(|c| c == ']' || c == ':');

        match self.peek_char() {
            Some(':') => {
                self.read_char();
                let tag = self.read_until(|c| c == ']');
                if tag == "global" {
                    Token::Function(identifier)
                } else {
                    panic!("Invalid tag: {}", tag)
                }
            }
            Some(']') => {
                if identifier == "global" {
                    Token::Global
                } else {
                    Token::Function(identifier)
                }
            }
            _ => panic!("Invalid block header")
        }
    }

    // Helper methods...
}
