mod player;
mod chars;
mod views;
mod preloader;

extern crate image;
extern crate piston_window;

use piston_window::*;
use views::screen_manager::{ScreenManager, ScreenType};
use preloader::preloader::Preloads;
use winit::window::WindowButtons;

const FPS: u64 = 60;

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum CK {
    DB,
    D,
    DF,
    B,
    F,
    UB,
    U,
    UF,
    LP,
    MP,
    HP,
    LK,
    MK,
    HK,
    Start,
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Rust FG", [512; 2])
        .resizable(false)
        .samples(8)
        .build()
        .unwrap();

    let conf_window: &winit::window::Window = &window.window.window;
    conf_window.set_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE);
    window.events.set_max_fps(FPS);
    window.events.set_ups(FPS);

    let mut screen_manager = ScreenManager::new(ScreenType::Main, &mut window);

    //let at_mut = at.get(0).unwrap();
    while let Some(e) = window.next() {
        if let Some(_) = e.update_args() {
            screen_manager.update(&mut window);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            screen_manager.on_press(key);
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            screen_manager.on_release(key);
        }

        window.draw_2d(&e, |c, g, device| {
            screen_manager.draw(c, g, device);
        });
    }
}