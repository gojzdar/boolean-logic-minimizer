use core::str;

#[derive(Debug, Clone)]
pub struct Lexeme {
    pub start: usize,
    pub len: usize,
}

impl Lexeme {
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn new(start: usize, len: usize) -> Self {
        Lexeme { start, len }
    }
    pub fn new_empty(start: usize) -> Self {
        Lexeme { start, len: 0 }
    }
    pub fn str_from<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start..(self.start + self.len)]
    }
}
