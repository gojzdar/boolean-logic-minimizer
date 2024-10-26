use crate::expression::Expression;

#[allow(clippy::upper_case_acronyms)]
pub enum Operation {
    NOT(Box<Expression>),
    AND(Vec<Expression>),
    OR(Vec<Expression>),
    XOR(Vec<Expression>),
}
