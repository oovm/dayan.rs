use num_traits::Zero;
use std::{
    fmt::{Display, Formatter, Write},
    ops::{Add, Range},
};
mod arithmetic;
// mod parser;

mod display;

/// n-ary ast
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NAryHydra {
    /// The head of ast, usually a natural number
    Head {
        /// The order of the ast
        order: usize,
        /// The raw text span
        range: Range<usize>,
    },
    /// The body of ast
    Body {
        /// The ranks of the ast
        ranks: Vec<usize>,
        /// The terms of the ast
        terms: Vec<NAryHydra>,
        /// The raw text span
        range: Range<usize>,
    },
}

impl NAryHydra {
    /// Remove invalid nodes and simplify the representation
    pub fn simplify(&self) -> Self {
        match self {
            Self::Head { order, range } => Self::Head { order: *order, range: range.clone() },
            Self::Body { ranks, terms, range } => {
                let ranks = ranks.iter().filter(|i| **i != 0).cloned().collect();
                let terms = terms.iter().map(|i| i.simplify()).filter(|i| !i.is_zero()).collect();
                Self::Body { ranks, terms, range: range.clone() }
            }
        }
    }
}

// impl NAryHydra {
//     /// Convert to an expression tree
//     pub fn as_expression(&self) -> Result<ExpressionTree, DayanError> {
//         match self {
//             NAryHydra::Number(v) => Ok(ExpressionTree::Number(*v)),
//             NAryHydra::Beta(rank, p) => {
//                 let group = same_group(p);
//                 match *rank {
//                     // identity
//                     0 => match p.len() {
//                         0 => Ok(ExpressionTree::Number(0)),
//                         1 => p[0].as_expression(),
//                         _ => Err(DayanError::too_much_argument("DayanBeta::Beta(0, _)", p.len())
//                             .with_min_argument(0)
//                             .with_max_argument(1)),
//                     },
//                     // sum
//                     1 => self.cast_add(group).map(|s| s.as_simplify()),
//                     // product
//                     2 => self.cast_mul(group).map(|s| s.as_simplify()),
//                     3 => self.cast_pow(group).map(|s| s.as_simplify()),
//                     _ => panic!("DayanBeta::Beta(3, _) is not a valid expression"),
//                 }
//             }
//         }
//     }
//     /// Check if expression is number
//     pub fn is_number(&self) -> bool {
//         matches!(self, NAryHydra::Number(_))
//     }
// }
//
// impl NAryHydra {
//     fn cast_add(&self, count: BTreeMap<&NAryHydra, u32>) -> Result<ExpressionTree, DayanError> {
//         let mut terms = vec![];
//         for (node, count) in count.into_iter().rev() {
//             terms.push(node.as_rank1()? * ExpressionTree::Number(count))
//         }
//         Ok(ExpressionTree::Sum { add: terms })
//     }
//     fn as_rank1(&self) -> Result<ExpressionTree, DayanError> {
//         match self {
//             NAryHydra::Number(1) => Ok(ExpressionTree::Number(1)),
//             NAryHydra::Number(_) => {
//                 panic!()
//             }
//             NAryHydra::Beta(_, _) => self.as_expression(),
//         }
//     }
// }
// impl NAryHydra {
//     fn cast_mul(&self, count: BTreeMap<&NAryHydra, u32>) -> Result<ExpressionTree, DayanError> {
//         let mut terms = vec![];
//         for (node, count) in count.into_iter().rev() {
//             terms.push(node.as_rank2(count)?)
//         }
//         Ok(ExpressionTree::Product { mul: terms })
//     }
//     fn as_rank2(&self, count: u32) -> Result<ExpressionTree, DayanError> {
//         match self {
//             NAryHydra::Number(1) => Ok(ExpressionTree::Letter('ω') ^ ExpressionTree::Number(count)),
//             NAryHydra::Number(2) => Ok(ExpressionTree::subscript('ε', count)),
//             NAryHydra::Number(_) => {
//                 panic!()
//             }
//             NAryHydra::Beta(_, _) => self.as_expression(),
//         }
//     }
// }
//
// impl NAryHydra {
//     fn cast_pow(&self, count: BTreeMap<&NAryHydra, u32>) -> Result<ExpressionTree, DayanError> {
//         let mut terms = vec![];
//         for (node, count) in count.into_iter().rev() {
//             terms.push(node.as_rank3(count, 1)?)
//         }
//         Ok(ExpressionTree::subscript('φ', ExpressionTree::Product { mul: terms }))
//     }
//     fn as_rank3(&self, count: u32, depth: u32) -> Result<ExpressionTree, DayanError> {
//         match self {
//             NAryHydra::Number(1) => {
//                 todo!()
//             }
//             NAryHydra::Number(2) => {
//                 todo!()
//             }
//             NAryHydra::Number(_) => {
//                 panic!()
//             }
//             NAryHydra::Beta(_, _) => self.as_expression(),
//         }
//     }
// }
//
// fn same_group(list: &[NAryHydra]) -> BTreeMap<&NAryHydra, u32> {
//     let mut groups: BTreeMap<&NAryHydra, u32> = Default::default();
//     let mut current = None;
//     for item in list {
//         if item.is_zero() {
//             continue;
//         }
//         match current {
//             None => {
//                 current = Some(item);
//                 groups.insert(item, 1);
//             }
//             Some(current_item) => {
//                 if current_item == item {
//                     *groups.get_mut(current_item).unwrap() += 1;
//                 }
//                 else {
//                     current = Some(item);
//                     groups.insert(item, 1);
//                 }
//             }
//         }
//     }
//     groups
// }
//
// impl NAryHydra {
//     pub fn as_dps(&self) -> Result<DayanPairSequence, DayanError> {
//         let mut out = DayanPairSequence::default();
//         match self {
//             NAryHydra::Number(v) => {
//                 for _ in 0..*v {
//                     out += DayanPair::new(0, 0);
//                 }
//             }
//             NAryHydra::Beta(rank, p) => {
//                 for i in 1..*rank {
//                     out += DayanPair::new(i - 1, i - 1);
//                 }
//                 for i in p {
//                     i.visit_dps(&mut out, *rank, *rank - 1)?;
//                 }
//             }
//         }
//
//         Ok(out)
//     }
//     fn visit_dps(&self, dps: &mut DayanPairSequence, max: u32, depth: u32) -> Result<(), DayanError> {
//         match self {
//             NAryHydra::Number(v) => {
//                 *dps += DayanPair::new(depth, *v - 1);
//             }
//             NAryHydra::Beta(rank, p) => {
//                 for i in p {
//                     if *rank > max {
//                         *dps += DayanPair::new(depth, *rank - 1);
//                         i.visit_dps(dps, *rank, depth + 1)?;
//                     }
//                     else {
//                         for _ in 0..max - *rank {
//                             *dps += DayanPair::new(depth, 0);
//                         }
//                         *dps += DayanPair::new(depth + 1, *rank - 1);
//                         i.visit_dps(dps, max, depth + 1)?;
//                     }
//                 }
//             }
//         }
//         Ok(())
//     }
// }
