use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct TransactionServiceCreationError(pub String);

#[derive(Debug)]
pub struct TransactionFailedError(pub String);

impl fmt::Display for TransactionServiceCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl fmt::Display for TransactionFailedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl Error for TransactionServiceCreationError {}
impl Error for TransactionFailedError {}
