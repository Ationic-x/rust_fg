use std::path::PathBuf;
use std::{fmt, io};
use std::error::Error;

use crate::player::character::sff::decoder::Version;

/// Errores relacionados con la carga y manipulación de archivos SFF (Sprite File Format).
#[derive(Debug)]
pub enum SffError {
    /// Datos inválidos.
    InvalidData(io::Error),
    /// Archivo no encontrado.
    NotFound(PathBuf),
    /// Versión no soportada.
    UnsupportedVersion(Version),
    /// Profundidad de color desconocida.
    UnknownColorDepth(u8),
    /// Versión de encabezado no soportada.
    UnsupportedHeaderVersion(u8),
    /// Firma inválida.
    InvalidSignature,
}

impl fmt::Display for SffError {
    /// Formatea el error para visualización.
    ///
    /// # Argumentos
    ///
    /// * `f` - Formateador de texto.
    ///
    /// # Retorna
    ///
    /// Retorna un resultado de formato.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SffError::UnknownColorDepth(cd) => write!(f, "Profundidad de color desconocida: {}", cd),
            SffError::UnsupportedHeaderVersion(v) => write!(f, "Versión de encabezado no soportada: {}", v),
            SffError::UnsupportedVersion(v) => write!(f, "Versión no soportada: {:?}", v),
            SffError::InvalidData(err) => err.fmt(f),
            SffError::InvalidSignature => write!(f, "Firma inválida"),
            SffError::NotFound(file) =>  write!(f, "Archivo SFF no encontrado: {}", file.to_string_lossy()),
        }
    }
}

impl Error for SffError {
    /// Devuelve la fuente del error.
    ///
    /// # Retorna
    ///
    /// Retorna una opción que contiene una referencia al error subyacente.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SffError::InvalidData(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for SffError {
    /// Convierte un error de E/S en un error de SFF.
    ///
    /// # Argumentos
    ///
    /// * `error` - Error de E/S a convertir.
    ///
    /// # Retorna
    ///
    /// Retorna el error de SFF resultante.
    fn from(error: io::Error) -> Self {
        SffError::InvalidData(error)
    }
}
