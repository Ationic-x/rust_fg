use std::sync::mpsc::Sender;

use gfx_device_gl::Device;
use piston::Key;
use piston_window::{Context, G2d, PistonWindow};

use super::screen_manager::Event;

pub trait Screen {
    fn new(window: &mut PistonWindow, event_sender: Sender<Event>) -> Self where Self: Sized;
    fn update(&mut self, window: Option<&mut PistonWindow>);
    fn on_press(&mut self, key: Key);
    fn on_release(&mut self, key: Key);
    fn draw(&mut self, c: Context, g: &mut G2d, device: &mut Device);
}