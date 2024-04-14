pub mod input;
pub mod character;

use character::commands;
use input::manage::InputManager;
use piston_window::*;
use sprite::*;
use std::{
    rc::Rc,
    time::Instant,
};
use winit::window::WindowButtons;

const FPS: u64 = 60;
const PAUSE_DURATION: i32 = 3;

// CK refer to command keys avaible commands in a fight
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
}

fn main() {
    // --------------------------------------------
    // - CREATE WINDOW
    // --------------------------------------------
    // Size window
    let window_size = [512; 2];
    // Making the window were to play
    let mut window: PistonWindow = WindowSettings::new("Square Game", window_size)
        .resizable(false)
        .build()
        .unwrap();

    // Shorter reference to window
    let conf_window: &winit::window::Window = &window.window.window;

    // Extra settings
    // Disable maximize option
    conf_window.set_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE);

    // Creating a texture context of the PistonWindow
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };

    // --------------------------------------------
    // - CREATE SPRITE
    // --------------------------------------------
    // Getting folder of assets
    let assets = std::env::current_dir().unwrap().join("src").join("assets");

    // Creating a texture of the sprite inside assets and the window
    let texture = Rc::new(
        Texture::from_path(
            &mut texture_context,
            assets.join("HotaruFutaba_861.png"),
            Flip::Horizontal,
            &TextureSettings::new(),
        )
        .unwrap(),
    );
    // Getting the sprite from texture
    let sprite = Sprite::from_texture(texture);
    // Getting the position of the sprite
    let sprite_coord = sprite.get_position();

    let mut input_manager = InputManager::new();
    let mut ticks = 0;

    let tree = commands::create_command_tree("example");

    tree.print(0);

    let mut debug = false;

    window.events.set_max_fps(FPS);
    window.events.set_ups(FPS);
    let mut total_frames = -1;
    let mut action_timer = 0;
    let mut enable_action_timer = false;
    let mut last_print_time = Instant::now();

    // --------------------------------------------
    // - LOOP WINDOW
    // --------------------------------------------
    while let Some(e) = window.next() {
        if let Some(_) = e.update_args() {
            input_manager.update_hold_key();
            ticks += 1u16;
            if action_timer < PAUSE_DURATION {
                action_timer += 1;
            } else if enable_action_timer {
                input_manager.walk_input_buffer(&tree);
                enable_action_timer = false;
            }
            if debug {
                total_frames += 1;
                let elapsed_seconds = last_print_time.elapsed().as_secs();
                if elapsed_seconds > 0 {
                    let average_fps = (total_frames as f64) / (elapsed_seconds as f64);
                    println!("FPS: {:}", average_fps as u64);
                    total_frames = -1;
                    last_print_time = Instant::now();
                }
            }
        }

        // Read Key pressed
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Up
                | Key::Down
                | Key::Left
                | Key::Right
                | Key::A
                | Key::S
                | Key::D
                | Key::Z
                | Key::X
                | Key::C => {
                    if input_manager.set_player_input(&key, true) {
                        return;
                    }
                    input_manager.handle_key_input(&mut ticks, enable_action_timer);
                    if ![Key::Up, Key::Down, Key::Left, Key::Right].contains(&key)
                        && !enable_action_timer
                    {
                        action_timer = 0;
                        enable_action_timer = true;
                    }
                }
                Key::F1 => {
                    debug = !debug;
                    if debug {
                        println!("FPS: {:}", 0);
                        total_frames = -1;
                        last_print_time = Instant::now();
                    }
                }
                _ => {}
            }
        }

        // Read Key released
        if let Some(Button::Keyboard(key)) = e.release_args() {
            match key {
                Key::Up
                | Key::Down
                | Key::Left
                | Key::Right
                | Key::A
                | Key::S
                | Key::D
                | Key::Z
                | Key::X
                | Key::C => {
                    if !input_manager.set_player_input(&key, false) {
                        return;
                    }
                    input_manager.handle_key_input(&mut ticks, false);
                }
                _ => {}
            }
        }

        // Update the window image, redraw all the sprites
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);
            sprite.draw(c.transform.trans(sprite_coord.0, sprite_coord.1), g);
        });
    }
}