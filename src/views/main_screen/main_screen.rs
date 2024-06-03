use std::{process, sync::{mpsc::Sender, Arc, Mutex}};

use graphics::{clear, image};
use piston::Key;
use piston_window::PistonWindow;

use crate::{preloader::preloader::Preloads, views::{
    common::Screen,
    screen_manager::{Event, ScreenType},
}};

use super::gui;

pub struct MainScreen {
    preloads: Arc<Mutex<Preloads>>,
    selected_index: usize,
    event_sender: Sender<Event>,
}

impl Screen for MainScreen {
    fn new(event_sender: Sender<Event>, preloads: Arc<Mutex<Preloads>>) -> Self
    where
        Self: Sized,
    {

        Self {
            preloads,
            selected_index: 0,
            event_sender,
        }
    }

    fn update(&mut self) {
    }

    fn on_press(&mut self, key: piston_window::prelude::Key) {
        match key {
            Key::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            Key::Down => {
                if self.selected_index < 1 {
                    self.selected_index += 1;
                }
            }
            Key::Return | Key::Z => match self.selected_index {
                0 => self
                    .event_sender
                    .send(Event::ChangeScreen(ScreenType::Roster))
                    .unwrap(),
                1 => process::exit(0),
                _ => (),
            },
            Key::Escape => {
                process::exit(0);
            }
            _ => (),
        }
    }

    fn on_release(&mut self, key: piston_window::prelude::Key) {
        let _ = key;
    }

    fn draw(
        &mut self,
        c: graphics::Context,
        g: &mut piston_window::prelude::G2d,
        device: &mut gfx_device_gl::Device,
    ) {
        clear([1.0; 4], g);
        let mut preload = self.preloads.lock().unwrap();
        image(preload.get_mut_ref_background().get(0).unwrap(), c.transform, g);
        let glyphs = preload.get_mut_ref_fonts().get_mut(0).unwrap();
        gui::draw_title(c, g, device, glyphs);
        gui::draw_options(c, g, device, glyphs, self.selected_index);
    }
}
