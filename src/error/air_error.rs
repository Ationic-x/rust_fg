use std::{fmt, io};
use std::error::Error;


#[derive(Debug)]
pub enum AirError {
    InvalidData(io::Error),
    NotFound(String),
    BadFormat(fmt::Error),
}

impl fmt::Display for AirError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AirError::InvalidData(err) => err.fmt(f),
            AirError::BadFormat(err) => err.fmt(f),
            AirError::NotFound(file) =>  write!(f, "Air file not found: {}", file),
        }
    }
}


impl Error for AirError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AirError::InvalidData(ref err) => Some(err),
            AirError::BadFormat(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for AirError {
    fn from(error: io::Error) -> Self {
        AirError::InvalidData(error)
    }
}

impl From<fmt::Error> for AirError {
    fn from(error: fmt::Error) -> Self {
        AirError::BadFormat(error)
    }
}