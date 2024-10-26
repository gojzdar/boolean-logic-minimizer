#[derive(Debug, Clone, Copy)]
pub struct Var {
    name: char,
    subscript: Option<u8>,
    negated: bool,
}
impl Var {
    pub fn new(name: char, negated: bool, subscript: Option<u8>) -> Self {
        Var {
            name,
            negated,
            subscript,
        }
    }
    pub fn is_dual(&self, other: &Self) -> bool {
        self.negated != other.negated && self.has_same_name(other)
    }
    pub fn has_same_name(&self, other: &Self) -> bool {
        self.name == other.name && self.subscript == other.subscript
    }
}

impl std::cmp::PartialOrd for Var {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Var {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name
            .cmp(&other.name)
            .then_with(|| self.subscript.cmp(&other.subscript))
            .then_with(|| self.negated.cmp(&other.negated))
    }
}
impl std::cmp::Eq for Var {}
impl std::cmp::PartialEq for Var {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.negated == other.negated
            && self.subscript == other.subscript
    }
}
impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.negated {
            // adds bar over character, to indicate negation
            let letter_modifier = '\u{0304}';
            write!(f, "{}{}", self.name, letter_modifier)?
        } else {
            write!(f, "{}", self.name)?
        }
        if let Some(subscript) = self.subscript {
            write!(f, "{}", decimal_to_subscript(subscript as u32))?;
        }
        Ok(())
    }
}

fn decimal_to_subscript(num: u32) -> String {
    let subscripts = [
        '\u{2080}', // Subscript 0
        '\u{2081}', // Subscript 1
        '\u{2082}', // Subscript 2
        '\u{2083}', // Subscript 3
        '\u{2084}', // Subscript 4
        '\u{2085}', // Subscript 5
        '\u{2086}', // Subscript 6
        '\u{2087}', // Subscript 7
        '\u{2088}', // Subscript 8
        '\u{2089}', // Subscript 9
    ];

    let mut result = String::new();
    let digits = num.to_string();

    for digit in digits.chars() {
        if let Some(d) = digit.to_digit(10) {
            result.push(subscripts[d as usize]);
        }
    }

    result
}
