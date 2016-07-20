extern crate curl;

use std::error::{self, Error as StdError};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Api(String, String),
    Json,
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
