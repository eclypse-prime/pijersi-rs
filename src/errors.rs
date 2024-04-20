use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct IllegalActionError {
    message: String,
}

impl IllegalActionError {
    pub fn new(msg: &str) -> IllegalActionError {
        IllegalActionError {
            message: msg.to_owned(),
        }
    }
}

impl fmt::Display for IllegalActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for IllegalActionError {
    fn description(&self) -> &str {
        &self.message
    }
}
