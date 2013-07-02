use crate::{DayanError, ExpressionTree};
use std::collections::BTreeMap;

/// A beta expression
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DayanBeta {
    /// A positive integer
    Number(u32),
    /// A beta expression
    Beta(u32, Vec<DayanBeta>),
}

impl DayanBeta {
    pub fn as_expression(&self) -> Result<ExpressionTree, DayanError> {
        match self {
            DayanBeta::Number(v) => Ok(ExpressionTree::Number(*v)),
            DayanBeta::Beta(rank, p) => {
                let group = same_group(p);
                match *rank {
                    0 => panic!("DayanBeta::Beta(0, _) is not a valid expression"),
                    1 => self.cast_add(group),
                    2 => self.cast_mul(group),
                    _ => match p.as_slice() {
                        [head, rest @ ..] => head.cast_pow(rest),
                        _ => panic!("DayanBeta::Beta(3, _) is not a valid expression"),
                    },
                }
            }
        }
    }
    fn is_zero(&self) -> bool {
        match self {
            DayanBeta::Number(v) => *v == 0,
            _ => false,
        }
    }
    /// Check if expression is one
    pub fn is_number(&self) -> bool {
        match self {
            DayanBeta::Number(_) => true,
            _ => false,
        }
    }
    fn cast_add(&self, count: BTreeMap<&DayanBeta, u32>) -> Result<ExpressionTree, DayanError> {
        let mut terms = vec![];
        for (node, count) in count {
            terms.push(node.as_rank1()? * ExpressionTree::Number(count))
        }
       Ok(ExpressionTree::Sum { add: terms }.as_simplify())
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
    fn cast_mul(&self, count: BTreeMap<&DayanBeta, u32>) -> Result<ExpressionTree, DayanError> {
        let mut terms = vec![];
        for (node, count) in count {
            terms.push(node.as_rank2()? * ExpressionTree::Number(count))
        }
        Ok((ExpressionTree::Letter('w') ^ ExpressionTree::Sum { add: terms }).as_simplify())
    }
    fn as_rank2(&self) -> Result<ExpressionTree, DayanError> {
        match self {
            DayanBeta::Number(1) => Ok(ExpressionTree::Letter('w')),
            DayanBeta::Number(2) => Ok(ExpressionTree::Letter('w')),
            DayanBeta::Number(_) => {
                panic!()
            }
            DayanBeta::Beta(_, _) => self.as_expression(),
        }
    }
    fn cast_pow(&self, rest: &[Self]) -> Result<ExpressionTree, DayanError> {
        let mut base = self.as_expression()?;
        todo!()
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
