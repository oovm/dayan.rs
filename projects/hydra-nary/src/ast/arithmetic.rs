use super::*;
use std::ops::ShlAssign;

impl Zero for NAryHydra {
    fn zero() -> Self {
        Self::Head { order: 0, range: Default::default() }
    }

    fn is_zero(&self) -> bool {
        match self {
            NAryHydra::Head { order, .. } => *order == 0,
            NAryHydra::Body { terms, .. } => terms.iter().all(|t| t.is_zero()),
        }
    }
}

impl Add for NAryHydra {
    type Output = Self;

    fn add(self, _: Self) -> Self::Output {
        unreachable!()
    }
}

impl ShlAssign for NAryHydra {
    fn shl_assign(&mut self, rhs: Self) {
        match self {
            NAryHydra::Head { .. } => {
                panic!("Cannot append to a head node");
            }
            NAryHydra::Body { terms, .. } => terms.push(rhs),
        }
    }
}
