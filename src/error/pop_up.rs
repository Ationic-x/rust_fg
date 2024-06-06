use std::fmt::Display;
use native_dialog::{MessageDialog, MessageType};

// Trait que define un error que puede ser mostrado
pub trait ShowableError: Display {}

impl<T: Display> ShowableError for T {}

/// Muestra un diálogo emergente de error con el mensaje de error proporcionado.
///
/// # Argumentos
///
/// * `error` - Una referencia al error que se va a mostrar en el diálogo.
pub fn show_error_popup<E: ShowableError>(error: &E) {
    MessageDialog::new()
        .set_type(MessageType::Error)
        .set_title("Error")
        .set_text(&format!("{}", error))
        .show_alert()
        .unwrap();
}