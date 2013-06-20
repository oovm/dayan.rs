mod display;
mod arithmetic;

use std::fmt::{Display, Formatter, Write};
use std::mem::replace;
use std::ops::{AddAssign, BitXorAssign, MulAssign};

/// A tree representing an expression
#[derive(Debug)]
pub enum ExpressionTree {
    /// A positive integer
    Number(u32),
    /// A character letter
    Letter(char),
    /// `lhs + rhs`
    Add {
        /// The left hand side of the addition
        lhs: Box<ExpressionTree>,
        /// The right hand side of the addition
        rhs: Box<ExpressionTree>,
    },
    /// `lhs × rhs`
    Mul {
        /// The left hand side of the multiplication
        lhs: Box<ExpressionTree>,
        /// The right hand side of the multiplication
        rhs: Box<ExpressionTree>,
    },
    /// `head ^ rest`
    Sup {
        /// The head of the superscript
        head: Box<ExpressionTree>,
        /// The rest of the superscript
        rest: Box<ExpressionTree>,
    },
    /// `head _ rest`
    Sub {
        /// The head of the subscript
        head: Box<ExpressionTree>,
        /// The rest of the subscript
        rest: Box<ExpressionTree>,
    },
}

impl ExpressionTree {
    /// `x × k + b`
    pub fn linear(x: char, k: u32, b: u32) -> Self {
        let mut out = ExpressionTree::Letter(x);
        out *= ExpressionTree::Number(k);
        out += ExpressionTree::Number(b);
        out
    }
    /// `x ^ p + rhs`
    pub fn pow_add(mut base: ExpressionTree, p: u32, rhs: ExpressionTree) -> Self {
        if p != 1 {
            base ^= ExpressionTree::Number(p);
        }
        base += rhs;
        base
    }
    /// Check if expression is zero
    pub fn is_zero(&self) -> bool {
        match self {
            ExpressionTree::Number(v) => {
                *v == 0
            }
            _ => {
                false
            }
        }
    }
    /// Check if expression is one
    pub fn is_one(&self) -> bool {
        match self {
            ExpressionTree::Number(v) => {
                *v == 1
            }
            _ => {
                false
            }
        }
    }
}

