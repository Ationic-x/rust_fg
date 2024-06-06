use std::fmt;
use std::error::Error;


/// Errores relacionados con la carga de recursos durante la fase de precarga del juego.
#[derive(Debug)]
pub enum PreloadError {
    /// Error que indica que no se pudo encontrar el archivo de fuente especificado.
    FontNotFound(String),
    /// Error que indica que no se pudo encontrar el archivo de fondo especificado.
    BackgroundNotFound(String),
}

impl fmt::Display for PreloadError {
    /// Implementación de formato para mostrar los errores de precarga.
    ///
    /// # Argumentos
    ///
    /// * `self` - La referencia al error de precarga.
    /// * `f` - El formateador utilizado para escribir el mensaje de error.
    ///
    /// # Retorna
    ///
    /// Un resultado que indica si el formato fue exitoso o no.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PreloadError::FontNotFound(file) => write!(f, "Failed to load Font: {}", file),
            PreloadError::BackgroundNotFound(file) => write!(f, "Failed to load Background: {}", file),
        }
    }
}

impl Error for PreloadError {
    /// Implementación de método para obtener la fuente del error de precarga.
    ///
    /// # Argumentos
    ///
    /// * `self` - La referencia al error de precarga.
    ///
    /// # Retorna
    ///
    /// `None` porque no hay una fuente de error específica para los errores de precarga.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}