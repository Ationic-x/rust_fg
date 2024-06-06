use std::fmt;
use std::error::Error;


/// Errores relacionados con la gestión de comandos en el juego.
#[derive(Debug)]
pub enum CmdError {
    /// Error que indica que no se pudo encontrar el archivo de comando especificado.
    NotFound(String),
}

impl fmt::Display for CmdError {
    /// Implementación de formato para mostrar los errores de comandos.
    ///
    /// # Argumentos
    ///
    /// * `self` - La referencia al error de comando.
    /// * `f` - El formateador utilizado para escribir el mensaje de error.
    ///
    /// # Retorna
    ///
    /// Un resultado que indica si el formato fue exitoso o no.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CmdError::NotFound(file) =>  write!(f, "Cmd file not found: {}", file),
        }
    }
}

impl Error for CmdError {
    /// Implementación de método para obtener la fuente del error de comando.
    ///
    /// # Argumentos
    ///
    /// * `self` - La referencia al error de comando.
    ///
    /// # Retorna
    ///
    /// `None` porque no hay una fuente de error específica para los errores de comando.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}