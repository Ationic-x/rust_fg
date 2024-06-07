

extern crate image;
extern crate piston_window;

use piston_window::*;
use winit::window::WindowButtons;

/// Máximo de frames por segundo (FPS) para la ventana del juego.
const FPS: u64 = 60;


use rust_fg::views::screen_manager::{ScreenManager, ScreenType};

fn new_window(title: &str, size: [f64;2]) -> PistonWindow{
    WindowSettings::new(title, size)
        .resizable(false)
        .samples(8)
        .build()
        .unwrap()
}

/// Punto de entrada de la aplicación.
fn main() {
    let mut window: PistonWindow = new_window("Rust_FG", [512.0;2]);

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