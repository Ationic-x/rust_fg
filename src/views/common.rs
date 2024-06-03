use std::sync::{mpsc::Sender, Arc, Mutex};

use gfx_device_gl::Device;
use piston::Key;
use piston_window::{Context, G2d};

use crate::preloader::preloader::Preloads;

use super::screen_manager::Event;

pub trait Screen {
    fn new(event_sender: Sender<Event>, preloads: Arc<Mutex<Preloads>>) -> Self where Self: Sized;
    fn update(&mut self);
    fn on_press(&mut self, key: Key);
    fn on_release(&mut self, key: Key);
    fn draw(&mut self, c: Context, g: &mut G2d, device: &mut Device);
}