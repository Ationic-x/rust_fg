use std::sync::{mpsc::Sender, Arc, Mutex};

use graphics::{clear, image};
use piston::Key;

use crate::{
    preloader::preloader::Preloads,
    views::{
        screen::Screen,
        screen_manager::{Event, ScreenType},
    },
};

use super::gui;

const TICK_RESET: usize = 20;

/// Representa la pantalla de selección de personajes.
pub struct RosterScreen {
    p1_selected_index: usize,
    p1_selected: bool,
    p1_color: usize,
    p1_index_color: usize,
    p2_selected_index: usize,
    p2_selected: bool,
    p2_index_color: usize,
    p2_color: usize,
    ticks: usize,
    event_sender: Sender<Event>,
    preloads: Arc<Mutex<Preloads>>,
}

impl Screen for RosterScreen {
    /// Crea una nueva instancia de `RosterScreen`.
    ///
    /// # Argumentos
    ///
    /// * `event_sender` - El canal de eventos para comunicarse con el administrador de pantallas.
    /// * `preloads` - Los recursos precargados para el juego.
    ///
    /// # Retorna
    ///
    /// Una nueva instancia de `RosterScreen`.
    fn new(event_sender: Sender<Event>, preloads: Arc<Mutex<Preloads>>) -> Self
    where
        Self: Sized,
    {
        Self {
            p1_selected_index: 0,
            p2_selected_index: 0,
            preloads,
            ticks: 0,
            p1_selected: false,
            p1_index_color: 1,
            p1_color: 0,
            p2_selected: false,
            p2_index_color: 1,
            p2_color: 0,
            event_sender,
        }
    }

    /// Actualiza el estado de la pantalla de selección de personajes.
    fn update(&mut self) {
        self.ticks += 1;
        if self.p1_color > 0 && self.p2_color > 0 && self.ticks > TICK_RESET {
            let preloads = self.preloads.lock().unwrap();
            let characters = [
                preloads.get_ref_roster()[self.p1_selected_index].get_name(),
                preloads.get_ref_roster()[self.p2_selected_index].get_name(),
            ];
            self.event_sender
                .send(Event::SetCharacters(characters))
                .unwrap();
            self.event_sender
                .send(Event::SetPalettes([self.p1_color, self.p2_color]))
                .unwrap();
            self.event_sender
                .send(Event::ChangeScreen(ScreenType::Fight))
                .unwrap();
        }
    }

    /// Maneja el evento de presionar una tecla en la pantalla de selección de personajes.
    ///
    /// # Argumentos
    ///
    /// * `key` - La tecla que se ha presionado.
    fn on_press(&mut self, key: piston_window::prelude::Key) {
        match key {
            Key::Up | Key::Down | Key::Left | Key::Right | Key::Z => {
                if key == Key::Z {
                    if !self.p1_selected {
                        self.p1_selected = true;
                    } else if self.p1_color == 0 {
                        self.p1_color = self.p1_index_color;
                        self.ticks = 0;
                    }
                }
                if self.p1_selected && self.p1_color == 0 {
                    if self.p1_index_color > 1 && key == Key::Left {
                        self.p1_index_color -= 1;
                    }
                    if self.p1_index_color < 6 && key == Key::Right {
                        self.p1_index_color += 1;
                    }
                }
            }
            Key::J | Key::I | Key::K | Key::L | Key::F => {
                if key == Key::F {
                    if !self.p2_selected {
                        self.p2_selected = true;
                    } else if self.p2_color == 0 {
                        self.p2_color = self.p2_index_color;
                        self.ticks = 0;
                    }
                }
                if self.p2_selected && self.p2_color == 0 {
                    if self.p2_index_color > 1 && key == Key::J {
                        self.p2_index_color -= 1;
                    }
                    if self.p2_index_color < 6 && key == Key::L {
                        self.p2_index_color += 1;
                    }
                }
            }
            Key::Escape => {
                self.event_sender
                    .send(Event::ChangeScreen(ScreenType::Main))
                    .unwrap();
            }
            _ => (),
        }
    }

    /// Maneja el evento de soltar una tecla en la pantalla de selección de personajes.
    ///
    /// # Argumentos
    ///
    /// * `key` - La tecla que se ha soltado.
    fn on_release(&mut self, key: piston_window::prelude::Key) {
        let _ = key;
    }

    /// Dibuja el contenido de la pantalla de selección de personajes.
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
        clear([1.0; 4], g);
        let mut preloads = self.preloads.lock().unwrap();
        image(
            preloads.get_mut_ref_background().get(0).unwrap(),
            c.transform,
            g,
        );
        if self.ticks / TICK_RESET % 2 == 0 {
            gui::draw_selector(c, g, self.p2_selected_index, false);
            gui::draw_selector(c, g, self.p1_selected_index, true);
        } else {
            gui::draw_selector(c, g, self.p1_selected_index, true);
            gui::draw_selector(c, g, self.p2_selected_index, false);
        }
        gui::draw_characters(c, g, preloads.get_ref_roster());

        if self.p1_selected {
            gui::draw_preview(c, g, device, &mut preloads, self.p1_selected_index, true);
            gui::draw_color_pick(
                c,
                g,
                device,
                &mut preloads,
                self.p1_index_color,
                true,
                self.p1_color > 0,
            );
        }

        if self.p2_selected {
            gui::draw_preview(c, g, device, &mut preloads, self.p2_selected_index, false);
            gui::draw_color_pick(
                c,
                g,
                device,
                &mut preloads,
                self.p2_index_color,
                false,
                self.p2_color > 0,
            );
        }
    }
}
