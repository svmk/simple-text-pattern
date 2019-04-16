use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
/// Structure that contains error message.
pub struct Error(String);

impl Error {
    pub (crate) fn new(error: impl Into<String>) -> Error {
        return Error(error.into());
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        return &self.0;
    }
}