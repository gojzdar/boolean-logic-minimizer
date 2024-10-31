use crate::{
    constant::Constant,
    expression::Expression,
    scope::VarScope,
    traits::{Eval, Operator, Simplify, VarVisibility},
};

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug)]
pub enum Operation {
    NOT(Box<Expression>),
    AND(Vec<Expression>),
    OR(Vec<Expression>),
    XOR(Vec<Expression>),
}

impl Operator for Operation {
    fn is_commutative(&self) -> bool {
        // NOT, AND, OR and XOR are all commutative
        true
    }
    fn is_associative(&self) -> bool {
        // NOT, AND, OR and XOR are all associative
        true
    }
}

impl VarVisibility for Operation {
    fn get_used_variables(&self, varset: &mut std::collections::HashSet<crate::var::VarName>) {
        match self {
            Self::NOT(exp) => exp.get_used_variables(varset),
            Self::AND(vecexp) => vecexp.iter().for_each(|exp| exp.get_used_variables(varset)),
            Self::OR(vecexp) => vecexp.iter().for_each(|exp| exp.get_used_variables(varset)),
            Self::XOR(vecexp) => vecexp.iter().for_each(|exp| exp.get_used_variables(varset)),
        }
    }
}

impl Eval for Operation {
    fn evaluate(
        &self,
        scope: &crate::scope::VarScope,
    ) -> Result<crate::constant::Constant, String> {
        match self {
            Self::NOT(exp) => Ok(exp.evaluate(scope)?.negate()),
            Self::AND(vecexp) => Self::evaluate_and(vecexp, scope),
            Self::OR(vecexp) => Self::evaluate_or(vecexp, scope),
            Self::XOR(vecexp) => Self::evaluate_xor(vecexp, scope),
        }
    }
}

impl Simplify for Operation {
    fn simplify(self) -> Expression {
        match self {
            Self::NOT(data) => Operation::simplify_not(*data),
            Self::AND(data) => Self::simplify_and(data),
            Self::OR(data) => Self::simplify_or(data),
            Self::XOR(data) => Self::simplify_xor(data),
        }
    }
    fn simplify_with(self, scope: &crate::scope::VarScope) -> Expression {
        match self {
            Self::NOT(data) => Operation::simplify_not_with(*data, scope),
            Self::AND(data) => Self::simplify_and_with(data, scope),
            Self::OR(data) => Self::simplify_or_with(data, scope),
            Self::XOR(data) => Self::simplify_xor_with(data, scope),
        }
    }
}

impl Operation {
    fn simplify_not(exp: Expression) -> Expression {
        let out = match exp.simplify() {
            Expression::Constant(Constant::ZERO) => return Expression::Constant(Constant::ONE),
            Expression::Constant(Constant::ONE) => return Expression::Constant(Constant::ZERO),
            Expression::Operation(Self::NOT(inner)) => return *inner,
            other => other,
        };

        Expression::Operation(Self::NOT(Box::new(out)))
    }
    fn simplify_not_with(exp: Expression, scope: &VarScope) -> Expression {
        let out = match exp.simplify_with(scope) {
            Expression::Constant(Constant::ZERO) => return Expression::Constant(Constant::ONE),
            Expression::Constant(Constant::ONE) => return Expression::Constant(Constant::ZERO),
            Expression::Operation(Self::NOT(inner)) => return *inner,
            other => other,
        };

        Expression::Operation(Self::NOT(Box::new(out)))
    }

    fn simplify_and(vecexp: Vec<Expression>) -> Expression {
        let mut new_exp: Vec<Expression> = vec![];

        for exp in vecexp.into_iter() {
            match exp.simplify() {
                Expression::Constant(Constant::ONE) => continue,
                Expression::Constant(Constant::ZERO) => {
                    return Expression::Constant(Constant::ZERO)
                }
                Expression::Operation(Self::AND(mut inner_and)) => new_exp.append(&mut inner_and),
                other => new_exp.push(other),
            }
        }

        match new_exp.len() {
            0 => Expression::Constant(Constant::ONE),
            1 => new_exp.into_iter().next().unwrap(),
            _ => Expression::Operation(Self::AND(new_exp)),
        }
    }
    fn simplify_and_with(vecexp: Vec<Expression>, scope: &VarScope) -> Expression {
        let mut new_exp: Vec<Expression> = vec![];

        for exp in vecexp.into_iter() {
            match exp.simplify_with(scope) {
                Expression::Constant(Constant::ONE) => continue,
                Expression::Constant(Constant::ZERO) => {
                    return Expression::Constant(Constant::ZERO)
                }
                Expression::Operation(Self::AND(mut inner_and)) => new_exp.append(&mut inner_and),
                other => new_exp.push(other),
            }
        }
        match new_exp.len() {
            0 => Expression::Constant(Constant::ONE),
            1 => new_exp.into_iter().next().unwrap(),
            _ => Expression::Operation(Self::AND(new_exp)),
        }
    }
    fn simplify_or(vecexp: Vec<Expression>) -> Expression {
        let mut new_exp: Vec<Expression> = vec![];

        for exp in vecexp.into_iter() {
            match exp.simplify() {
                Expression::Constant(Constant::ZERO) => continue,
                Expression::Constant(Constant::ONE) => return Expression::Constant(Constant::ONE),
                Expression::Operation(Self::OR(mut inner_or)) => new_exp.append(&mut inner_or),
                other => new_exp.push(other),
            }
        }

        match new_exp.len() {
            0 => Expression::Constant(Constant::ZERO),
            1 => new_exp.into_iter().next().unwrap(),
            _ => Expression::Operation(Self::OR(new_exp)),
        }
    }
    fn simplify_or_with(vecexp: Vec<Expression>, scope: &VarScope) -> Expression {
        let mut new_exp: Vec<Expression> = vec![];

        for exp in vecexp.into_iter() {
            match exp.simplify_with(scope) {
                Expression::Constant(Constant::ZERO) => continue,
                Expression::Constant(Constant::ONE) => return Expression::Constant(Constant::ONE),
                Expression::Operation(Self::OR(mut inner_or)) => new_exp.append(&mut inner_or),
                other => new_exp.push(other),
            }
        }
        match new_exp.len() {
            0 => Expression::Constant(Constant::ZERO),
            1 => new_exp.into_iter().next().unwrap(),
            _ => Expression::Operation(Self::OR(new_exp)),
        }
    }
    fn simplify_xor(vecexp: Vec<Expression>) -> Expression {
        let mut new_exp: Vec<Expression> = vec![];
        let mut spare_ones = 0;

        for exp in vecexp.into_iter() {
            match exp.simplify() {
                Expression::Constant(Constant::ONE) => spare_ones += 1,
                Expression::Constant(Constant::ZERO) => continue,
                Expression::Operation(Self::XOR(mut inner_xor)) => new_exp.append(&mut inner_xor),
                other => new_exp.push(other),
            }
        }

        let negate_result = spare_ones % 2 == 1;

        if new_exp.is_empty() {
            return Expression::Constant(Constant::ZERO.negate_if(negate_result));
        }

        let exp = match new_exp.len() {
            0 => unreachable!(),
            1 => new_exp.into_iter().next().unwrap(),
            _ => Expression::Operation(Operation::XOR(new_exp)),
        };

        if negate_result {
            Expression::Operation(Operation::NOT(Box::new(exp)))
        } else {
            exp
        }
    }

    fn simplify_xor_with(vecexp: Vec<Expression>, scope: &VarScope) -> Expression {
        let mut new_exp: Vec<Expression> = vec![];
        let mut spare_ones = 0;

        for exp in vecexp.into_iter() {
            match exp.simplify_with(scope) {
                Expression::Constant(Constant::ONE) => spare_ones += 1,
                Expression::Constant(Constant::ZERO) => continue,
                Expression::Operation(Self::XOR(mut inner_xor)) => new_exp.append(&mut inner_xor),
                other => new_exp.push(other),
            }
        }

        let negate_result = spare_ones % 2 == 1;

        if new_exp.is_empty() {
            return Expression::Constant(Constant::ZERO.negate_if(negate_result));
        }

        let exp = match new_exp.len() {
            0 => unreachable!(),
            1 => new_exp.into_iter().next().unwrap(),
            _ => Expression::Operation(Operation::XOR(new_exp)),
        };

        if negate_result {
            Expression::Operation(Operation::NOT(Box::new(exp)))
        } else {
            exp
        }
    }
}

impl Operation {
    fn evaluate_and(
        vecexp: &[Expression],
        scope: &crate::scope::VarScope,
    ) -> Result<Constant, String> {
        for exp in vecexp.iter() {
            match exp.evaluate(scope)? {
                Constant::ZERO => return Ok(Constant::ZERO),
                Constant::ONE => continue,
            }
        }
        Ok(Constant::ONE)
    }

    fn evaluate_or(
        vecexp: &[Expression],
        scope: &crate::scope::VarScope,
    ) -> Result<Constant, String> {
        for exp in vecexp.iter() {
            match exp.evaluate(scope)? {
                Constant::ONE => return Ok(Constant::ONE),
                Constant::ZERO => continue,
            }
        }
        Ok(Constant::ZERO)
    }

    fn evaluate_xor(
        vecexp: &[Expression],
        scope: &crate::scope::VarScope,
    ) -> Result<Constant, String> {
        let mut ones = 0;
        for exp in vecexp.iter() {
            if exp.evaluate(scope)? == Constant::ONE {
                ones += 1;
            }
        }

        if ones % 2 == 0 {
            Ok(Constant::ZERO)
        } else {
            Ok(Constant::ONE)
        }
    }
}
