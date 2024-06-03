use std::fmt;
use std::error::Error;


#[derive(Debug)]
pub enum CmdError {
    NotFound(String),
}

impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CmdError::NotFound(file) =>  write!(f, "Cmd file not found: {}", file),
        }
    }
}


impl Error for CmdError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            _ => None,
        }
    }
}