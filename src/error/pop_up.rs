use std::fmt::Display;
use native_dialog::{MessageDialog, MessageType};

pub trait ShowableError: Display {}

impl<T: Display> ShowableError for T {}

pub fn show_error_popup<E: ShowableError>(error: &E) {
    MessageDialog::new()
        .set_type(MessageType::Error)
        .set_title("Error")
        .set_text(&format!("{}", error))
        .show_alert()
        .unwrap();
}