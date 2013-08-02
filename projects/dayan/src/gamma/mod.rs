use std::fmt::{Debug, Display};

// mod display;
// mod parser;

/// A psi expression
#[derive(Clone)]
pub enum DayanGamma {
    /// A positive integer
    Number(u32),
    /// The first transfinite ordinal `ω`
    /// `D[0](0)`
    Psi { line1: Vec<DayanGamma>, line2: Vec<DayanGamma> },
}

impl Iterator for DayanGamma {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
