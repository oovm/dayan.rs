use pex::StopBecause;
use std::ops::Range;

/// The error
#[derive(Debug, Clone)]
pub struct DayanError {
    kind: Box<DayanErrorKind>,
}

impl From<StopBecause> for DayanError {
    fn from(value: StopBecause) -> Self {
        Self { kind: Box::new(DayanErrorKind::SyntaxError { message: value.to_string(), range: value.range() }) }
    }
}

/// The
#[derive(Debug, Clone)]
pub enum DayanErrorKind {
    /// The syntax error
    SyntaxError {
        /// The error message
        message: String,
        /// The range of the error
        range: Range<usize>,
    },
    /// The error when the argument count is too less
    TooLessArgument {
        /// Function which missing argument
        function: String,
        /// The current argument count
        count: u32,
        /// The minimum argument excepted
        except_min: Option<u32>,
        /// The maximum argument excepted
        except_max: Option<u32>,
    },
    /// The error when the argument count is too much
    TooMuchArgument {
        /// Function which has too much argument
        function: String,
        /// The current argument count
        count: u32,
        /// The minimum argument excepted
        except_min: Option<u32>,
        /// The maximum argument excepted
        except_max: Option<u32>,
    },
}

impl DayanError {
    /// Report function that the argument is too less
    pub fn too_less_argument<S: ToString>(message: S, count: usize) -> Self {
        DayanError {
            kind: Box::new(DayanErrorKind::TooLessArgument {
                function: message.to_string(),
                count: count as u32,
                except_min: None,
                except_max: None,
            }),
        }
    }
    /// Report function that the argument is too much
    pub fn too_much_argument<S: ToString>(message: S, count: usize) -> Self {
        DayanError {
            kind: Box::new(DayanErrorKind::TooMuchArgument {
                function: message.to_string(),
                count: count as u32,
                except_min: None,
                except_max: None,
            }),
        }
    }
    /// Fill the minimum argument excepted
    pub fn with_min_argument(mut self, except: u32) -> Self {
        match self.kind.as_mut() {
            DayanErrorKind::TooLessArgument { except_min, .. } => {
                *except_min = Some(except);
            }
            _ => {}
        }
        self
    }
    /// Fill the maximum argument excepted
    pub fn with_max_argument(mut self, except: u32) -> Self {
        match self.kind.as_mut() {
            DayanErrorKind::TooLessArgument { except_max, .. } => {
                *except_max = Some(except);
            }
            _ => {}
        }
        self
    }
}
