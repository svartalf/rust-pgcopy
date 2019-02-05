use std::io;
use std::fmt;
use std::result;
use std::error::Error as StdError;

use serde::ser::Error as SerdeError;

#[derive(Debug)]
pub enum ErrorKind {
    Io(io::Error),
    Serialize(String),

    #[doc(hidden)]
    __Nonexhaustive,
}

#[derive(Debug)]
pub struct Error(ErrorKind);

pub type Result<T> = result::Result<T, Error>;

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error(ErrorKind::Io(e))
    }
}

impl SerdeError for Error {
    fn custom<T>(msg: T) -> Self where T: fmt::Display {
        Error(ErrorKind::Serialize(msg.to_string()))
    }
}
