use std::fmt;
use std::error::Error;


#[derive(Debug)]
pub enum PreloadError {
    FontNotFound(String),
    BackgroundNotFound(String),
}

impl fmt::Display for PreloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PreloadError::FontNotFound(file) =>  write!(f, "Failed to load Font : {}", file),
            PreloadError::BackgroundNotFound(file) =>  write!(f, "Failed to load Background : {}", file),
        }
    }
}

impl Error for PreloadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            _ => None,
        }
    }
}