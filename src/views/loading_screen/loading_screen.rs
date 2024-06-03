use std::sync::{mpsc::Sender, Arc, Mutex};

use graphics::clear;
use piston_window::PistonWindow;

use crate::{preloader::preloader::Preloads, views::{common::Screen, screen_manager::Event}};

use super::gui;

pub struct LoadingScreen {
    preloads: Arc<Mutex<Preloads>>,
    event_sender: Sender<Event>,
    ticks: usize,
}

impl Screen for LoadingScreen {
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

    fn update(&mut self) {
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
        gui::draw_loading(c, g, device, self.preloads.lock().unwrap().get_mut_ref_fonts().get_mut(1).unwrap(),);
    }
}
