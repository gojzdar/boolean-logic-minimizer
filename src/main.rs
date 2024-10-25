use std::{cmp::Ordering, env::args, fmt::Display};

#[derive(Debug, Clone, Copy)]
struct Var {
    name: char,
    negated: bool,
}

#[derive(Debug, Clone)]
struct Term {
    vars: Vec<Var>,
}

impl Var {
    fn new(name: char, negated: bool) -> Self {
        Var { name, negated }
    }
    fn is_dual(&self, other: Self) -> bool {
        self.name == other.name && self.negated != other.negated
    }
}

impl Term {
    fn new_from_vars(mut vars: Vec<Var>) -> Self {
        vars.sort();
        Term { vars }
    }
    fn new_from_minterm(len: i32, minterm: u64) -> Self {
        let mut vars = Vec::new();

        for i in 0..len {
            // Check if the i-th bit is set in the minterm
            let bit = (minterm >> (len - 1 - i)) & 1;
            let name = (b'a' + i as u8) as char; // Assuming variables are named 'a', 'b', 'c', ...
            let negated = bit == 0; // If the bit is 0, the variable is negated

            vars.push(Var::new(name, negated));
        }

        Term { vars }
    }
    fn is_neighbor(&self, other: &Self) -> bool {
        if self.vars.len() != other.vars.len() {
            return false;
        }
        let mut has_differed = false;
        for (var1, var2) in self.vars.iter().zip(other.vars.iter()) {
            if var1.name != var2.name {
                return false;
            }
            if var1.is_dual(*var2) {
                if has_differed {
                    return false;
                }
                has_differed = true
            }
        }
        has_differed
    }
    fn new_from_neighbors(term1: &Self, term2: &Self) -> Option<Self> {
        if !term1.is_neighbor(term2) {
            return None;
        }

        let mut merged: Vec<Var> = vec![];

        for (e1, e2) in term1.vars.iter().zip(term2.vars.iter()) {
            if e1.negated != e2.negated {
                continue;
            }
            merged.push(*e1);
        }

        Some(Self::new_from_vars(merged))
    }

    fn has_var(&self, var: Var) -> bool {
        self.vars.iter().any(|e| var == *e)
    }

    /// Returns None if none are redundant. If first is redundant, it returns 1, otherwise 2.
    fn is_one_redundant(term1: &Self, term2: &Self) -> Option<i8> {
        let (shorter, longer, swapped) = if term1.vars.len() <= term2.vars.len() {
            (term1, term2, false)
        } else {
            (term2, term1, true)
        };

        for var in shorter.vars.iter() {
            if !longer.has_var(*var) {
                return None;
            }
        }

        if !swapped {
            // first is shorter, therefore the second is redundant
            Some(2)
        } else {
            Some(1)
        }
    }
}

fn trim_redundant_terms(terms: &[Term]) -> Vec<Term> {
    let mut is_redundant: Vec<bool> = vec![false; terms.len()];
    for (ind1, term1) in terms.iter().enumerate() {
        if is_redundant[ind1] {
            continue;
        }
        for (ind2, term2) in terms.iter().enumerate().skip(ind1 + 1) {
            if is_redundant[ind2] {
                continue;
            }

            if let Some(redundant) = Term::is_one_redundant(term1, term2) {
                println!(
                    "Between '{}' and '{}' the #{} is redundant",
                    term1, term2, redundant
                );
                if redundant == 1 {
                    is_redundant[ind1] = true;
                } else {
                    is_redundant[ind2] = true;
                }
            }
        }
    }

    terms
        .iter()
        .enumerate()
        .filter(|(i, _)| !is_redundant[*i]) // Filter where is_redundant[i] is false
        .map(|(_, term)| term.clone())
        .collect()
}

fn step_quine(terms: &[Term]) -> (Vec<Term>, usize) {
    let mut merges = 0;
    let mut merged_pairs: Vec<Term> = vec![];
    let mut has_been_used: Vec<bool> = vec![false; terms.len()];

    for (ind1, term1) in terms.iter().enumerate() {
        for (ind2, term2) in terms.iter().enumerate().skip(ind1) {
            match Term::new_from_neighbors(term1, term2) {
                None => {}
                Some(merged) => {
                    has_been_used[ind1] = true;
                    has_been_used[ind2] = true;
                    println!("Merged '{}' + '{}' => '{}'", term1, term2, merged);
                    merged_pairs.push(merged);
                    merges += 1;
                }
            }
        }
        if !has_been_used[ind1] {
            merged_pairs.push(term1.clone());
        }
    }

    (trim_redundant_terms(&merged_pairs), merges)
}

fn minimize(terms: &[Term]) -> Vec<Term> {
    let (mut iteration, mut merges) = step_quine(terms);
    while merges > 0 {
        (iteration, merges) = step_quine(&iteration);
    }
    iteration
}

fn print_terms(terms: &[Term]) {
    for term in terms.iter() {
        print!("{}, ", term);
    }
    println!();
}

fn main() {
    let cmd_args: Vec<String> = args().collect();
    println!(r#"argument structure: <number of variables> <minterm [minterm [...]]>"#);
    let n = cmd_args[1]
        .parse::<i32>()
        .expect("First argument must be <number of variables>");
    let max_minterm = (1 << n) - 1;
    let mut terms: Vec<Term> = vec![];

    for arg in cmd_args.iter().skip(2) {
        let minterm: u64 = arg.parse::<u64>().expect("Minterm is not a number!");
        if minterm > max_minterm {
            eprintln!(
                "Minterm {} cannot be represented with {} variables (0 <= m <= {})",
                minterm, n, max_minterm
            );
            return;
        }

        let term = Term::new_from_minterm(n, minterm);
        println!("Minterm {} -> {}", minterm, term);
        terms.push(term);
    }
    print_terms(&terms);
    print_terms(&minimize(&terms));
}

impl PartialOrd for Var {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Var {
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
impl Eq for Var {}
impl PartialEq for Var {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.negated == other.negated
    }
}
impl Display for Var {
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

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for var in self.vars.iter() {
            write!(f, "{}", var)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}
