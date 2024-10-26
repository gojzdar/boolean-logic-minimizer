use crate::{
    constant::Constant, operation::Operation, scope::VarScope, term::Term, traits::Simplify,
    var::Var,
};

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
            Expression::Term(_data) => todo!(), //data.simplify(),
            Expression::Operation(_data) => todo!(), // data.simplify(),
        }
    }
    fn simplify_with(self, scope: &VarScope) -> Expression {
        match self {
            Expression::Var(data) => data.simplify_with(scope),
            Expression::Constant(data) => data.simplify_with(scope),
            Expression::Term(_data) => todo!(), // data.simplify_with(scope),
            Expression::Operation(_data) => todo!(), // data.simplify_with(scope),
        }
    }
}
