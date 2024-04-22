pub mod character;
pub mod input;

extern crate image;
extern crate piston_window;

use character::commands;
use character::decoder;
use image::RgbaImage;
use input::manage::InputManager;
use piston_window::*;
use sprite::*;
use std::{rc::Rc, time::Instant};
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

    let assets = std::env::current_dir().unwrap().join("src").join("assets");

    // ---------------------------
    // ---------------------------
    // ---------------------------

    let sff_path = assets.join("sf3_ken.sff");
    let sff = std::fs::read(sff_path).expect("Failed to read SFF file");
    let sff = match decoder::Decoder::decode(&sff) {
        Ok(decoded_data) => decoded_data,
        Err(error) => {
            println!("Error al decodificar sff: {:?}", error);
            return;
        }
    };
    println!("{}-{}", sff.groups_count(), sff.images_count());
    let sprites = sff.sprites().collect::<Vec<_>>();
    let sprited = sprites
        .iter()
        .find(|sprite| sprite.id().group == 105 && sprite.id().image == 2)
        .unwrap();
    println!("{:?}", sprited.id());

    let mut pcx = match pcx::Reader::new(sprited.raw_data()) {
        Ok(pcx) => pcx,
        Err(error) => {
            println!("Error al decodificar PCX: {:?}", error);
            return;
        }
    };

    let width = pcx.width() as usize;
    let height = pcx.height() as usize;

    let palette = sprited
        .palette()
        .chunks_exact(3)
        .map(|i| [i[0], i[1], i[2], 255])
        .collect::<Vec<_>>();

    let mut data = vec![0; width * height];
    for row in 0..height {
        pcx.next_row_paletted(&mut data[row * width..row * width + width])
            .expect("Pallete");
    }

    let rgba = data
        .into_iter()
        .flat_map(|i| match i as usize {
            0 => [0, 0, 0, 0],
            i => palette[i],
        })
        .collect::<Vec<_>>();

    let img = &RgbaImage::from_raw(width as u32, height as u32, rgba).unwrap();

    let algo = Texture::from_image(
        &mut window.create_texture_context(),
        img,
        &TextureSettings::new().filter(piston_window::Filter::Nearest),
    )
    .unwrap();

    let texture_rc = Rc::new(algo);
    let mut sprite = Sprite::from_texture(texture_rc.clone());

    // ---------------------------
    // ---------------------------
    // ---------------------------

    let mut input_manager = InputManager::new();
    let mut ticks = 0;

    let tree = commands::create_command_tree("example");
    tree.print(0);

    let mut debug = false;

    let mut total_frames = -1;
    let mut action_timer = 0;
    let mut enable_action_timer = false;
    let mut last_print_time = Instant::now();

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

            sprite.set_scale(2.0, 2.0);
            sprite.set_position(100.0, 100.0);
            sprite.draw(c.transform, g)
        });
    }
}
