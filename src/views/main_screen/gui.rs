use gfx_device_gl::Device;
use graphics::{rectangle, text, CharacterCache, Context, Transformed};
use piston_window::{G2d, Glyphs};

const TITLE_GAME: &str = "RUST FG";
const OPTIONS: [&str; 3] = ["Versus", "Info", "Exit"];
const INFO_OPTIONS: [&str; 2] = ["GUI Controls", "Fight Controls"];
const FIGHT_CONTROLS: [[&str; 11]; 3] = [
    [
        "FORWARD", "BACK", "JUMP", "CROUCH", "LP", "MP", "HP", "LK", "MK", "HK", "TAUNT",
    ],
    [
        "RIGHT", "LEFT", "UP", "DOWN", "A", "S", "D", "Z", "X", "C", "RSHIFT",
    ],
    ["L", "J", "I", "K", "R", "T", "Y", "F", "G", "H", "RCTRL"],
];
const GUI_CONTROLS: [[&str; 3]; 7] = [
    ["ACCEPT", "RETURN/Z", "F"],
    ["DEBUG", "F1", ""],
    ["CANCEL/EXIT", "ESCAPE", ""],
    ["UP", "UP", "J"],
    ["DOWN", "DOWN", "K"],
    ["LEFT", "LEFT", "J"],
    ["RIGHT", "RIGHT", "L"],
];
const TITLE_SIZE: u32 = 50;
const TEXT_SIZE: u32 = 25;
const INFO_TEXT_SIZE: u32 = 20;
const WINDOW_SIZE: [f64; 2] = [512.0; 2];
const PADDING: f64 = 17.0;
const OPTION_MARGIN_TOP: f64 = 200.0;

/// Dibuja la información en pantalla, con opciones y controles.
///
/// # Parámetros
/// - `c`: El contexto de gráficos.
/// - `g`: La referencia mutable al gráfico 2D.
/// - `device`: El dispositivo gráfico.
/// - `glyphs`: Las fuentes para el texto.
/// - `index`: El índice de la opción seleccionada.
pub fn draw_info(c: Context, g: &mut G2d, device: &mut Device, glyphs: &mut Glyphs, index: usize) {
    let mut cummulative_width = 0.0;
    rectangle(
        [1.0, 1.0, 1.0, 0.7],
        [
            PADDING,
            PADDING,
            WINDOW_SIZE[0] - PADDING * 2.0,
            WINDOW_SIZE[1] - PADDING * 2.0,
        ],
        c.transform,
        g,
    );
    for (i, option) in INFO_OPTIONS.iter().enumerate() {
        let color = if i == index {
            [1.0, 0.4, 0.2, 1.0]
        } else {
            [0.0, 0.0, 0.0, 1.0]
        };

        let text_width = glyphs.width(TEXT_SIZE, option).unwrap();
        let text_height = TEXT_SIZE as f64;

        let x = PADDING * (3.0 + i as f64) + cummulative_width;
        let y = PADDING * 1.5 + text_height;
        let transform = c.transform.trans(x, y).zoom(0.5);

        cummulative_width = x + text_width;

        graphics::text::Text::new_color(color, TEXT_SIZE * 2)
            .draw(option, glyphs, &c.draw_state, transform, g)
            .unwrap();
    }

    if index == 1 {
        for (i, column) in FIGHT_CONTROLS.iter().enumerate() {
            for (j, control) in column.iter().enumerate() {
                let text_height = INFO_TEXT_SIZE as f64;

                let transform = c
                    .transform
                    .trans(
                        PADDING * 2.0 + 170.0 * (i as f64),
                        100.0 + j as f64 * (text_height + PADDING),
                    )
                    .zoom(0.5);

                graphics::text::Text::new_color([0.0, 0.0, 0.0, 1.0], INFO_TEXT_SIZE * 2)
                    .draw(control, glyphs, &c.draw_state, transform, g)
                    .unwrap();
            }
        }
    } else if index == 0 {
        for (i, column) in GUI_CONTROLS.iter().enumerate() {
            cummulative_width = 0.0;
            for (j, control) in column.iter().enumerate() {
                let text_width = glyphs.width(INFO_TEXT_SIZE, control).unwrap();
                let text_height = INFO_TEXT_SIZE as f64;

                let x = PADDING * 2.0 + cummulative_width;
                let y = 100.0 + i as f64 * (text_height + PADDING);
                let transform = c
                    .transform
                    .trans(
                        x,
                        y
                    )
                    .zoom(0.5);
                cummulative_width = x + text_width;
                if cummulative_width < 170.0 * (1.0 + j as f64){
                    cummulative_width = 170.0 * (1.0 + j as f64);
                }

                graphics::text::Text::new_color([0.0, 0.0, 0.0, 1.0], INFO_TEXT_SIZE * 2)
                    .draw(control, glyphs, &c.draw_state, transform, g)
                    .unwrap();
            }
        }
    }

    glyphs.factory.encoder.flush(device);
}

/// Dibuja las opciones de menú.
///
/// # Parámetros
/// - `c`: El contexto de gráficos.
/// - `g`: La referencia mutable al gráfico 2D.
/// - `device`: El dispositivo gráfico.
/// - `glyphs`: Las fuentes para el texto.
/// - `index`: El índice de la opción seleccionada.
pub fn draw_options(
    c: Context,
    g: &mut G2d,
    device: &mut Device,
    glyphs: &mut Glyphs,
    index: usize,
) {
    for (i, option) in OPTIONS.iter().enumerate() {
        let color = if i == index {
            [0.9, 0.8, 0.3, 1.0]
        } else {
            [1.0, 1.0, 1.0, 1.0]
        };

        let text_width = glyphs.width(TEXT_SIZE, option).unwrap();
        let text_height = TEXT_SIZE as f64;

        let transform = c
            .transform
            .trans(
                WINDOW_SIZE[0] / 2.0 - text_width / 2.0,
                OPTION_MARGIN_TOP + i as f64 * (text_height + PADDING),
            )
            .zoom(0.5);

        graphics::text::Text::new_color(color, TEXT_SIZE * 2)
            .draw(option, glyphs, &c.draw_state, transform, g)
            .unwrap();
    }

    glyphs.factory.encoder.flush(device);
}

/// Dibuja el título del juego.
///
/// # Parámetros
/// - `c`: El contexto de gráficos.
/// - `g`: La referencia mutable al gráfico 2D.
/// - `device`: El dispositivo gráfico.
/// - `glyphs`: Las fuentes para el texto.
pub fn draw_title(c: Context, g: &mut G2d, device: &mut Device, glyphs: &mut Glyphs) {
    let text_width = glyphs.width(TITLE_SIZE, TITLE_GAME).unwrap();
    let text_height = TITLE_SIZE as f64;

    let transform = c
        .transform
        .trans(
            WINDOW_SIZE[0] / 2.0 - text_width / 2.0,
            WINDOW_SIZE[1] / 4.0 + text_height / 4.0,
        )
        .zoom(0.5);

    text::Text::new_color([1.0, 1.0, 1.0, 1.0], TITLE_SIZE * 2)
        .draw(TITLE_GAME, glyphs, &c.draw_state, transform, g)
        .unwrap();

    glyphs.factory.encoder.flush(device);
}
