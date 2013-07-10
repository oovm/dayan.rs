#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod beta;
mod errors;
mod expression;
mod psi;

mod bms;

pub(crate) mod utils;

pub use crate::{
    beta::DayanBeta,
    errors::{DayanError, DayanErrorKind},
    expression::ExpressionTree,
    psi::DayanPsi,
};

pub use crate::bms::{BMSConfig, BashicuMatrixSystem};
