use crate::NAryHydra;
use pex::{
    helpers::{make_from_str, whitespace},
    BracketPair, BracketPattern, Parse, ParseResult, ParseState, StopBecause,
};
use std::{ops::Range, str::FromStr};

impl FromStr for NAryHydra {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, Self::parse)
    }
}

impl Parse for NAryHydra {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().choose_from(parse_body).choose_from(parse_head).end_choice()
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            NAryHydra::Head { range, .. } => range.clone(),
            NAryHydra::Body { range, .. } => range.clone(),
        }
    }
}
pub fn parse_body(input: ParseState) -> ParseResult<NAryHydra> {
    let state = input.skip(parse_function).skip(whitespace);
    let (state, mut ranks) = state.begin_choice().choose(unary_rank).choose(bracket_rank).end_choice()?;
    let (state, _) = state.skip(whitespace).match_char('(')?;
    // 1,2,3;
    let (state, inner) = state.match_optional(inner_rank).map_inner(|s| s.unwrap_or_default())?;
    ranks.extend(inner);
    let (state, terms) = inner_term(state)?;
    let (state, _) = state.skip(whitespace).match_char(')')?;
    state.finish(NAryHydra::Body { ranks, terms, range: state.away_from(input) })
}

#[test]
fn test() {
    let out = inner_rank(ParseState::new("1, 2;"));
    println!("{:#?}", out);
}

// rank,?; terms
fn inner_rank(input: ParseState) -> ParseResult<Vec<usize>> {
    let (state, ranks) = input.match_optional(|s| {
        let pairs = BracketPattern::new("", "");
        let (state, terms) = pairs.consume(input, whitespace, parse_integer)?;
        state.finish(terms.body)
    })?;
    let (state, _) = state.skip(whitespace).match_char(';')?;
    state.finish(ranks.unwrap_or_default())
}

fn inner_term(input: ParseState) -> ParseResult<Vec<NAryHydra>> {
    let mut items = Vec::new();
    let (state, first) = NAryHydra::parse(input)?;
    items.push(first);
    let (state, rank) = state.match_repeats(|s| {
        let (state, _) = s.skip(whitespace).match_char(',')?;
        let (state, rank) = NAryHydra::parse(state.skip(whitespace))?;
        state.finish(rank)
    })?;
    items.extend(rank);
    let state = state.skip(whitespace).skip(|c| c.match_char(',')).skip(whitespace);
    state.finish(items)
}

// [rank]?(terms)
fn bracket_rank(input: ParseState) -> ParseResult<Vec<usize>> {
    let (state, rank) = input.match_optional(|s| {
        let pair = BracketPattern::new("[", "]");
        let (state, ranks) = pair.consume(s, whitespace, parse_integer)?;
        state.finish(ranks.body)
    })?;
    state.finish(rank.unwrap_or_default())
}

// rank(terms)
fn unary_rank(input: ParseState) -> ParseResult<Vec<usize>> {
    let (state, rank) = parse_integer(input)?;
    state.finish(vec![rank])
}

pub fn parse_function(input: ParseState) -> ParseResult<usize> {
    let (state, str) = input.match_str_if(|c| c.is_alphabetic(), "DECIMAL")?;
    state.finish(usize::from_str(str)?)
}

pub fn parse_head(input: ParseState) -> ParseResult<NAryHydra> {
    let (state, i) = parse_integer(input)?;
    state.finish(NAryHydra::Head { order: i, range: state.away_from(input) })
}

pub fn parse_integer(input: ParseState) -> ParseResult<usize> {
    let (state, str) = input.match_str_if(|c| c.is_ascii_digit(), "DECIMAL")?;
    state.finish(usize::from_str(str)?)
}
