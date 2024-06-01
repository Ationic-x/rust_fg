use std::{process, sync::mpsc::Sender};

use graphics::clear;
use piston::Key ;
use piston_window::{Glyphs, PistonWindow, TextureSettings};

use crate::views::{common::Screen, screen_manager::{Event, ScreenType}};

use super::gui;


pub struct MainScreen{
    glyphs: Glyphs,
    selected_index: usize,
    event_sender: Sender<Event>,
}

impl Screen for MainScreen {
    fn new(window: &mut PistonWindow, event_sender: Sender<Event>) -> Self where Self: Sized {
        let glyphs = Glyphs::new("assets\\fonts\\OpenSans-ExtraBold.ttf", window.create_texture_context(), TextureSettings::new()).unwrap();

        Self {
            glyphs,
            selected_index: 0,
            event_sender,
        }
    }

    fn update(&mut self, window: Option<&mut PistonWindow>) {
        let _ = window;
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
            Key::Return | Key::Z => {
                match self.selected_index {
                    0 => self.event_sender.send(Event::ChangeScreen(ScreenType::Roster)).unwrap(),
                    1 => {process::exit(0)},
                    _ => (),
                }
            }
            Key::Escape => {
                process::exit(0);
            }
            _ => (),
        }
    }

    fn on_release(&mut self, key: piston_window::prelude::Key) {
        let _ = key;
    }

    fn draw(&mut self, c: graphics::Context, g: &mut piston_window::prelude::G2d, device: &mut gfx_device_gl::Device) {
        clear([1.0; 4], g);
        gui::draw_title(c, g, device, &mut self.glyphs);
        gui::draw_options(c, g, device, &mut self.glyphs, self.selected_index);
    }
}