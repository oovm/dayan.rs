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
    /// `D[0](0)`
    Psi { line1: Vec<DayanAlpha>, line2: Vec<DayanAlpha> },
}

impl Iterator for DayanAlpha {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            DayanAlpha::Number(_) => {}
            DayanAlpha::Psi { .. } => {}
        }
    }
}
