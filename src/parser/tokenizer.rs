use std::u8;

use crate::var::VarName;

use super::lexeme::Lexeme;

pub struct Tokenizer {
    errors: Vec<String>,
}

#[derive(Debug)]
pub enum Token {
    ParenL,
    ParenR,
    BracketL,
    BracketR,

    // operators
    Tilde,
    Ampersand,
    Pipe,
    Caret,

    // constants
    C0,
    C1,

    // identifier
    Identifier(VarName),
}

impl Tokenizer {
    pub fn new() -> Self {
        Self { errors: vec![] }
    }

    pub fn tokenize(&mut self, lexemes: Vec<Lexeme>, source: &str) -> Vec<Token> {
        let mut out = vec![];
        for lex in lexemes.iter() {
            if let Some(token) = self.to_token(lex, source) {
                out.push(token);
            }
        }
        out
    }

    pub fn get_errors(&self) -> &[String] {
        &self.errors
    }
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn to_token(&mut self, lexeme: &Lexeme, source: &str) -> Option<Token> {
        let str = lexeme.str_from(source);

        if str.is_empty() {
            return None;
        }

        Some(match str {
            "(" => Token::ParenL,
            ")" => Token::ParenR,
            "[" => Token::BracketL,
            "]" => Token::BracketR,

            // operators
            "~" => Token::Tilde,
            "&" => Token::Ampersand,
            "|" => Token::Pipe,
            "^" => Token::Caret,

            // constants
            "0" => Token::C0,
            "1" => Token::C1,

            other => return self.identifier_to_token(lexeme, other),
        })
    }

    fn identifier_to_token(&mut self, lexeme: &Lexeme, identifier: &str) -> Option<Token> {
        if identifier.is_empty() {
            return None;
        }

        let mut iter = identifier.chars();

        let name = iter.next().unwrap();

        if !name.is_alphabetic() {
            self.errors.push(format!(
                "Tokenizer error: Lexeme '{:?}' representing '{}' is not a valid identifier!",
                lexeme, identifier
            ));
            return None;
        }

        let mut has_any_digit = false;

        let mut number = 0;
        for digit in iter {
            if !digit.is_numeric() {
                self.errors.push(format!(
                    "Tokenizer error: Lexeme '{:?}' representing '{}' does not have a valid number!",
                    lexeme, identifier
                ));
                return None;
            }
            has_any_digit = true;

            number = number * 10 + digit.to_digit(10).unwrap();
        }

        if number <= u8::MAX as u32 {
            let subscript = if has_any_digit {
                Some(number as u8)
            } else {
                None
            };
            return Some(Token::Identifier(VarName::new(name, subscript)));
        }

        self.errors.push(format!(
            "Tokenizer error: Lexeme '{:?}' representing '{}', an identifier has a subscript that can't fit into u8!",
            lexeme, identifier
        ));
        None
    }
}
