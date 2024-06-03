use gfx_device_gl::Device;
use graphics::{text, CharacterCache, Context, Transformed};
use piston_window::{G2d, Glyphs};

const TITLE_GAME: &str = "RUST FG";
const OPTIONS: [&str;2] = ["Versus", "Exit"];
const TITLE_SIZE: u32 = 50;
const TEXT_SIZE: u32 = 25;
const WINDOW_WIDTH: f64 = 512.0;
const WINDOW_HEIGHT: f64 = 512.0;
const PADDING: f64 = 20.0;
const OPTION_MARGIN_TOP: f64 = 200.0;

pub fn draw_options(c: Context, g: &mut G2d, device: &mut Device, glyphs: &mut Glyphs, index: usize) {
    for (i, option) in OPTIONS.iter().enumerate() {
        let color = if i == index {
            [0.9, 0.8, 0.3, 1.0]
        } else {
            [1.0, 1.0, 1.0, 1.0]
        };

        let text_width = glyphs.width(TEXT_SIZE, option).unwrap();
        let text_height = TEXT_SIZE as f64;

        let transform = c.transform.trans(
            WINDOW_WIDTH / 2.0 - text_width / 2.0,
            OPTION_MARGIN_TOP + i as f64 * (text_height + PADDING)
        ).zoom(0.5);

        graphics::text::Text::new_color(color, TEXT_SIZE * 2)
            .draw(
                option,
                glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();
    }

    glyphs.factory.encoder.flush(device);
}

pub fn draw_title(c: Context, g: &mut G2d, device: &mut Device, glyphs: &mut Glyphs) {
    let text_width = glyphs.width(TITLE_SIZE, TITLE_GAME).unwrap();
    let text_height = TITLE_SIZE as f64;

    let transform = c.transform.trans(WINDOW_WIDTH / 2.0  - text_width / 2.0, WINDOW_HEIGHT / 4.0 + text_height / 4.0).zoom(0.5);

    text::Text::new_color([1.0, 1.0, 1.0, 1.0], TITLE_SIZE * 2)
        .draw(
            TITLE_GAME,
            glyphs,
            &c.draw_state,
            transform,
            g,
        )
        .unwrap();

    glyphs.factory.encoder.flush(device);
}