mod display;

use std::fmt::{Display, Formatter, Write};

#[derive(Debug)]
pub enum ExpressionTree {
    Number(u32),
    Letter(char),
    Add {
        lhs: Box<ExpressionTree>,
        rhs: Box<ExpressionTree>,
    },
    Mul {
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

impl ExpressionTree {
    pub fn linear(x: char, k: u32, b: u32) {

    }
}