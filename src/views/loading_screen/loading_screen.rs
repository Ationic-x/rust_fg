use std::sync::mpsc::Sender;

use graphics::clear;
use piston_window::{Glyphs, PistonWindow, TextureSettings};

use crate::views::{common::Screen, screen_manager::Event};

use super::gui;

pub struct LoadingScreen {
    glyphs: Glyphs,
    event_sender: Sender<Event>,
    ticks: usize,
}

impl Screen for LoadingScreen {
    fn new(window: &mut PistonWindow, event_sender: Sender<Event>) -> Self
    where
        Self: Sized,
    {
        let glyphs = Glyphs::new(
            "assets\\fonts\\OpenSans-Regular.ttf",
            window.create_texture_context(),
            TextureSettings::new(),
        )
        .unwrap();

        Self {
            glyphs,
            event_sender,
            ticks: 0,
        }
    }

    fn update(&mut self, window: Option<&mut PistonWindow>) {
        let _ = window;
        if self.ticks == 0 {
            self.event_sender.send(Event::ScreenReady()).unwrap();
        }
        self.ticks += 1;
    }

    fn on_press(&mut self, key: piston_window::prelude::Key) {
        let _ = key;
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
        clear([0.0; 4], g);
        gui::draw_loading(c, g, device, &mut self.glyphs);
    }
}
