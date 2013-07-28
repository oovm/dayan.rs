use std::fmt::{Debug, Formatter};
use std::ops::AddAssign;
use ndarray::Array2;

#[derive(Clone, Copy)]
pub struct DayanPair {
    depth: u32,
    count: u32,

}

impl Debug for DayanPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.depth, self.count)
    }
}

#[derive(Debug, Default)]
pub struct DayanPairSequence{
    sequence: Vec<DayanPair>
}
// impl Debug for DayanPairSequence {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         Debug::fmt(&self.sequence, f)
//     }
// }

impl DayanPair {
    pub fn new(depth: u32, count: u32) -> DayanPair {
        DayanPair {
            count,
            depth,
        }
    }
}

impl AddAssign<DayanPair> for DayanPairSequence {
    fn add_assign(&mut self, rhs: DayanPair) {
        self.sequence.push(rhs);
    }
}