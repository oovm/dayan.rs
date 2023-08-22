use super::*;

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
