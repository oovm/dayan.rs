use super::*;


impl One for DayanBeta {
    fn one() -> Self {
        todo!()
    }
}





impl Zero for DayanBeta {
    fn zero() -> Self {
        DayanBeta::Number(0)
    }

    fn is_zero(&self) -> bool {
        matches!(self, DayanBeta::Number(0))
    }
}
impl PartialOrd for DayanBeta {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DayanBeta {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (DayanBeta::Number(a), DayanBeta::Number(b)) => a.cmp(b),
            (DayanBeta::Number(_), DayanBeta::Beta(_, _)) => Ordering::Less,
            (DayanBeta::Beta(_, _), DayanBeta::Number(_)) => Ordering::Greater,
            (DayanBeta::Beta(a, _), DayanBeta::Beta(b, _)) => a.cmp(b),
        }
    }
}
impl Add for DayanBeta {
    type Output = Self;

    fn add(self, _: Self) -> Self::Output {
        unimplemented!()
    }
}

impl Mul for DayanBeta {
    type Output = Self;

    fn mul(self, _: Self) -> Self::Output {
        unimplemented!()
    }
}