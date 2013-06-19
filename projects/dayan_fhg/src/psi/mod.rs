use std::fmt::{Debug, Display, Formatter, Write};
use crate::{DayanError, ExpressionTree};

mod parser;
mod display;

#[derive(Clone)]
pub enum DayanPsi {
    Number(u32),
    Omega,
    Psi(Vec<DayanPsi>),
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
    fn check1(&self, depth: u32) -> Result<ExpressionTree, DayanError> {
        match self {
            DayanPsi::Number(v) => {
                println!("Number depth: {}", depth);
                if depth == 0 {
                    if *v == 0 {
                        Ok(ExpressionTree::Letter('ω'))
                    }
                    else {
                        Ok(ExpressionTree::Add {
                            lhs: Box::new(ExpressionTree::Letter('ω')),
                            rhs: Box::new(ExpressionTree::Number(*v)),
                        })
                    }
                }
                else {
                    if *v == 0 {
                        Ok(ExpressionTree::Mul {
                            lhs: Box::new(ExpressionTree::Letter('ω')),
                            rhs: Box::new(ExpressionTree::Number(depth + 1)),
                        })
                    }
                    else {
                        Ok(ExpressionTree::Mul {
                            lhs: Box::new(ExpressionTree::Letter('ω')),
                            rhs: Box::new(ExpressionTree::Add {
                                lhs: Box::new(ExpressionTree::Number(depth + 1)),
                                rhs: Box::new(ExpressionTree::Number(*v)),
                            }),
                        })
                    }
                }
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
