use std::fmt::{Display, Formatter};

pub enum DayanPsi {
    Number(u32),
    Omega,
    Psi(Vec<DayanPsi>),
}


impl Display for DayanPsi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}