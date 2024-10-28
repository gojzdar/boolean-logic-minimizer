use crate::{
    constant::Constant,
    operation::Operation,
    scope::VarScope,
    term::Term,
    traits::{Eval, Simplify, VarVisibility},
    var::Var,
};

#[derive(Clone, Debug)]
pub enum Expression {
    Constant(Constant),
    Var(Var),
    Term(Term),
    Operation(Operation),
}

impl Simplify for Expression {
    fn simplify(self) -> Expression {
        match self {
            Expression::Var(data) => data.simplify(),
            Expression::Constant(data) => data.simplify(),
            Expression::Term(data) => data.simplify(),
            Expression::Operation(data) => data.simplify(),
        }
    }
    fn simplify_with(self, scope: &VarScope) -> Expression {
        match self {
            Expression::Var(data) => data.simplify_with(scope),
            Expression::Constant(data) => data.simplify_with(scope),
            Expression::Term(data) => data.simplify_with(scope),
            Expression::Operation(data) => data.simplify_with(scope),
        }
    }
}

impl VarVisibility for Expression {
    fn get_used_variables(&self, varset: &mut std::collections::HashSet<crate::var::VarName>) {
        match self {
            Expression::Var(data) => data.get_used_variables(varset),
            Expression::Constant(data) => data.get_used_variables(varset),
            Expression::Term(data) => data.get_used_variables(varset),
            Expression::Operation(data) => data.get_used_variables(varset),
        }
    }
}

impl Eval for Expression {
    fn evaluate(&self, scope: &VarScope) -> Result<Constant, String> {
        match self {
            Expression::Var(data) => data.evaluate(scope),
            Expression::Constant(data) => data.evaluate(scope),
            Expression::Term(data) => data.evaluate(scope),
            Expression::Operation(data) => data.evaluate(scope),
        }
    }
}
