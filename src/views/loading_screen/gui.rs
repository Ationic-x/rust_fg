use gfx_device_gl::Device;
use graphics::{text, CharacterCache, Context, Transformed};
use piston_window::{G2d, Glyphs};

const LOADING_SIZE: u32 = 20;
const LOADING_TEXT: &str = "Loading...";
const WINDOW_SIZE: [f64;2] = [512.0;2];

pub fn draw_loading(c: Context, g: &mut G2d, device: &mut Device, glyphs: &mut Glyphs) {
    let text_width = glyphs.width(LOADING_SIZE, LOADING_TEXT).unwrap();
    let text_height = LOADING_SIZE as f64;

    let transform = c.transform.trans(WINDOW_SIZE[0] / 2.0  - text_width / 2.0, WINDOW_SIZE[1] / 2.0 - text_height / 2.0);

    text::Text::new_color([1.0, 1.0, 1.0, 1.0], LOADING_SIZE)
        .draw(
            LOADING_TEXT,
            glyphs,
            &c.draw_state,
            transform,
            g,
        )
        .unwrap();

    glyphs.factory.encoder.flush(device);
}