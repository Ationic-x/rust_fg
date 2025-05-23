use gfx_device_gl::Device;
use graphics::{line, rectangle, text, CharacterCache, Context, Transformed};
use piston_window::G2d;
use sprite::Sprite;
use std::{rc::Rc, sync::MutexGuard};

use crate::{player::character::sff::decoder::Sff, preloader::preloader::Preloads};

const KEY_CHAR: [i16; 2] = [9000, 0];
const KEY_PREVIEW: [i16; 2] = [9000, 1];
const WINDOW_SIZE: [f64; 2] = [512.0; 2];
const CHARACTER_SIZE: [f64; 2] = [25.0; 2];
const PREVIEW_SIZE: [f64; 2] = [120.0, 140.0];
const PADDING: [f64; 2] = [100.0; 2];
const INTERNAL_SEPARATION: [f64; 2] = [10.0; 2];
const P1_SELECTOR_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const P2_SELECTOR_COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const LINE_WIDTH: f64 = 2.0;
const TEXT_SIZE: u32 = 40;
const COLOR_TEXT_SIZE: u32 = 25;

/// Dibuja el selector de color para el personaje.
/// 
/// # Parámetros
/// - `c`: El contexto de gráficos.
/// - `g`: La referencia mutable al gráfico 2D.
/// - `device`: El dispositivo gráfico.
/// - `preloads`: Referencia mutable a las cargas previas.
/// - `index`: El índice del color.
/// - `first_player`: Indica si es el primer jugador.
/// - `picked`: Indica si el color ha sido seleccionado.
pub fn draw_color_pick(
    c: Context,
    g: &mut G2d,
    device: &mut Device,
    preloads: &mut MutexGuard<Preloads>,
    index: usize,
    first_player: bool,
    picked: bool,
) {
    let glyphs = preloads.get_mut_ref_fonts().get_mut(0).unwrap();
    let text_pick = glyphs.width(15, "Pick a color").unwrap();
    let x;
    if first_player {
        x = PREVIEW_SIZE[0] + 10.0;
    } else {
        x = WINDOW_SIZE[0] - PREVIEW_SIZE[0] - text_pick - 10.0;
    }

    let transform = c
        .transform
        .trans(x, WINDOW_SIZE[1] - PREVIEW_SIZE[1] / 4.0).zoom(0.5);
    
    text::Text::new_color([1.0, 1.0, 1.0, 1.0], 15 * 2)
        .draw("Pick a color", glyphs, &c.draw_state, transform, g)
        .unwrap();
    
    let text = format!("Color {}", index);
    let text_width = glyphs.width(COLOR_TEXT_SIZE, &text).unwrap();
    let x;
    if first_player {
        x = PREVIEW_SIZE[0] + 10.0;
    } else {
        x = WINDOW_SIZE[0] - PREVIEW_SIZE[0] - text_width - 10.0;
    }

    let transform = c
        .transform
        .trans(x, WINDOW_SIZE[1] - PREVIEW_SIZE[1] / 20.0).zoom(0.5);
    
    let color;
    if picked {
        color = [0.9, 0.3, 0.5, 1.0]
    } else {
        color = [1.0, 1.0, 1.0, 1.0]
    }

    text::Text::new_color(color, COLOR_TEXT_SIZE * 2)
        .draw(&text, glyphs, &c.draw_state, transform, g)
        .unwrap();

    glyphs.factory.encoder.flush(device);
}

/// Dibuja la vista previa del personaje.
/// 
/// # Parámetros
/// - `c`: El contexto de gráficos.
/// - `g`: La referencia mutable al gráfico 2D.
/// - `device`: El dispositivo gráfico.
/// - `preloads`: Referencia mutable a las cargas previas.
/// - `index`: El índice del personaje.
/// - `first_player`: Indica si es el primer jugador.
pub fn draw_preview(
    c: Context,
    g: &mut G2d,
    device: &mut Device,
    preloads: &mut MutexGuard<Preloads>,
    index: usize,
    first_player: bool,
) {
    let roster = preloads.get_ref_roster();
    if roster.is_empty() {
        return;
    }

    let mut x;
    if let Some(spr_data) = roster[index].sprites.get(&KEY_PREVIEW) {
        if let Some(texture) = &spr_data.tex {
            let mut sprite = Sprite::from_texture(Rc::clone(texture));
            if first_player {
                sprite.set_anchor(0.0, 1.0);
                x = 0.0;
            } else {
                sprite.set_anchor(1.0, 1.0);
                x = WINDOW_SIZE[0];
            }
            sprite.draw(c.transform.trans(x, WINDOW_SIZE[1]), g);
        }
    }

    if first_player {
        x = 0.0
    } else {
        x = WINDOW_SIZE[0] - PREVIEW_SIZE[0];
    }

    let background_rect = rectangle::rectangle_by_corners(
        x,
        WINDOW_SIZE[1] - PREVIEW_SIZE[1] / 3.5,
        x + PREVIEW_SIZE[0],
        WINDOW_SIZE[1] + 5.0,
    );

    if first_player {
        x = 0.0 + PREVIEW_SIZE[0] / 7.0;
    } else {
        x = WINDOW_SIZE[0] - PREVIEW_SIZE[0] * 6.0 / 7.0;
    }

    rectangle([1.0, 1.0, 1.0, 0.5], background_rect, c.transform, g);

    let text = roster[index].get_name();
    
    let glyphs = preloads.get_mut_ref_fonts().get_mut(0).unwrap();
    let transform = c
        .transform
        .trans(x, WINDOW_SIZE[1] - PREVIEW_SIZE[1] / 20.0).zoom(0.5);

    text::Text::new_color([0.0, 0.0, 0.0, 1.0], TEXT_SIZE * 2)
        .draw(&text, glyphs, &c.draw_state, transform, g)
        .unwrap();

    glyphs.factory.encoder.flush(device);
}

/// Dibuja el selector de personaje.
/// 
/// # Parámetros
/// - `c`: El contexto de gráficos.
/// - `g`: La referencia mutable al gráfico 2D.
/// - `index`: El índice del personaje.
/// - `first_player`: Indica si es el primer jugador.
pub fn draw_selector(c: Context, g: &mut G2d, index: usize, first_player: bool) {
    let characters_per_row = ((WINDOW_SIZE[0] - 2.0 * PADDING[0])
        / (CHARACTER_SIZE[0] + INTERNAL_SEPARATION[0]))
        .floor() as usize;

    let start_x = (WINDOW_SIZE[0]
        - (characters_per_row as f64 * (CHARACTER_SIZE[0] + INTERNAL_SEPARATION[0])
            - INTERNAL_SEPARATION[0]))
        / 2.0;
    let start_y = (WINDOW_SIZE[1]
        - (characters_per_row as f64 * (CHARACTER_SIZE[1] + INTERNAL_SEPARATION[1])
            - INTERNAL_SEPARATION[1]))
        / 2.0;

    let x = start_x
        + (index % characters_per_row) as f64 * (CHARACTER_SIZE[0] + INTERNAL_SEPARATION[0])
        - INTERNAL_SEPARATION[0] / 2.0;
    let y = start_y
        + (index / characters_per_row) as f64 * (CHARACTER_SIZE[1] + INTERNAL_SEPARATION[1])
        - INTERNAL_SEPARATION[1] / 2.0;

    let rect = [
        x,
        y,
        CHARACTER_SIZE[0] + INTERNAL_SEPARATION[0],
        CHARACTER_SIZE[1] + INTERNAL_SEPARATION[1],
    ];

    let color = if first_player {
        P1_SELECTOR_COLOR
    } else {
        P2_SELECTOR_COLOR
    };

    line(
        color,
        LINE_WIDTH,
        [rect[0], rect[1], rect[0] + rect[2], rect[1]],
        c.transform,
        g,
    );
    line(
        color,
        LINE_WIDTH,
        [
            rect[0],
            rect[1] + rect[3],
            rect[0] + rect[2],
            rect[1] + rect[3],
        ],
        c.transform,
        g,
    );
    line(
        color,
        LINE_WIDTH,
        [rect[0], rect[1], rect[0], rect[1] + rect[3]],
        c.transform,
        g,
    );
    line(
        color,
        LINE_WIDTH,
        [
            rect[0] + rect[2],
            rect[1],
            rect[0] + rect[2],
            rect[1] + rect[3],
        ],
        c.transform,
        g,
    );
}

/// Dibuja los personajes disponibles.
/// 
/// # Parámetros
/// - `c`: El contexto de gráficos.
/// - `g`: La referencia mutable al gráfico 2D.
/// - `roster`: La lista de personajes.
pub fn draw_characters(c: Context, g: &mut G2d, roster: &Vec<Sff>) {
    if roster.is_empty() {
        return;
    }

    let characters_per_row = ((WINDOW_SIZE[0] - 2.0 * PADDING[0])
        / (CHARACTER_SIZE[0] + INTERNAL_SEPARATION[0]))
        .floor() as usize;
    let characters_per_col = ((WINDOW_SIZE[1] - 2.0 * PADDING[1])
        / (CHARACTER_SIZE[1] + INTERNAL_SEPARATION[1]))
        .floor() as usize;

    let start_x = (WINDOW_SIZE[0]
        - (characters_per_row as f64 * (CHARACTER_SIZE[0] + INTERNAL_SEPARATION[0])
            - INTERNAL_SEPARATION[0]))
        / 2.0;
    let start_y = (WINDOW_SIZE[1]
        - (characters_per_col as f64 * (CHARACTER_SIZE[1] + INTERNAL_SEPARATION[1])
            - INTERNAL_SEPARATION[1]))
        / 2.0;

    for (i, sff) in roster.iter().enumerate() {
        if let Some(spr_data) = sff.sprites.get(&KEY_CHAR) {
            if let Some(texture) = &spr_data.tex {
                let x = start_x
                    + (i % characters_per_row) as f64
                        * (CHARACTER_SIZE[0] + INTERNAL_SEPARATION[0]);
                let y = start_y
                    + (i / characters_per_row) as f64
                        * (CHARACTER_SIZE[1] + INTERNAL_SEPARATION[1]);

                let mut sprite = Sprite::from_texture(Rc::clone(texture));
                sprite.set_anchor(0.0, 0.0);
                sprite.draw(c.transform.trans(x, y), g);
            }
        }
    }
}
