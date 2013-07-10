use crate::BashicuMatrixSystem;
use pex::StopBecause;
use std::str::FromStr;

impl FromStr for BashicuMatrixSystem {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
