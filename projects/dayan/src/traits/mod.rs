use crate::{BashicuMatrixSystem, BashicuMatrixSystemTex};
use hydra_nary::NAryHydra;

/// Trait for converting between different ordinal notations.
pub trait OrdinalNotation {
    fn as_bms(&self) -> BashicuMatrixSystem;

    fn as_nary_hydra(&self) -> NAryHydra;
}

impl OrdinalNotation for BashicuMatrixSystem {
    fn as_bms(&self) -> BashicuMatrixSystem {
        self.clone()
    }

    fn as_nary_hydra(&self) -> NAryHydra {
        let first = self.term();
        let mut body = NAryHydra::Body { ranks: vec![1; first - 1], terms: vec![], range: Default::default() };
        for line in self.get_terms_unsaturated() {
            match line.as_slice() {
                [depth, rest @ .., number] => {
                    let depth = *depth;
                    let mut ranks = vec![depth + 1];
                    ranks.extend(rest.iter().map(|i| i + 1));
                    // let new_depth = *new_depth.first().unwrap_or(&0);
                    let new_body = NAryHydra::Body {
                        ranks,
                        terms: vec![NAryHydra::Head { order: *number, range: Default::default() }],
                        range: Default::default(),
                    };
                    body.mut_child(depth as usize).push(new_body)
                }
                _ => {
                    unreachable!()
                }
            }
        }
        body
    }
}

impl BashicuMatrixSystem {
    fn as_nary_hydra_term(&self, row: &[u32]) -> NAryHydra {
        if row.is_empty() {
            return NAryHydra::Head { order: 0, range: Default::default() };
        }

        let order = row[0] as u32 + 1;

        NAryHydra::Head { order, range: Default::default() }
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
