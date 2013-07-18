use crate::{utils::parse_integer, BashicuMatrixSystem, DayanError};
use pex::{
    helpers::{make_from_str, whitespace},
    BracketPattern, ParseResult, ParseState,
};
use std::str::FromStr;

impl FromStr for BashicuMatrixSystem {
    type Err = DayanError;

    fn from_str(s: &str) -> Result<Self, DayanError> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        let matrix = make_from_str(state, parse_many)?;
        Self::new(matrix)
    }
}

fn parse_many(input: ParseState) -> ParseResult<Vec<Vec<u32>>> {
    let (state, many) = input.match_repeats(parse_column)?;
    state.finish(many)
}

fn parse_column(input: ParseState) -> ParseResult<Vec<u32>> {
    let pair = BracketPattern::new("(", ")").with_delimiter(",");
    let (state, terms) = pair.consume(input.skip(whitespace), whitespace, parse_integer)?;
    state.finish(terms.body)
}
