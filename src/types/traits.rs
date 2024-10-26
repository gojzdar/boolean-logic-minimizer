use std::{
    collections::{HashMap, HashSet},
    thread::Scope,
};

use crate::{constant::Constant, expression::Expression, scope::VarScope, var::VarName};

pub trait Eval {
    /// Given values of all variables, evaluate the expression.
    fn evaluate(&self, scope: Scope) -> Result<Constant, String>;
}

pub trait Simplify {
    /// Given values of SOME variables, simplify the expression
    fn simplify_with(&mut self, scope: VarScope);
    /// Simplify, using univesal rules only
    fn simplify(&mut self);
}

pub trait VarVisibility {
    fn get_used_variables(&self, varset: &mut HashSet<VarName>);
}

pub trait Operator: Eval + Simplify + VarVisibility {}
