use crate::{DayanError, ExpressionTree};
use num_traits::{One, Zero};
use std::{
    cmp::Ordering,
    collections::BTreeMap,
    ops::{Add, Mul},
    str::FromStr,
    vec,
};
use crate::dps::{DayanPair, DayanPairSequence};

mod arithmetic;
mod parser;

/// A beta expression
#[derive(Debug, Eq, PartialEq)]
pub enum DayanBeta {
    /// A positive integer
    Number(u32),
    /// A beta expression
    Beta(u32, Vec<DayanBeta>),
}

impl DayanBeta {
    /// Convert to an expression tree
    pub fn as_expression(&self) -> Result<ExpressionTree, DayanError> {
        match self {
            DayanBeta::Number(v) => Ok(ExpressionTree::Number(*v)),
            DayanBeta::Beta(rank, p) => {
                let group = same_group(p);
                match *rank {
                    // identity
                    0 => match p.len() {
                        0 => Ok(ExpressionTree::Number(0)),
                        1 => p[0].as_expression(),
                        _ => Err(DayanError::too_much_argument("DayanBeta::Beta(0, _)", p.len())
                            .with_min_argument(0)
                            .with_max_argument(1)),
                    },
                    // sum
                    1 => self.cast_add(group).map(|s| s.as_simplify()),
                    // product
                    2 => self.cast_mul(group).map(|s| s.as_simplify()),
                    3 => self.cast_pow(group).map(|s| s.as_simplify()),
                    _ => panic!("DayanBeta::Beta(3, _) is not a valid expression"),
                }
            }
        }
    }
    /// Check if expression is number
    pub fn is_number(&self) -> bool {
        matches!(self, DayanBeta::Number(_))
    }
}

impl DayanBeta {
    fn cast_add(&self, count: BTreeMap<&DayanBeta, u32>) -> Result<ExpressionTree, DayanError> {
        let mut terms = vec![];
        for (node, count) in count.into_iter().rev() {
            terms.push(node.as_rank1()? * ExpressionTree::Number(count))
        }
        Ok(ExpressionTree::Sum { add: terms })
    }
    fn as_rank1(&self) -> Result<ExpressionTree, DayanError> {
        match self {
            DayanBeta::Number(1) => Ok(ExpressionTree::Number(1)),
            DayanBeta::Number(_) => {
                panic!()
            }
            DayanBeta::Beta(_, _) => self.as_expression(),
        }
    }
}
impl DayanBeta {
    fn cast_mul(&self, count: BTreeMap<&DayanBeta, u32>) -> Result<ExpressionTree, DayanError> {
        let mut terms = vec![];
        for (node, count) in count.into_iter().rev() {
            terms.push(node.as_rank2(count)?)
        }
        Ok(ExpressionTree::Product { mul: terms })
    }
    fn as_rank2(&self, count: u32) -> Result<ExpressionTree, DayanError> {
        match self {
            DayanBeta::Number(1) => Ok(ExpressionTree::Letter('ω') ^ ExpressionTree::Number(count)),
            DayanBeta::Number(2) => Ok(ExpressionTree::subscript('ε', count)),
            DayanBeta::Number(_) => {
                panic!()
            }
            DayanBeta::Beta(_, _) => self.as_expression(),
        }
    }
}

impl DayanBeta {
    fn cast_pow(&self, count: BTreeMap<&DayanBeta, u32>) -> Result<ExpressionTree, DayanError> {
        let mut terms = vec![];
        for (node, count) in count.into_iter().rev() {
            terms.push(node.as_rank3(count, 1)?)
        }
        Ok(ExpressionTree::subscript('φ', ExpressionTree::Product { mul: terms }))
    }
    fn as_rank3(&self, count: u32, depth: u32) -> Result<ExpressionTree, DayanError> {
        match self {
            DayanBeta::Number(1) => {
                todo!()
            }
            DayanBeta::Number(2) => {
                todo!()
            }
            DayanBeta::Number(_) => {
                panic!()
            }
            DayanBeta::Beta(_, _) => self.as_expression(),
        }
    }
}

fn same_group(list: &[DayanBeta]) -> BTreeMap<&DayanBeta, u32> {
    let mut groups: BTreeMap<&DayanBeta, u32> = Default::default();
    let mut current = None;
    for item in list {
        if item.is_zero() {
            continue;
        }
        match current {
            None => {
                current = Some(item);
                groups.insert(item, 1);
            }
            Some(current_item) => {
                if current_item == item {
                    *groups.get_mut(current_item).unwrap() += 1;
                }
                else {
                    current = Some(item);
                    groups.insert(item, 1);
                }
            }
        }
    }
    groups
}


impl DayanBeta {
    pub fn as_dps(&self) -> Result<DayanPairSequence, DayanError> {
        let mut out = DayanPairSequence::default();
        match self {
            DayanBeta::Number(v) => {
                for _ in 0..*v {
                    out += DayanPair::new(0,0);
                }
            },
            DayanBeta::Beta(rank, p) => {
                for i in 1..*rank {
                    out += DayanPair::new(i-1,i-1);
                }
                for i in p {
                    i.visit_dps(&mut out, *rank, *rank - 1)?;
                }
            }
        }

        Ok(out)
    }
    fn visit_dps(&self, dps: &mut DayanPairSequence, max: u32, depth: u32) -> Result<(), DayanError> {
        match self {
            DayanBeta::Number(v) => {
                *dps += DayanPair::new(depth, *v - 1);
            },
            DayanBeta::Beta(rank, p) => {
                for i in p {
                    if *rank > max {
                        *dps += DayanPair::new(depth, *rank - 1);
                        i.visit_dps(dps, *rank,depth+1)?;
                    }
                    else {
                        for _ in 0..max - *rank {
                            *dps += DayanPair::new(depth, 0);
                        }
                        *dps += DayanPair::new(depth+1, *rank - 1);
                        i.visit_dps(dps, max,depth+1)?;
                    }
                }
            }
        }
        Ok(())
    }
}
