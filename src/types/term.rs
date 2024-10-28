use std::env::vars;

use crate::{
    constant::Constant,
    expression::Expression,
    traits::{Eval, Simplify, VarVisibility},
    var::Var,
};

#[derive(Debug, Clone)]
pub struct Term {
    vars: Vec<Var>,
}

impl Term {
    pub fn new_from_vars(mut vars: Vec<Var>) -> Self {
        vars.sort();
        Term { vars }
    }
    pub fn new_from_minterm(len: i32, minterm: u64) -> Self {
        let mut vars = Vec::new();

        for i in 0..len {
            // Check if the i-th bit is set in the minterm
            let bit = (minterm >> (len - 1 - i)) & 1;
            // let name = (b'a' + i as u8) as char; // Assuming variables are named 'a', 'b', 'c', ...
            let negated = bit == 0; // If the bit is 0, the variable is negated

            vars.push(Var::new('x', Some((i + 1) as u8), negated));
        }

        Term { vars }
    }
    pub fn is_neighbor(&self, other: &Self) -> bool {
        if self.vars.len() != other.vars.len() {
            return false;
        }
        let mut has_differed = false;
        for (var1, var2) in self.vars.iter().zip(other.vars.iter()) {
            if !var1.has_same_name(var2) {
                return false;
            }
            if var1.is_dual(var2) {
                if has_differed {
                    return false;
                }
                has_differed = true
            }
        }
        has_differed
    }
    pub fn new_from_neighbors(term1: &Self, term2: &Self) -> Option<Self> {
        if !term1.is_neighbor(term2) {
            return None;
        }

        let mut merged: Vec<Var> = vec![];

        for (e1, e2) in term1.vars.iter().zip(term2.vars.iter()) {
            if e1.is_dual(e2) {
                continue;
            }
            merged.push(*e1);
        }

        Some(Self::new_from_vars(merged))
    }

    pub fn has_var(&self, var: Var) -> bool {
        self.vars.iter().any(|e| var == *e)
    }

    /// Returns None if none are redundant. If first is redundant, it returns 1, otherwise 2.
    pub fn is_one_redundant(term1: &Self, term2: &Self) -> Option<i8> {
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

impl VarVisibility for Term {
    fn get_used_variables(&self, varset: &mut std::collections::HashSet<crate::var::VarName>) {
        for var in self.vars.iter() {
            var.get_used_variables(varset);
        }
    }
}
impl Eval for Term {
    fn evaluate(&self, scope: &crate::scope::VarScope) -> Result<Constant, String> {
        for var in self.vars.iter() {
            match var.evaluate(scope)? {
                Constant::ZERO => return Ok(Constant::ZERO),
                Constant::ONE => continue,
            }
        }
        Ok(Constant::ONE)
    }
}
impl Simplify for Term {
    fn simplify_with(self, scope: &crate::scope::VarScope) -> Expression {
        let mut required_vars: Vec<Var> = vec![];
        for var in self.vars.into_iter() {
            match var.simplify_with(scope) {
                Expression::Constant(Constant::ZERO) => {
                    return Expression::Constant(Constant::ZERO)
                },
                Expression::Constant(Constant::ONE) => continue,
                Expression::Var(v) => required_vars.push(v),
                other => unreachable!("Variable evaluation can only return Constant or the Variable. Happened with: {:?}", other),
            };
        }
        if required_vars.is_empty() {
            Expression::Constant(Constant::ONE)
        } else {
            Expression::Term(Term::new_from_vars(required_vars))
        }
    }
    fn simplify(self) -> Expression {
        // assumming Term is sorted, with non negated vars coming first
        // let mut required_vars: Vec<Var> = vec![];

        let mut redundant_vars = vec![false; self.vars.len()];

        for i in 0..(self.vars.len()) {
            let var1 = self.vars[i];

            if i != self.vars.len() - 1 {
                let var2 = self.vars[i + 1];
                // check for duplicates, dual forms
                if var1 == var2 {
                    redundant_vars[i + 1] = true;
                } else if var1.is_dual(&var2) {
                    // x & ~x = 0
                    return Expression::Constant(Constant::ZERO);
                }
            }
        }
        Expression::Term(Term::new_from_vars(
            self.vars
                .into_iter()
                .enumerate()
                .filter_map(|(i, v)| if !redundant_vars[i] { Some(v) } else { None })
                .collect(),
        ))
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for var in self.vars.iter() {
            write!(f, "{}", var)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}
