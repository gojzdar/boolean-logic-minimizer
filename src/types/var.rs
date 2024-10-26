use crate::{
    constant::Constant,
    expression::Expression,
    scope::{self, VarScope},
    traits::{Eval, Simplify, VarVisibility},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarName {
    name: char,
    subscript: Option<u8>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Var {
    name: VarName,
    negated: bool,
}

impl VarName {
    pub fn new(name: char, subscript: Option<u8>) -> Self {
        VarName { name, subscript }
    }
    pub fn get_string(&self, negated: bool) -> String {
        let mut out = if negated {
            // adds bar over character, to indicate negation
            let letter_modifier = '\u{0304}';
            format!("{}{}", self.name, letter_modifier)
        } else {
            format!("{}", self.name)
        };

        if let Some(subscript) = self.subscript {
            out += decimal_to_subscript(subscript as u32).as_str();
        }

        out
    }
}

impl Var {
    pub fn new(name: char, subscript: Option<u8>, negated: bool) -> Self {
        Var {
            name: VarName::new(name, subscript),
            negated,
        }
    }
    pub fn is_dual(&self, other: &Self) -> bool {
        self.negated != other.negated && self.has_same_name(other)
    }
    pub fn has_same_name(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eval for Var {
    fn evaluate(&self, scope: &VarScope) -> Result<crate::constant::Constant, String> {
        match scope.mappings.get_key_value(&self.name) {
            None => Err(format!(
                "Variable '{}' isn't defined in this scope!",
                self.name.get_string(false)
            )),
            Some((_, constant)) => Ok(*constant),
        }
    }
}

impl VarVisibility for Var {
    fn get_used_variables(&self, varset: &mut std::collections::HashSet<VarName>) {
        varset.insert(self.name);
    }
}

impl Simplify for Var {
    fn simplify(self) -> Expression {
        Expression::Var(self)
    }
    fn simplify_with(self, scope: &VarScope) -> Expression {
        match scope.mappings.get_key_value(&self.name) {
            None => Expression::Var(self),
            Some((_key, value)) => Expression::Constant(*value),
        }
    }
}

impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.get_string(self.negated))
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_naming() {
        let zneg = Var::new('z', None, true);

        assert_eq!(
            zneg,
            Var {
                name: VarName {
                    name: 'z',
                    subscript: None
                },
                negated: true
            }
        );
    }

    #[test]
    fn correct_print() {
        let x3neg = Var::new('x', Some(3), true);
        assert_eq!(x3neg.to_string(), "x̄₃");

        let y = Var::new('y', None, false);
        assert_eq!(y.to_string(), "y");

        let q240neg = Var::new('q', Some(240), true);
        assert_eq!(q240neg.to_string(), "q̄₂₄₀");
    }

    #[test]
    fn sorting() {
        let x3neg = Var::new('x', Some(3), true);
        let x3 = Var::new('x', Some(3), false);

        let y = Var::new('y', None, false);
        let y0 = Var::new('y', Some(0), false);
        let y0neg = Var::new('y', Some(0), true);

        let q240neg = Var::new('q', Some(240), true);

        let mut vec = vec![y0, x3, y0neg, y, x3neg, q240neg];
        vec.sort();
        assert_eq!(vec, [q240neg, x3, x3neg, y, y0, y0neg]);
    }
}
