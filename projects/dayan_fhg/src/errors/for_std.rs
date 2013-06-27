use crate::DayanError;
use std::io::Error;

impl From<std::io::Error> for DayanError {
    fn from(value: Error) -> Self {
        todo!()
    }
}
