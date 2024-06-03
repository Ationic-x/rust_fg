use std::path::PathBuf;
use std::{fmt, io};
use std::error::Error;

use crate::player::character::sff::decoder::Version;


#[derive(Debug)]
pub enum SffError {
    InvalidData(io::Error),
    NotFound(PathBuf),
    UnsupportedVersion(Version),
    UknownColorDepth(u8),
    UnsupportedHeaderVersion(u8),
    InvalidSignature,
}

impl fmt::Display for SffError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SffError::UknownColorDepth(cd) => write!(f, "Uknown Color Depth {}", cd),
            SffError::UnsupportedHeaderVersion(v) => write!(f, "Unsupported header version {}", v),
            SffError::UnsupportedVersion(v) => write!(f, "Unsupported version {:?}", v),
            SffError::InvalidData(err) => err.fmt(f),
            SffError::InvalidSignature => write!(f, "Invalid Signature"),
            SffError::NotFound(file) =>  write!(f, "Sff File not found: {}", file.to_string_lossy()),
        }
    }
}


impl Error for SffError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SffError::InvalidData(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for SffError {
    fn from(error: io::Error) -> Self {
        SffError::InvalidData(error)
    }
}