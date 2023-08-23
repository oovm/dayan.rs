use crate::NAryHydra;
use latexify::Latexify;
use std::fmt::{Debug, Display, Formatter, Write};
mod display;
mod tex;

/// render the node as a string
#[derive(Clone, Debug)]
pub struct NAryHydraTeX {
    /// function name of the node
    pub function: Option<char>,
    /// placeholder for the function name
    pub placeholder: String,
    /// put ranks front or back
    pub prefix: bool,
}

impl Default for NAryHydraTeX {
    fn default() -> Self {
        Self { function: Some('p'), placeholder: "∘".to_string(), prefix: true }
    }
}

impl NAryHydraTeX {
    /// render the node as a string
    pub fn render(&self, node: &NAryHydra) -> String {
        let mut buffer = String::new();
        Latexify::fmt(node, self, &mut buffer).unwrap();
        buffer
    }
}
