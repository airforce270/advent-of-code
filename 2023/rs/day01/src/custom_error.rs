use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct AocError {
    pub details: String,
}

impl AocError {
    pub fn new(msg: &str) -> AocError {
        AocError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for AocError {
    fn description(&self) -> &str {
        &self.details
    }
}
