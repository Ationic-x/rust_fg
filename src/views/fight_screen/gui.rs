use std::f64::consts::PI;

use gfx_device_gl::Device;
use graphics::{ellipse, polygon, rectangle, text, CharacterCache, Context, Transformed};
use piston_window::{G2d, Glyphs};

const HEALTH_BAR_HEIGHT: f64 = 20.0;
const HEALTH_BAR_MAX_WIDTH: f64 = 200.0;
const PADDING: f64 = 10.0;
const TIMER_RADIUS: f64 = 30.0;
const TIMER_CENTER_X: f64 = HEALTH_BAR_MAX_WIDTH + TIMER_RADIUS + 25.0;
const TIMER_CENTER_Y: f64 = TIMER_RADIUS;
const TEXT_SIZE: u32 = 32;
const FPS_TEXT_SIZE: u32 = 15;

pub fn draw_power_bar(c: Context, g: &mut G2d, power: f64, is_player_one: bool) {
    let power_bar_width = (power / 100.0) * HEALTH_BAR_MAX_WIDTH / 2.0;
    let mut x_pos = PADDING;

    if !is_player_one {
        x_pos = 512.0 - HEALTH_BAR_MAX_WIDTH / 2.0 - PADDING;
    }

    let y_pos = PADDING * 4.0;

    let power_bar_background = [0.5, 0.5, 0.5, 1.0];
    let power_bar_foreground = [0.11, 0.87, 0.97, 1.0];

    rectangle(
        power_bar_background,
        [x_pos, y_pos, HEALTH_BAR_MAX_WIDTH / 2.0, HEALTH_BAR_HEIGHT / 2.0],
        c.transform,
        g,
    );

    let (bar_x_pos, bar_width) = if is_player_one {
        (x_pos, power_bar_width)
    } else {
        (
            x_pos + HEALTH_BAR_MAX_WIDTH / 2.0 - power_bar_width,
            power_bar_width,
        )
    };

    rectangle(
        power_bar_foreground,
        [bar_x_pos, y_pos, bar_width, HEALTH_BAR_HEIGHT / 2.0],
        c.transform,
        g,
    );
}


pub fn draw_health_bar(c: Context, g: &mut G2d, life: f64, is_player_one: bool) {
    let health_bar_width = (life / 100.0) * HEALTH_BAR_MAX_WIDTH;
    let mut x_pos = PADDING;

    if !is_player_one {
        x_pos = 512.0 - HEALTH_BAR_MAX_WIDTH - PADDING;
    }

    let y_pos = PADDING + PADDING;

    let health_bar_background = [0.5, 0.5, 0.5, 1.0];
    let health_bar_foreground = [0.97, 0.87, 0.11, 1.0];

    rectangle(
        health_bar_background,
        [x_pos, y_pos, HEALTH_BAR_MAX_WIDTH, HEALTH_BAR_HEIGHT],
        c.transform,
        g,
    );

    let (bar_x_pos, bar_width) = if is_player_one {
        (x_pos, health_bar_width)
    } else {
        (
            x_pos + HEALTH_BAR_MAX_WIDTH - health_bar_width,
            health_bar_width,
        )
    };

    rectangle(
        health_bar_foreground,
        [bar_x_pos, y_pos, bar_width, HEALTH_BAR_HEIGHT],
        c.transform,
        g,
    );
}

pub fn draw_countdown(c: Context, g: &mut G2d, device: &mut Device, glyphs: &mut Glyphs, timer: u32) {
    let progress = timer as f64 / 3.0;
    let end_angle = 2.0 * PI * progress;

    let center = [TIMER_CENTER_X, 255.0];
    let radius = TIMER_RADIUS * 2.0;

    ellipse(
        [0.5, 0.5, 0.5, 1.0],
        [
            center[0] - radius,
            center[1] - radius,
            radius * 2.0,
            radius * 2.0,
        ],
        c.transform,
        g,
    );

    let mut points = vec![center];
    for i in 0..=100 {
        let angle = (i as f64 / 100.0) * end_angle + PI;
        points.push([
            center[0] + radius * angle.sin(),
            center[1] + radius * angle.cos(),
        ]);
    }

    polygon(
        [0.3, 0.5, 0.8, 1.0],
        &points,
        c.transform,
        g,
    );

    let text = format!("{:.0}", timer);
    let text_width = glyphs.width(TEXT_SIZE, &text).unwrap();
    let text_height = TEXT_SIZE as f64;

    let transform = c.transform.trans(center[0] - text_width, center[1] + text_height / 2.0);

    text::Text::new_color([1.0, 1.0, 1.0, 1.0], TEXT_SIZE * 2)
        .draw(
            &text,
            glyphs,
            &c.draw_state,
            transform,
            g,
        )
        .unwrap();

    glyphs.factory.encoder.flush(device);
}

pub fn draw_timer(c: Context, g: &mut G2d, device: &mut Device, glyphs: &mut Glyphs, timer: u32) {
    let progress = timer as f64 / 100.0;
    let end_angle = 2.0 * PI * progress;

    let center = [TIMER_CENTER_X, TIMER_CENTER_Y];
    let radius = TIMER_RADIUS;

    ellipse(
        [0.5, 0.5, 0.5, 1.0],
        [
            center[0] - radius,
            center[1] - radius,
            radius * 2.0,
            radius * 2.0,
        ],
        c.transform,
        g,
    );

    let mut points = vec![center];
    for i in 0..=100 {
        let angle = (i as f64 / 100.0) * end_angle + PI;
        points.push([
            center[0] + radius * angle.sin(),
            center[1] + radius * angle.cos(),
        ]);
    }

    polygon(
        [0.3, 0.5, 0.8, 1.0],
        &points,
        c.transform,
        g,
    );

    let text = format!("{:.0}", timer);
    let text_width = glyphs.width(TEXT_SIZE, &text).unwrap();
    let text_height = TEXT_SIZE as f64;

    let transform = c.transform.trans(center[0] - text_width / 2.0, center[1] + text_height / 4.0);

    text::Text::new_color([1.0, 1.0, 1.0, 1.0], TEXT_SIZE)
        .draw(
            &text,
            glyphs,
            &c.draw_state,
            transform,
            g,
        )
        .unwrap();

    glyphs.factory.encoder.flush(device);
}

pub fn draw_fps(c: Context, g: &mut G2d, device: &mut Device, glyphs: &mut Glyphs, fps: f64) {
    let text = format!("FPS({})", fps);

    let transform = c.transform.trans(0.0, 200.0).zoom(0.5);

    text::Text::new_color([0.0, 1.0, 0.0, 1.0], FPS_TEXT_SIZE * 2)
        .draw(
            &text,
            glyphs,
            &c.draw_state,
            transform,
            g,
        )
        .unwrap();

    glyphs.factory.encoder.flush(device);
}

pub fn end_round(c: Context, g: &mut G2d, device: &mut Device, glyphs: &mut Glyphs, winner: u8) {
    let mut text = "Who won?";
    match winner {
        1 => {
            text = "Player 1 Won!";
        }
        2 => {
            text = "Player 2 Won!";
        }
        3 => {
            text = "It's a Draw!";
        }
        _ => {}
    }
    let text_width = glyphs.width(TEXT_SIZE, text).unwrap();
    let text_height = TEXT_SIZE as f64;

    let transform = c.transform.trans(500.0 / 2.0  - text_width / 2.0, 500.0 / 2.0 + text_height / 4.0).zoom(0.5);

    text::Text::new_color([0.0, 0.0, 0.0, 1.0], TEXT_SIZE * 2)
        .draw(
            &text,
            glyphs,
            &c.draw_state,
            transform,
            g,
        )
        .unwrap();

    glyphs.factory.encoder.flush(device);
}