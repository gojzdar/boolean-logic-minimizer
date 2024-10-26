use std::thread::Scope;

use crate::traits::{Eval, Simplify, VarVisibility};

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy)]
pub enum Constant {
    ZERO = 0,
    ONE = 1,
}

impl Constant {}

impl VarVisibility for Constant {
    fn get_used_variables(&self, _varset: &mut std::collections::HashSet<crate::var::VarName>) {}
}

impl Eval for Constant {
    fn evaluate(&self, _scope: Scope) -> Result<Constant, String> {
        Ok(*self)
    }
}

impl Simplify for Constant {
    fn simplify(&mut self) {}
    fn simplify_with(&mut self, _scope: crate::scope::VarScope) {}
}
