use std::sync::{mpsc::Sender, Arc, Mutex};

use gfx_device_gl::Device;
use piston::Key;
use piston_window::{Context, G2d};

use crate::preloader::preloader::Preloads;

use super::screen_manager::Event;

/// Trait que define el comportamiento de una pantalla en el juego.
pub trait Screen {
    /// Crea una nueva instancia de la pantalla.
    ///
    /// # Argumentos
    ///
    /// * `event_sender` - Un canal de comunicación para enviar eventos.
    /// * `preloads` - Un `Arc<Mutex<Preloads>>` que contiene los recursos pre-cargados compartidos.
    ///
    /// # Retorna
    ///
    /// Una nueva instancia de la pantalla.
    fn new(event_sender: Sender<Event>, preloads: Arc<Mutex<Preloads>>) -> Self
    where
        Self: Sized;

    /// Actualiza el estado de la pantalla.
    fn update(&mut self);

    /// Maneja el evento de presionar una tecla.
    ///
    /// # Argumentos
    ///
    /// * `key` - La tecla que se ha presionado.
    fn on_press(&mut self, key: Key);

    /// Maneja el evento de soltar una tecla.
    ///
    /// # Argumentos
    ///
    /// * `key` - La tecla que se ha soltado.
    fn on_release(&mut self, key: Key);

    /// Dibuja el contenido de la pantalla.
    ///
    /// # Argumentos
    ///
    /// * `c` - El contexto de renderizado.
    /// * `g` - El contexto de gráficos 2D.
    /// * `device` - El dispositivo de renderizado.
    fn draw(&mut self, c: Context, g: &mut G2d, device: &mut Device);
}
