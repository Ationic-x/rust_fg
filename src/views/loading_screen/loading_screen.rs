use std::sync::{mpsc::Sender, Arc, Mutex};

use graphics::clear;

use crate::{
    preloader::preloader::Preloads,
    views::{screen::Screen, screen_manager::Event},
};

use super::gui;

/// Representa la pantalla de carga del juego.
pub struct LoadingScreen {
    preloads: Arc<Mutex<Preloads>>,
    event_sender: Sender<Event>,
    ticks: usize,
}

impl Screen for LoadingScreen {
    /// Crea una nueva instancia de `LoadingScreen`.
    ///
    /// # Argumentos
    ///
    /// * `event_sender` - El canal de eventos para comunicarse con el administrador de pantallas.
    /// * `preloads` - Los recursos precargados para el juego.
    ///
    /// # Retorna
    ///
    /// Una nueva instancia de `LoadingScreen`.
    fn new(event_sender: Sender<Event>, preloads: Arc<Mutex<Preloads>>) -> Self
    where
        Self: Sized,
    {
        Self {
            preloads,
            event_sender,
            ticks: 0,
        }
    }

    /// Actualiza el estado de la pantalla de carga.
    fn update(&mut self) {
        if self.ticks == 0 {
            self.event_sender.send(Event::ScreenReady()).unwrap();
        }
        self.ticks += 1;
    }

    /// Maneja el evento de presionar una tecla en la pantalla de carga.
    ///
    /// # Argumentos
    ///
    /// * `key` - La tecla que se ha presionado.
    fn on_press(&mut self, key: piston_window::prelude::Key) {
        let _ = key;
    }

    /// Maneja el evento de soltar una tecla en la pantalla de carga.
    ///
    /// # Argumentos
    ///
    /// * `key` - La tecla que se ha soltado.
    fn on_release(&mut self, key: piston_window::prelude::Key) {
        let _ = key;
    }

    /// Dibuja el contenido de la pantalla de carga.
    ///
    /// # Argumentos
    ///
    /// * `c` - El contexto de dibujo.
    /// * `g` - El contexto de gráficos.
    /// * `device` - El dispositivo de dibujo.
    fn draw(
        &mut self,
        c: graphics::Context,
        g: &mut piston_window::prelude::G2d,
        device: &mut gfx_device_gl::Device,
    ) {
        clear([0.0; 4], g);
        gui::draw_loading(
            c,
            g,
            device,
            self.preloads
                .lock()
                .unwrap()
                .get_mut_ref_fonts()
                .get_mut(1)
                .unwrap(),
        );
    }
}
