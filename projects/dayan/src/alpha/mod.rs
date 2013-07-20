use crate::{DayanError, ExpressionTree};
use std::{
    fmt::{Debug, Display, Formatter, Write},
    str::FromStr,
};

mod display;
mod parser;

/// A psi expression
#[derive(Clone)]
pub enum DayanAlpha {
    /// A positive integer
    Number(u32),
    /// The first transfinite ordinal `ω`
    Omega,
    /// `φ(a, b, c, ...)`
    Psi(Vec<DayanAlpha>),
}

impl DayanAlpha {
    /// Convert this psi representation to an expression tree
    pub fn as_expression(&self) -> Result<ExpressionTree, DayanError> {
        let out = match self {
            DayanAlpha::Number(v) => ExpressionTree::Number(*v),
            DayanAlpha::Omega => ExpressionTree::Letter('ω'),
            DayanAlpha::Psi(v) => match v.as_slice() {
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
            DayanAlpha::Number(v) => Ok(ExpressionTree::mul_add('ω', depth + 1, *v)),
            DayanAlpha::Omega => Ok(ExpressionTree::mul_add('ω', depth + 2, 0)),
            DayanAlpha::Psi(v) => match v.as_slice() {
                [] => Err(DayanError::too_less_argument("φ", 0).with_min_argument(1).with_max_argument(1)),
                [a] => a.unary(depth + 1),
                _ => Err(DayanError::too_much_argument("φ", v.len()).with_min_argument(1).with_max_argument(1)),
            },
        }
    }
    fn binary(&self, rhs: &Self) -> Result<ExpressionTree, DayanError> {
        match self {
            // FIXME: φ(1, φ(1, φ(0))): w^{2} + w^{2} + ω
            DayanAlpha::Number(v) => Ok(ExpressionTree::pow_add('ω', *v + 1, rhs.as_expression()?)),
            DayanAlpha::Omega => Ok(ExpressionTree::pow_add('ω', 'ω', rhs.as_expression()?)),
            DayanAlpha::Psi(v) => match v.as_slice() {
                [] => Err(DayanError::too_less_argument("φ", 0).with_min_argument(1).with_max_argument(2)),
                [a] => {
                    let base = ExpressionTree::Letter('ω') ^ a.unary(0)?;
                    Ok(base + rhs.as_expression()?)
                }
                [a, b] => {
                    let base = ExpressionTree::Letter('ω') ^ a.binary(b)?;
                    Ok(base + rhs.as_expression()?)
                }
                _ => Err(DayanError::too_much_argument("φ", v.len()).with_min_argument(1).with_max_argument(2)),
            },
        }
    }
    fn ternary(&self, mid: &Self, rhs: &Self) -> Result<ExpressionTree, DayanError> {
        match self {
            DayanAlpha::Number(1) => Ok(ExpressionTree::sub_add('ε', mid.as_expression()?, rhs.as_expression()?)),
            DayanAlpha::Number(2) => Ok(ExpressionTree::sub_add('ζ', mid.as_expression()?, rhs.as_expression()?)),
            DayanAlpha::Number(3) => Ok(ExpressionTree::sub_add('η', mid.as_expression()?, rhs.as_expression()?)),
            DayanAlpha::Number(v) => Ok(ExpressionTree::pow_add('ω', *v + 1, mid.as_expression()?) + rhs.as_expression()?),
            DayanAlpha::Omega => Ok(ExpressionTree::pow_add('ω', 'ω', mid.as_expression()?) + rhs.as_expression()?),
            DayanAlpha::Psi(v) => match v.as_slice() {
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
                _ => Err(DayanError::too_much_argument("φ", v.len()).with_min_argument(1).with_max_argument(3)),
            },
        }
    }
}
