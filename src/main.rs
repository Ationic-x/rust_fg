pub mod character;
pub mod input;

extern crate image;
extern crate piston_window;

use character::{commands, sff};
use input::manager::InputManager;
use piston_window::*;
use sprite::*;
use std::time::Duration;
use std::time::Instant;
use winit::window::WindowButtons;

const FPS: u64 = 60;
const PAUSE_DURATION: i32 = 3;

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
    let mut window: PistonWindow = WindowSettings::new("Rust FG", [512; 2])
        .resizable(false)
        .build()
        .unwrap();

    let conf_window: &winit::window::Window = &window.window.window;
    conf_window.set_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE);
    window.events.set_max_fps(FPS);
    window.events.set_ups(FPS);

    // ---------------------------
    // ---------------------------
    // ---------------------------

    let sprite_manager = sff::manager::SpriteManager::new("sf3_ken".to_string()).unwrap();

    let mut context = window.create_texture_context();

    let mut sprite = Sprite::from_texture(sprite_manager.get_texture(&mut context, 105, 2));
    sprite.set_anchor(0.0, 1.0);
    sprite.set_scale(1.5, 1.5);

    // ---------------------------
    // ---------------------------
    // ---------------------------

    let mut coord = (0.0,512.0);

    let mut input_manager = InputManager::new();
    let mut ticks = 0;

    let tree = commands::create_command_tree("example");
    tree.print(0);

    let mut debug = false;

    let mut total_frames = -1;
    let mut action_timer = 0;
    let mut enable_action_timer = false;
    let mut last_print_time = Instant::now();

    let mut last_update = Instant::now();
    let mut total_time = Duration::new(0, 0);

    while let Some(e) = window.next() {
        if let Some(_) = e.update_args() {
            // Delta time
            let delta_time = last_update.elapsed();
            last_update = Instant::now();
            total_time += delta_time;
            //

            input_manager.update_hold_key();
            ticks += 1u16;
            
            if action_timer < PAUSE_DURATION {
                action_timer += 1;
            } else if enable_action_timer {
                input_manager.walk_input_buffer(&tree);
                enable_action_timer = false;
            }

            // Commmon moves
            let player_input = &input_manager.player_input;
            let speed = 150.0; // Velocidad en unidades por segundo
            let delta_seconds = delta_time.as_secs_f64();
            if player_input.f {
                coord.0 += speed * delta_seconds;
            }
            if player_input.b {
                coord.0 -= speed * delta_seconds;
                sprite.set_texture(sprite_manager.get_texture(&mut context, 10, 1));
            }
            if player_input.u {
                coord.1 -= speed * delta_seconds;
            }
            //

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

        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);

            sprite.set_position(coord.0, coord.1);
            sprite.draw(c.transform, g);
        });
    }
}
