use crate::{DayanError, ExpressionTree};
use std::fmt::{Debug, Display, Formatter, Write};

mod display;
mod parser;

/// A psi expression
#[derive(Clone)]
pub enum DayanPsi {
    /// A positive integer
    Number(u32),
    /// The first transfinite ordinal `ω`
    Omega,
    /// `ϕ(a, b, c, ...)`
    Psi(Vec<DayanPsi>),
}

impl DayanPsi {
    /// Convert this psi representation to an expression tree
    pub fn as_expression(&self) -> Result<ExpressionTree, DayanError> {
        let out = match self {
            DayanPsi::Number(v) => ExpressionTree::Number(*v),
            DayanPsi::Omega => ExpressionTree::Letter('ω'),
            DayanPsi::Psi(v) => match v.as_slice() {
                [] => Err(DayanError::too_less_argument("psi", 0).with_min_argument(1))?,
                [a] => a.unary(0)?,
                [a, b] => a.binary(b)?,
                [a, b, c] => a.ternary(b, c)?,
                _ => {
                    panic!("too many psi")
                }
            },
        };
        Ok(out)
    }
    fn unary(&self, depth: u32) -> Result<ExpressionTree, DayanError> {
        match self {
            DayanPsi::Number(v) => Ok(ExpressionTree::mul_add('ω', depth + 1, *v)),
            DayanPsi::Omega => Ok(ExpressionTree::mul_add('ω', depth + 2, 0)),
            DayanPsi::Psi(v) => match v.as_slice() {
                [] => Err(DayanError::too_less_argument("ϕ", 0).with_min_argument(1).with_max_argument(1)),
                [a] => a.unary(depth + 1),
                _ => Err(DayanError::too_much_argument("ϕ", v.len()).with_min_argument(1).with_max_argument(1)),
            },
        }
    }
    fn binary(&self, rhs: &Self) -> Result<ExpressionTree, DayanError> {
        match self {
            // FIXME: ϕ(1, ϕ(1, ϕ(0))): w^{2} + w^{2} + ω
            DayanPsi::Number(v) => Ok(ExpressionTree::pow_add('ω', *v + 1, rhs.as_expression()?)),
            DayanPsi::Omega => Ok(ExpressionTree::pow_add('ω', 'ω', rhs.as_expression()?)),
            DayanPsi::Psi(v) => match v.as_slice() {
                [] => Err(DayanError::too_less_argument("ϕ", 0).with_min_argument(1).with_max_argument(2)),
                [a] => {
                    let base = ExpressionTree::Letter('ω') ^ a.unary(0)?;
                    Ok(base + rhs.as_expression()?)
                }
                [a, b] => {
                    let base = ExpressionTree::Letter('ω') ^ a.binary(b)?;
                    Ok(base + rhs.as_expression()?)
                }
                _ => Err(DayanError::too_much_argument("ϕ", v.len()).with_min_argument(1).with_max_argument(2)),
            },
        }
    }
    fn ternary(&self, mid: &Self, rhs: &Self) -> Result<ExpressionTree, DayanError> {
        match self {
            DayanPsi::Number(1) => Ok(ExpressionTree::sub_add('ε', mid.as_expression()?, rhs.as_expression()?)),
            DayanPsi::Number(2) => Ok(ExpressionTree::sub_add('ζ', mid.as_expression()?, rhs.as_expression()?)),
            DayanPsi::Number(3) => Ok(ExpressionTree::sub_add('η', mid.as_expression()?, rhs.as_expression()?)),
            DayanPsi::Number(v) => Ok(ExpressionTree::pow_add('ω', *v + 1, mid.as_expression()?) + rhs.as_expression()?),
            DayanPsi::Omega => Ok(ExpressionTree::pow_add('ω', 'ω', mid.as_expression()?) + rhs.as_expression()?),
            DayanPsi::Psi(v) => match v.as_slice() {
                [] => Err(DayanError::too_less_argument("psi", 0).with_min_argument(1).with_max_argument(3)),
                [a] => {
                    let base = ExpressionTree::Letter('ω') ^ a.unary(0)?;
                    Ok(base + mid.as_expression()? + rhs.as_expression()?)
                }
                [a, b] => {
                    let base = ExpressionTree::Letter('ω') ^ a.binary(b)?;
                    Ok(base + mid.as_expression()? + rhs.as_expression()?)
                }
                [a, b, c] => {
                    let base = ExpressionTree::Letter('ω') ^ a.ternary(b, c)?;
                    Ok(base + mid.as_expression()? + rhs.as_expression()?)
                }
                _ => Err(DayanError::too_much_argument("ϕ", v.len()).with_min_argument(1).with_max_argument(3)),
            },
        }
    }
}
