use std::path::PathBuf;
use std::{fmt, io};
use std::error::Error;

use native_dialog::{MessageDialog, MessageType};

use super::decoder::Version;


#[derive(Debug)]
pub enum DecodeError {
    InvalidData(io::Error),
    NotFound(PathBuf),
    UnsupportedVersion(Version),
    UknownColorDepth(u8),
    UnsupportedHeaderVersion(u8),
    InvalidSignature,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::UknownColorDepth(cd) => write!(f, "Uknown Color Depth {}", cd),
            DecodeError::UnsupportedHeaderVersion(v) => write!(f, "Unsupported header version {}", v),
            DecodeError::UnsupportedVersion(v) => write!(f, "Unsupported version {:?}", v),
            DecodeError::InvalidData(err) => err.fmt(f),
            DecodeError::InvalidSignature => write!(f, "Invalid Signature"),
            DecodeError::NotFound(file) =>  write!(f, "File not found: {}", file.to_string_lossy()),
        }
    }
}


impl Error for DecodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DecodeError::InvalidData(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for DecodeError {
    fn from(error: io::Error) -> Self {
        DecodeError::InvalidData(error)
    }
}

pub fn show_error_popup(error: &DecodeError) {
    MessageDialog::new()
        .set_type(MessageType::Error)
        .set_title("Error")
        .set_text(&format!("{}", error))
        .show_alert()
        .unwrap();
}