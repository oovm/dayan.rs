mod arithmetic;
mod display;

use std::{
    fmt::{Display, Formatter, Write},
    ops::{Add, BitXor, Mul},
};

/// A tree representing an expression
#[derive(Clone, Debug)]
pub enum ExpressionTree {
    /// A positive integer
    Number(u32),
    /// A character letter
    Letter(char),
    /// `lhs + rhs`
    Sum {
        /// The left hand side of the addition
        add: Vec<ExpressionTree>,
    },
    /// `lhs × rhs`
    Product {
        /// The left hand side of the multiplication
        mul: Vec<ExpressionTree>,
    },
    /// `head ^ rest`
    Sup {
        /// The head of the superscript
        base: Box<ExpressionTree>,
        /// The rest of the superscript
        rest: Box<ExpressionTree>,
    },
    /// `head _ rest`
    Subscript {
        /// The head of the subscript
        base: Box<ExpressionTree>,
        rest: Box<ExpressionTree>,
    },
    Function {
        body: FunctionExpression,
    }
}

/// A function expression
#[derive(Clone, Debug)]
pub struct FunctionExpression {
    name: char,
    sup: Vec<ExpressionTree>,
    sub: Vec<ExpressionTree>,
    args: Vec<ExpressionTree>,
}

impl FunctionExpression {
    /// Create a new function expression
    pub fn new(name: char) -> Self {
        FunctionExpression {
            name,
            sup: Vec::new(),
            sub: Vec::new(),
            args: Vec::new(),
        }
    }
}

impl ExpressionTree {
    pub fn subscript<B, R>(base: B, rest: R) -> Self
    where
        B: Into<ExpressionTree>,
        R: Into<ExpressionTree>,
    {
        ExpressionTree::Subscript {
            base: Box::new(base.into()),
            rest: Box::new(rest.into()),
        }
    }

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
    /// `x _ k + a`
    #[inline]
    pub fn sub_add<B, P, A>(base: B, power: P, a: A) -> Self
    where
        B: Into<ExpressionTree>,
        P: Into<ExpressionTree>,
        A: Into<ExpressionTree>,
    {
        Self::subscript(base, power) + a.into()
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
    /// Convert to a string
    pub fn as_simplify(&self) -> ExpressionTree {
        match self {
            ExpressionTree::Number(v) => ExpressionTree::Number(*v),
            ExpressionTree::Letter(v) => ExpressionTree::Letter(*v),
            ExpressionTree::Sum { add } => {
                let mut new = Vec::with_capacity(add.len());
                for old in add {
                    let term = old.as_simplify();
                    if term.is_zero() {
                        continue;
                    }
                    new.push(term);
                }
                if new.is_empty() {
                    ExpressionTree::Number(0)
                } else if new.len() == 1 {
                    new.remove(0)
                } else {
                    ExpressionTree::Sum { add: new }
                }
            }
            ExpressionTree::Product { mul } => {
                let mut new = Vec::with_capacity(mul.len());
                for old in mul {
                    let term = old.as_simplify();
                    if term.is_zero() {
                        return ExpressionTree::Number(0);
                    }
                    if term.is_one() {
                        continue;
                    }
                    new.push(term);
                }
                if new.is_empty() {
                    ExpressionTree::Number(1)
                } else if new.len() == 1 {
                    new.remove(0)
                } else {
                    ExpressionTree::Product { mul: new }
                }

            }
            ExpressionTree::Sup { base, rest } => {
                ExpressionTree::Sup { base: Box::new(base.as_simplify()), rest: Box::new(rest.as_simplify()) }
            }
            ExpressionTree::Subscript { base, rest } => {
                ExpressionTree::Subscript { base: Box::new(base.as_simplify()), rest: Box::new(rest.as_simplify()) }
            }
            ExpressionTree::Function { body } => {
                ExpressionTree::Function {
                    body: FunctionExpression {
                        name: body.name,
                        sup: body.sup.iter().map(|t| t.as_simplify()).collect(),
                        sub: body.sub.iter().map(|t| t.as_simplify()).collect(),
                        args: body.args.iter().map(|t| t.as_simplify()).collect(),
                    }
                }
            }
        }
    }
}
