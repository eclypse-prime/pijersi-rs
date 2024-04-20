use core::fmt;

/// An error that can be returned when attempting to play an illegal action.
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

/// An error that can be returned when attempting to convert from an illegal FEN string.
#[derive(Debug)]
pub struct StringParseError {
    message: String,
}

impl StringParseError {
    pub fn new(msg: &str) -> StringParseError {
        StringParseError {
            message: msg.to_owned(),
        }
    }
}

impl fmt::Display for StringParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
