use std::{process, sync::{mpsc::Sender, Arc, Mutex}};

use graphics::{clear, image};
use piston::Key;

use crate::{preloader::preloader::Preloads, views::{
    common::Screen,
    screen_manager::{Event, ScreenType},
}};

use super::gui;

pub struct MainScreen {
    preloads: Arc<Mutex<Preloads>>,
    selected_index: usize,
    event_sender: Sender<Event>,
    info_popup: bool,
    info_index: usize,
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
            info_popup: false,
            info_index: 0,
        }
    }

    fn update(&mut self) {
    }

    fn on_press(&mut self, key: piston_window::prelude::Key) {
        match key {
            Key::Left | Key::J => {
                if self.info_popup && self.info_index > 0{
                    self.info_index -= 1;
                }
            }
            Key::Right | Key::L => {
                if self.info_popup && self.info_index < 1 {
                    self.info_index += 1;
                }
            }
            Key::Up | Key::I => {
                if self.info_popup {
                    return;
                }
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            Key::Down | Key::K => {
                if self.info_popup {
                    return;
                }
                if self.selected_index < 2 {
                    self.selected_index += 1;
                }
            }
            Key::Return | Key::Z | Key::F => match self.selected_index {
                0 => self
                    .event_sender
                    .send(Event::ChangeScreen(ScreenType::Roster))
                    .unwrap(),
                1 => self.info_popup = true,
                2 => process::exit(0),
                _ => (),
            },
            Key::Escape => {
                if self.info_popup {
                    self.info_popup = false;
                    return;
                }
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
        if self.info_popup {
            gui::draw_info(c, g, device, glyphs, self.info_index);
        }
    }
}
