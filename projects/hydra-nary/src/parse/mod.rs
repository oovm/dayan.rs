use crate::NAryHydra;
use pex::{
    helpers::{make_from_str, whitespace},
    Parse, ParseResult, ParseState, StopBecause,
};
use std::{ops::Range, str::FromStr};
use NAryHydra::Body;

impl FromStr for NAryHydra {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, Self::parse)
    }
}

impl Parse for NAryHydra {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().choose_from(parse_unary).choose_from(parse_multiple).choose_from(parse_head).end_choice()
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            NAryHydra::Head { range, .. } => range.clone(),
            Body { range, .. } => range.clone(),
        }
    }
}

/// `p[1](2; 3)`
fn parse_multiple(input: ParseState) -> ParseResult<NAryHydra> {
    let state = input.skip(function_name).skip(whitespace);
    let (state, mut ranks) = state
        .match_optional(|s| {
            let (state, _) = s.match_char('[')?;
            let (state, body) = state.skip(whitespace).match_repeats(parse_number)?;
            let (state, _) = state.match_char(']')?;
            state.finish(body)
        })
        .map_inner(|v| v.unwrap_or_default())?;
    let (state, _) = state.skip(whitespace).match_char('(')?;
    let (state, inner) = state.match_optional(inner_rank).map_inner(|s| s.unwrap_or_default())?;
    ranks.extend(inner);
    let (state, terms) = state.match_repeats(parse_term)?;
    let (state, _) = state.match_char(')')?;
    state.finish(Body { ranks, terms, range: state.away_from(input) })
}

// p1(2; 3)
fn parse_unary(input: ParseState) -> ParseResult<NAryHydra> {
    let state = input.skip(function_name).skip(whitespace);
    let (state, mut ranks) = parse_integer(state).map_inner(|v| vec![v])?;
    let (state, _) = state.skip(whitespace).match_char('(')?;
    let (state, inner) = state.match_optional(inner_rank).map_inner(|s| s.unwrap_or_default())?;
    ranks.extend(inner);
    let (state, terms) = state.match_repeats(parse_term)?;
    let (state, _) = state.match_char(')')?;
    state.finish(Body { ranks, terms, range: state.away_from(input) })
}

pub fn parse_head(input: ParseState) -> ParseResult<NAryHydra> {
    let (state, i) = parse_integer(input)?;
    state.finish(NAryHydra::Head { order: i, range: state.away_from(input) })
}

/// ```vk
/// ;
/// 1;
/// 1,;
/// 1,2;
/// (n,)*;
/// ```
fn inner_rank(input: ParseState) -> ParseResult<Vec<usize>> {
    let (state, terms) = input.skip(whitespace).match_repeats(parse_number)?;
    let (state, _) = state.match_char(';')?;
    state.skip(whitespace).finish(terms)
}

fn parse_term(input: ParseState) -> ParseResult<NAryHydra> {
    let (state, hydra) = NAryHydra::parse(input)?;
    let state = state.skip(whitespace).skip(|c| c.match_char(',')).skip(whitespace);
    state.finish(hydra)
}

fn function_name(input: ParseState) -> ParseResult<()> {
    let (state, _) = input.match_str_if(|c| c.is_alphabetic(), "FUNCTION")?;
    state.finish(())
}

fn parse_number(input: ParseState) -> ParseResult<usize> {
    let (state, int) = parse_integer(input)?;
    let state = state.skip(whitespace).skip(|c| c.match_char(',')).skip(whitespace);
    state.finish(int)
}

fn parse_integer(input: ParseState) -> ParseResult<usize> {
    let (state, str) = input.match_str_if(|c| c.is_ascii_digit(), "DECIMAL")?;
    state.finish(usize::from_str(str)?)
}
