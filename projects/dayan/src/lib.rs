#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod alpha;
mod bms;
mod dps;
mod errors;
mod expression;
mod gamma;

pub(crate) mod utils;

pub use crate::{
    alpha::DayanAlpha,
    bms::{BMSConfig, BashicuMatrixSystem},
    errors::{DayanError, DayanErrorKind},
    expression::ExpressionTree,
};
pub use hydra_nary::{NAryHydra, NAryHydraTeX};
