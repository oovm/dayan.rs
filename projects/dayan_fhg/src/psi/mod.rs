use std::fmt::{Debug, Display, Formatter, Write};

mod parser;
mod display;

#[derive(Clone)]
pub enum DayanPsi {
    Number(u32),
    Omega,
    Psi(Vec<DayanPsi>),
}



