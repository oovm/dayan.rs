use super::*;
use crate::utils::parse_integer;
use pex::{
    helpers::{make_from_str, whitespace},
    BracketPattern, ParseResult, ParseState, StopBecause,
};

impl FromStr for DayanAlpha {
    type Err = StopBecause;
    fn from_str(expression: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(expression.trim_end()).skip(whitespace);
        make_from_str(state, Self::parse)
    }
}

impl DayanAlpha {
    /// Parse a psi expression
    pub fn parse(state: ParseState) -> ParseResult<Self> {
        state.begin_choice().choose(parse_number).choose(parse_omega).choose(parse_psi).end_choice()
    }
}

fn parse_psi(input: ParseState) -> ParseResult<DayanAlpha> {
    let (state, _) = input.match_optional(parse_word)?;
    let pair = BracketPattern::new("(", ")").with_delimiter(",");
    let (state, terms) = pair.consume(state.skip(whitespace), whitespace, DayanAlpha::parse)?;
    state.finish(DayanAlpha::Psi(terms.body))
}

fn parse_word(input: ParseState) -> ParseResult<&str> {
    let (state, str) = input.match_str_if(|c| c.is_alphabetic(), "WORD")?;
    state.finish(str)
}

fn parse_number(input: ParseState) -> ParseResult<DayanAlpha> {
    parse_integer(input).map_inner(|s| DayanAlpha::Number(s))
}

fn parse_omega(input: ParseState) -> ParseResult<DayanAlpha> {
    match input.get_character(0) {
        Some('ω') => input.advance('ω'.len_utf8()).finish(DayanAlpha::Omega),
        Some('w') => input.advance('w'.len_utf8()).finish(DayanAlpha::Omega),
        _ => StopBecause::missing_character('ω', input.start_offset)?,
    }
}
