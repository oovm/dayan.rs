use std::str::FromStr;
use std::string::ParseError;
use crate::DayanPsi;

impl FromStr for DayanPsi {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}