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
                [a] => a.linear(0)?,
                [a, b] => a.pow(b, 0)?,
                _ => {
                    panic!("too many psi")
                }
            },
        };
        Ok(out)
    }
    fn linear(&self, depth: u32) -> Result<ExpressionTree, DayanError> {
        match self {
            DayanPsi::Number(v) => Ok(ExpressionTree::linear('ω', depth + 1, *v)),
            DayanPsi::Omega => Ok(ExpressionTree::linear('ω', depth + 2, 0)),
            DayanPsi::Psi(v) => match v.as_slice() {
                [] => Err(DayanError::too_less_argument("ϕ", 0).with_min_argument(1).with_max_argument(1)),
                [a] => a.linear(depth + 1),
                _ => Err(DayanError::too_much_argument("ϕ", v.len()).with_min_argument(1).with_max_argument(1)),
            },
        }
    }
    fn pow(&self, rhs: &Self, depth: u32) -> Result<ExpressionTree, DayanError> {
        match self {
            // w ^ (v + 1)
            DayanPsi::Number(v) => {
                // FIXME: ϕ(1, ϕ(1, ϕ(0))): w^{2} + w^{2} + ω
                let mut base = rhs.as_expression()? + *v;
                for i in 0..=depth {
                    base = ExpressionTree::Letter('ω') ^ base;
                }
                Ok(base)
            }
            DayanPsi::Omega => {
                let mut base = ExpressionTree::Letter('ω');
                for i in 0..=depth {
                    base ^= ExpressionTree::Letter('ω');
                }
                Ok(base + rhs.as_expression()?)
            }
            DayanPsi::Psi(v) => match v.as_slice() {
                [] => Err(DayanError::too_less_argument("psi", 0).with_min_argument(1).with_max_argument(1)),
                [a] => a.pow(&Self::Number(0), depth + 1),
                [a, b] => a.pow(b, depth + 1),
                _ => Err(DayanError::too_much_argument("psi", v.len()).with_min_argument(1).with_max_argument(1)),
            },
        }
    }
}
