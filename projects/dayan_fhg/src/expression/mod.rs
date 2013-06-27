mod arithmetic;
mod display;

use std::{
    fmt::{Display, Formatter, Write},
    ops::{Add, BitXor, Mul},
};

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
        base: Box<ExpressionTree>,
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
    /// `x × k + a`
    #[inline]
    pub fn mul_add<B, P, A>(base: B, times: P, a: A) -> Self
    where
        B: Into<ExpressionTree>,
        P: Into<ExpressionTree>,
        A: Into<ExpressionTree>,
    {
        base.into() * times.into() + a.into()
    }
    /// `x ^ k + a`
    #[inline]
    pub fn pow_add<B, P, A>(base: B, power: P, a: A) -> Self
    where
        B: Into<ExpressionTree>,
        P: Into<ExpressionTree>,
        A: Into<ExpressionTree>,
    {
        (base.into() ^ power.into()) + a.into()
    }
    #[inline]
    pub fn sub_add<B, P, A>(base: B, power: P, a: A) -> Self
    where
        B: Into<ExpressionTree>,
        P: Into<ExpressionTree>,
        A: Into<ExpressionTree>,
    {
        ExpressionTree::Sub { head: Box::new(base.into()), rest: Box::new(power.into()) } + a.into()
    }
    /// Check if expression is zero
    pub fn is_zero(&self) -> bool {
        match self {
            ExpressionTree::Number(v) => *v == 0,
            _ => false,
        }
    }
    /// Check if expression is one
    pub fn is_one(&self) -> bool {
        match self {
            ExpressionTree::Number(v) => *v == 1,
            _ => false,
        }
    }
    fn is_digit(&self) -> bool {
        match self {
            ExpressionTree::Number(v) if *v < 10 => true,
            ExpressionTree::Letter(_) => true,
            _ => false,
        }
    }
}
