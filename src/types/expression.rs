use crate::{constant::Constant, operation::Operation, term::Term, var::Var};

pub enum Expression {
    Constant(Constant),
    Var(Var),
    Term(Term),
    Operation(Operation),
}
