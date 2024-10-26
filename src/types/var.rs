#[derive(Debug, Clone, Copy)]
pub struct Var {
    name: char,
    negated: bool,
}
impl Var {
    pub fn new(name: char, negated: bool) -> Self {
        Var { name, negated }
    }
    pub fn is_dual(&self, other: &Self) -> bool {
        self.negated != other.negated && self.has_same_name(other)
    }
    pub fn has_same_name(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl std::cmp::PartialOrd for Var {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Var {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Comparison first by name, then by negation
        match self.name.cmp(&other.name) {
            // If names are equal, compare by negation
            std::cmp::Ordering::Equal => self.negated.cmp(&other.negated),
            // return the ordering by name
            ordering => ordering,
        }
    }
}
impl std::cmp::Eq for Var {}
impl std::cmp::PartialEq for Var {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.negated == other.negated
    }
}
impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.negated {
            // adds bar over character, to indicate negation
            let letter_modifier = '\u{0304}';
            write!(f, "{}{}", self.name, letter_modifier)
        } else {
            write!(f, "{}", self.name)
        }
    }
}
