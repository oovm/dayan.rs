use std::fmt::{Debug, Display, Formatter, Write};
use crate::{DayanError, ExpressionTree};

mod parser;
mod display;

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
            DayanPsi::Number(v) => {
                ExpressionTree::Number(*v)
            }
            DayanPsi::Omega => {
                ExpressionTree::Letter('ω')
            }
            DayanPsi::Psi(v) => {
                match v.as_slice() {
                    [] => {
                        Err(DayanError::too_less_argument("psi", 0).with_min_argument(1))?
                    }
                    [a] => {
                        a.linear(0)?
                    }
                    [a, b] => {
                        ExpressionTree::Sub {
                            head: Box::new(a.as_expression()?),
                            rest: Box::new(b.as_expression()?),
                        }
                    }
                    _ => {
                        panic!("too many psi")
                    }
                }
            }
        };
        Ok(out)
    }
    fn linear(&self, depth: u32) -> Result<ExpressionTree, DayanError> {
        match self {
            DayanPsi::Number(v) => {
                Ok(ExpressionTree::linear('ω', depth + 1, *v))
            }
            DayanPsi::Omega => {
                Ok(ExpressionTree::linear('ω', depth + 2, 0))
            }
            DayanPsi::Psi(v) => {
                match v.as_slice() {
                    [] => {
                        Err(DayanError::too_less_argument("psi", 0).with_min_argument(1).with_max_argument(1))
                    }
                    [a] => {
                        a.linear(depth + 1)
                    }
                    _ => {
                        Err(DayanError::too_much_argument("psi", v.len()).with_min_argument(1).with_max_argument(1))
                    }
                }
            }
        }
    }
}
