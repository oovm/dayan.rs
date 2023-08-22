// use super::*;
//
// impl One for NAryHydra {
//     fn one() -> Self {
//         todo!()
//     }
// }
//
// impl Zero for NAryHydra {
//     fn zero() -> Self {
//         NAryHydra::Number(0)
//     }
//
//     fn is_zero(&self) -> bool {
//         matches!(self, NAryHydra::Number(0))
//     }
// }
// impl PartialOrd for NAryHydra {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
//
// impl Ord for NAryHydra {
//     fn cmp(&self, other: &Self) -> Ordering {
//         match (self, other) {
//             (NAryHydra::Number(a), NAryHydra::Number(b)) => a.cmp(b),
//             (NAryHydra::Number(_), NAryHydra::Beta(_, _)) => Ordering::Less,
//             (NAryHydra::Beta(_, _), NAryHydra::Number(_)) => Ordering::Greater,
//             (NAryHydra::Beta(a, _), NAryHydra::Beta(b, _)) => a.cmp(b),
//         }
//     }
// }
// impl Add for NAryHydra {
//     type Output = Self;
//
//     fn add(self, _: Self) -> Self::Output {
//         unimplemented!()
//     }
// }
//
// impl Mul for NAryHydra {
//     type Output = Self;
//
//     fn mul(self, _: Self) -> Self::Output {
//         unimplemented!()
//     }
// }
