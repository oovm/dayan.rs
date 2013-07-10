use pex::{ParseResult, ParseState};
use std::str::FromStr;

pub fn parse_integer(input: ParseState) -> ParseResult<u32> {
    let (state, str) = input.match_str_if(|c| c.is_ascii_digit(), "DECIMAL")?;
    state.finish(u32::from_str(str)?)
}
