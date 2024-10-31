use std::{char, iter::Peekable, str::Chars};

use super::lexeme::Lexeme;

pub struct Scanner<'a> {
    input: &'a str,
    iter: Peekable<std::iter::Enumerate<Chars<'a>>>,
    errors: Vec<String>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Scanner<'a> {
        Scanner {
            input,
            iter: input.chars().enumerate().peekable(),
            errors: vec![],
        }
    }

    pub fn scan(&mut self) -> Vec<Lexeme> {
        let mut lexemes = vec![];
        while let Some(lex) = self.next_lexeme() {
            if lex.is_empty() {
                continue;
            }
            println!("Lex: '{}'", lex.str_from(self.input));
            lexemes.push(lex);
        }

        lexemes
    }

    pub fn get_errors(&self) -> &[String] {
        &self.errors
    }
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn next_if<F>(&mut self, predicate: F) -> Option<(usize, char)>
    where
        F: FnOnce(char) -> bool,
    {
        // Use the underlying iterator's next_if method
        if let Some(&(_index, next_char)) = self.iter.peek() {
            if predicate(next_char) {
                return self.iter.next(); // Consume and return the item
            }
        }
        None // Return None if it doesn't match
    }

    fn next_lexeme(&mut self) -> Option<Lexeme> {
        let (start, current) = self.iter.next()?;
        if matches!(current, |'~'| '&' | '|' | '^') {
            // operators
            return Some(Lexeme::new(start, 1));
        }

        if matches!(current, '[' | ']' | '(' | ')') {
            // brackets
            return Some(Lexeme::new(start, 1));
        }

        if current.is_numeric() {
            return if matches!(current, '0' | '1') {
                Some(Lexeme::new(start, 1))
            } else {
                self.errors
                    .push(format!("Numerical character '{}' is not allowed!", current));
                Some(Lexeme::new_empty(start))
            };
        }

        if current.is_whitespace() {
            return Some(Lexeme::new_empty(start));
        }

        if !current.is_alphabetic() {
            self.errors.push(format!(
                "Character '{}' is neither whitespace, nor numeric!",
                current
            ));
            return Some(Lexeme::new_empty(start));
        }

        // must be alphabetic now

        let mut len = 1;
        while let Some((_, _)) = self.next_if(|c| c.is_numeric()) {
            len += 1;
        }

        Some(Lexeme::new(start, len))
    }
}
