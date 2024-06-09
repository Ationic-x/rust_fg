

extern crate image;
extern crate piston_window;

use std::path::Path;

use piston_window::*;
use winit::window::{Icon, WindowButtons};
use rust_fg::views::screen_manager::{ScreenManager, ScreenType};

/// Máximo de frames por segundo (FPS) para la ventana del juego.
const FPS: u64 = 60;

/// Función para crear un ventana por defecto tipo Piston
fn new_window(title: &str, size: [f64;2]) -> PistonWindow{
    WindowSettings::new(title, size)
        .resizable(false)
        .samples(8)
        .build()
        .unwrap()
}

/// Función para cargar y crar un Struct tipo icon
fn load_icon(path: &Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path).unwrap().into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

/// Punto de entrada de la aplicación.
fn main() {
    let mut window: PistonWindow = new_window("Rust_FG", [512.0;2]);
    let icon_path = Path::new("assets/icon/icon.png");
    let window_icon = load_icon(icon_path);

    let conf_window: &winit::window::Window = &window.window.window;
    conf_window.set_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE);
    conf_window.set_window_icon(Some(window_icon));
    window.events.set_max_fps(FPS);
    window.events.set_ups(FPS);

    let mut screen_manager = ScreenManager::new(ScreenType::Main, &mut window);

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