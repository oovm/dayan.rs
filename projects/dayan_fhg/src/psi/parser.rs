use super::*;
use pex::{
    helpers::{make_from_str, whitespace},
    BracketPattern, ParseResult, ParseState, StopBecause,
};
use std::str::FromStr;

impl FromStr for DayanPsi {
    type Err = StopBecause;
    fn from_str(expression: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(expression.trim_end()).skip(whitespace);
        make_from_str(state, Self::parse)
    }
}

impl DayanPsi {
    /// Parse a psi expression
    pub fn parse(state: ParseState) -> ParseResult<Self> {
        state.begin_choice().choose(parse_number).choose(parse_omega).choose(parse_psi).end_choice()
    }
}

fn parse_psi(input: ParseState) -> ParseResult<DayanPsi> {
    let (state, _) = input.match_optional(parse_word)?;
    let pair = BracketPattern::new("(", ")").with_delimiter(",");
    let (state, terms) = pair.consume(state.skip(whitespace), whitespace, DayanPsi::parse)?;
    state.finish(DayanPsi::Psi(terms.body))
}

fn parse_word(input: ParseState) -> ParseResult<&str> {
    let (state, str) = input.match_str_if(|c| c.is_alphabetic(), "WORD")?;
    state.finish(str)
}

fn parse_number(input: ParseState) -> ParseResult<DayanPsi> {
    let (state, str) = input.match_str_if(|c| c.is_ascii_digit(), "DECIMAL")?;
    state.finish(DayanPsi::Number(u32::from_str(str)?))
}

fn parse_omega(input: ParseState) -> ParseResult<DayanPsi> {
    match input.get_character(0) {
        Some('ω') => input.advance('ω'.len_utf8()).finish(DayanPsi::Omega),
        Some('w') => input.advance('w'.len_utf8()).finish(DayanPsi::Omega),
        _ => StopBecause::missing_character('ω', input.start_offset)?,
    }
}
