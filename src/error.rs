extern crate curl;

use std::error::{self, Error as StdError};
use std::fmt;

/// Error occuring during an API call.
#[derive(Debug)]
pub enum Error {
    /// Beam's API returned an error.
    Api(String, String),
    /// An error occurred while parsing JSON.
    Json,
    /// An HTTP error occurred.
    Http(curl::ErrCode),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Api(_, ref msg) => msg,
            Error::Json => "Invalid JSON",
            Error::Http(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Api(_, _) => None,
            Error::Json => None,
            Error::Http(ref err) => Some(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.description())
    }
}
