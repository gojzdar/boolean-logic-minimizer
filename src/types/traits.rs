use std::collections::HashSet;

use crate::{constant::Constant, expression::Expression, scope::VarScope, var::VarName};

pub trait Eval {
    /// Given values of all variables, evaluate the expression.
    fn evaluate(&self, scope: &VarScope) -> Result<Constant, String>;
}

pub trait Simplify {
    /// Given values of SOME variables, simplify the expression
    fn simplify_with(self, scope: &VarScope) -> Expression;
    /// Simplify, using univesal rules only
    fn simplify(self) -> Expression;
}

pub trait VarVisibility {
    fn get_used_variables(&self, varset: &mut HashSet<VarName>);
}

pub trait Operator: Eval + Simplify + VarVisibility {}
