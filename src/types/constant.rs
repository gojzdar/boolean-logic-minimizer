use crate::{
    expression::Expression,
    scope::VarScope,
    traits::{Eval, Simplify, VarVisibility},
};

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Constant {
    ZERO = 0,
    ONE = 1,
}

impl Constant {
    pub fn negate(self) -> Constant {
        match self {
            Self::ZERO => Constant::ONE,
            Self::ONE => Constant::ZERO,
        }
    }
}

impl From<bool> for Constant {
    fn from(value: bool) -> Self {
        if value {
            Constant::ONE
        } else {
            Constant::ZERO
        }
    }
}

impl VarVisibility for Constant {
    fn get_used_variables(&self, _varset: &mut std::collections::HashSet<crate::var::VarName>) {}
}

impl Eval for Constant {
    fn evaluate(&self, _scope: &VarScope) -> Result<Constant, String> {
        Ok(*self)
    }
}

impl Simplify for Constant {
    fn simplify(self) -> Expression {
        Expression::Constant(self)
    }
    fn simplify_with(self, _scope: &VarScope) -> Expression {
        Expression::Constant(self)
    }
}
