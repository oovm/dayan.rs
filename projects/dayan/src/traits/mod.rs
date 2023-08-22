use crate::{BashicuMatrixSystem, BashicuMatrixSystemTex};
use hydra_nary::NAryHydra;

pub trait OrdinalNotation {
    fn as_bms(&self) -> BashicuMatrixSystem;

    fn as_nary_hydra(&self) -> NAryHydra;
}

impl OrdinalNotation for BashicuMatrixSystem {
    fn as_bms(&self) -> BashicuMatrixSystem {
        self.clone()
    }

    fn as_nary_hydra(&self) -> NAryHydra {
        let mut depth = 0;
        let first = self.terms().nth(0).map(|v| v.len()).unwrap_or(1);
        let mut body = NAryHydra::Body { ranks: vec![0; first], terms: vec![], range: Default::default() };
        for line in self.terms().skip(1) {
            match line.as_slice() {
                [rank @ .., rest] => {
                    let new_depth = *rank.first().unwrap_or(&0);
                    let new_body = NAryHydra::Body {
                        ranks: rank.to_vec(),
                        terms: vec![NAryHydra::Head { order: *rest, range: Default::default() }],
                        range: Default::default(),
                    };
                    body <<= new_body;
                }
                _ => {
                    unreachable!()
                }
            }
        }
        body
    }
}

impl OrdinalNotation for NAryHydra {
    fn as_bms(&self) -> BashicuMatrixSystem {
        match self {
            Self::Head { order, .. } => {
                unreachable!("{}", order)
            }
            Self::Body { ranks, terms, .. } => {
                let mut depth = 0;
                let mut matrix = vec![];
                for (index, rank) in ranks.iter().enumerate() {
                    if index != 0 {
                        matrix.push(vec![0; depth]);
                    }
                    matrix.push(vec![*rank; terms.len()]);
                    depth += 1;
                }
                todo!()
                // BashicuMatrixSystem::new(matrix).unwrap()
            }
        }
    }

    fn as_nary_hydra(&self) -> NAryHydra {
        self.clone()
    }
}
