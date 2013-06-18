/// The error
#[derive(Debug, Clone)]
pub struct DayanError {
    kind: Box<DayanErrorKind>,
}

/// The
#[derive(Debug, Clone)]
pub enum DayanErrorKind {
    SyntaxError {
        message: String
    }
}