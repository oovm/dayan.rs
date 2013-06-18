use std::fmt::{Debug, Display, Formatter, Write};
use crate::DayanError;

mod parser;
mod display;

#[derive(Clone)]
pub enum DayanPsi {
    Number(u32),
    Omega,
    Psi(Vec<DayanPsi>),
}

#[derive(Debug)]
pub enum ExpressionTree {
    Number(u32),
    Letter(char),
    Add {
        lhs: Box<ExpressionTree>,
        rhs: Box<ExpressionTree>,
    },
    Sup {
        head: Box<ExpressionTree>,
        rest: Box<ExpressionTree>,
    },
    Sub {
        head: Box<ExpressionTree>,
        rest: Box<ExpressionTree>,
    },
}

impl DayanPsi {
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
                        panic!("empty psi")
                    }
                    [a] => {
                        a.check1(0)?
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
    fn check1(&self, depth: usize) -> Result<ExpressionTree, DayanError> {
        match self {
            DayanPsi::Number(v) => {
                println!("Number depth: {}", depth);
                Ok(ExpressionTree::Number(*v))
            }
            DayanPsi::Omega => {
                println!("Omega depth: {}", depth);
                Ok(ExpressionTree::Letter('ω'))
            }
            DayanPsi::Psi(v) => {
                match v.as_slice() {
                    [] => {
                        panic!("empty psi")
                    }
                    [a] => {
                        a.check1(depth + 1)
                    }
                    _ => {
                        panic!("too many psi")
                    }
                }
            }
        }
    }
}
