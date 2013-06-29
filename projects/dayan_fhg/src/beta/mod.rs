use crate::{DayanError, ExpressionTree};
use std::ptr::DynMetadata;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DayanBeta {
    Number(u32),
    Beta(u32, Vec<DayanBeta>),
}

impl DayanBeta {
    pub fn as_expression(&self) -> Result<ExpressionTree, DayanError> {
        match self {
            DayanBeta::Number(v) => Ok(ExpressionTree::Number(*v)),
            DayanBeta::Beta(rank, p) => match *rank {
                0 => panic!("DayanBeta::Beta(0, _) is not a valid expression"),
                1 => match p.as_slice() {
                    [] => Ok(ExpressionTree::Number(0)),
                    [head, rest @ ..] => head.cast_add(rest),
                },
                2 => match p.as_slice() {
                    [] => Ok(ExpressionTree::Number(1)),
                    [head, rest @ ..] => head.cast_mul(rest),
                },
                _ => match p.as_slice() {
                    [head, rest @ ..] => head.cast_pow(rest),
                    _ => panic!("DayanBeta::Beta(3, _) is not a valid expression"),
                },
            },
        }
    }
    fn cast_add(&self, rest: &[Self]) -> Result<ExpressionTree, DayanError> {
        let mut base = self.as_expression()?;
        for node in rest {
            base = base + node.as_expression()?;
        }
        Ok(base)
    }
    fn cast_mul(&self, rest: &[Self]) -> Result<ExpressionTree, DayanError> {
        let mut base = self.as_expression()?;
        for node in rest {
            base = base * node.as_expression()?;
        }
        Ok(base)
    }
    fn cast_pow(&self, rest: &[Self]) -> Result<ExpressionTree, DayanError> {
        let mut base = self.as_expression()?;
        todo!()
    }
}
fn same_group(list: &[DayanBeta]) -> Vec<&[DayanBeta]> {
    let mut groups = vec![];
}
