use std::{fmt, io};
use std::error::Error;

/// Errores relacionados con la carga y manipulación de archivos AIR (Animation Information Report).
#[derive(Debug)]
pub enum AirError {
    /// Error que indica datos inválidos al cargar o manipular un archivo AIR.
    InvalidData(io::Error),
    /// Error que indica que no se pudo encontrar el archivo AIR especificado.
    NotFound(String),
    /// Error que indica un formato incorrecto al intentar formatear un archivo AIR.
    BadFormat(fmt::Error),
}

impl fmt::Display for AirError {
    /// Implementación de formato para mostrar los errores AIR.
    ///
    /// # Argumentos
    ///
    /// * `self` - La referencia al error AIR.
    /// * `f` - El formateador utilizado para escribir el mensaje de error.
    ///
    /// # Retorna
    ///
    /// Un resultado que indica si el formato fue exitoso o no.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AirError::InvalidData(err) => err.fmt(f),
            AirError::BadFormat(err) => err.fmt(f),
            AirError::NotFound(file) =>  write!(f, "Air file not found: {}", file),
        }
    }
}

impl Error for AirError {
    /// Implementación de método para obtener la fuente del error AIR.
    ///
    /// # Argumentos
    ///
    /// * `self` - La referencia al error AIR.
    ///
    /// # Retorna
    ///
    /// Una opción que contiene una referencia a la fuente del error si está presente.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AirError::InvalidData(ref err) => Some(err),
            AirError::BadFormat(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for AirError {
    /// Implementación de conversión desde `io::Error` a `AirError`.
    ///
    /// # Argumentos
    ///
    /// * `error` - El error de E/S que se está convirtiendo a `AirError`.
    ///
    /// # Retorna
    ///
    /// Un `AirError` que encapsula el error de E/S.
    fn from(error: io::Error) -> Self {
        AirError::InvalidData(error)
    }
}

impl From<fmt::Error> for AirError {
    /// Implementación de conversión desde `fmt::Error` a `AirError`.
    ///
    /// # Argumentos
    ///
    /// * `error` - El error de formato que se está convirtiendo a `AirError`.
    ///
    /// # Retorna
    ///
    /// Un `AirError` que encapsula el error de formato.
    fn from(error: fmt::Error) -> Self {
        AirError::BadFormat(error)
    }
}