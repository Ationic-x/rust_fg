use std::{fs, sync::mpsc::Sender};

use graphics::clear;
use piston::Key;
use piston_window::{Glyphs, PistonWindow, TextureSettings};

use crate::{
    chars,
    player::character::sff::decoder::Sff,
    views::{
        common::Screen,
        screen_manager::{Event, ScreenType},
    },
};

use super::gui;

const TICK_RESET: usize = 20;

pub struct RosterScreen {
    glyphs: Glyphs,
    p1_selected_index: usize,
    p1_selected: bool,
    p1_color: usize,
    p1_index_color: usize,
    p2_selected_index: usize,
    p2_selected: bool,
    p2_index_color: usize,
    p2_color: usize,
    roster: Vec<Sff>,
    ticks: usize,
    event_sender: Sender<Event>,
}

impl Screen for RosterScreen {
    fn new(window: &mut PistonWindow, event_sender: Sender<Event>) -> Self
    where
        Self: Sized,
    {
        let mut roster = Vec::new();

        let glyphs = Glyphs::new(
            "assets\\fonts\\OpenSans-Regular.ttf",
            window.create_texture_context(),
            TextureSettings::new(),
        )
        .unwrap();

        for entry in fs::read_dir("src\\chars\\").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    if let Some(char_name) = name.to_str() {
                        let char = chars::get_char(char_name).unwrap();
                        let context = window.create_texture_context();
                        roster.push(
                            Sff::preload_sff(
                                char_name,
                                char.get_sff_name().to_string() + ".sff",
                                true,
                                context,
                            )
                            .unwrap(),
                        );
                    }
                }
            }
        }

        Self {
            glyphs,
            p1_selected_index: 0,
            p2_selected_index: 0,
            roster,
            ticks: 0,
            p1_selected: false,
            p1_index_color: 1,
            p1_color: 0,
            p2_selected: false,
            p2_index_color: 1,
            p2_color: 0,
            event_sender,
        }
    }

    fn update(&mut self, window: Option<&mut PistonWindow>) {
        let _ = window;
        self.ticks += 1;
        if self.p1_color > 0 && self.p2_color > 0 && self.ticks > 20 {
            let characters = [
                self.roster[self.p1_selected_index].get_name(),
                self.roster[self.p2_selected_index].get_name(),
            ];
            self.event_sender
                .send(Event::SetCharacters(characters))
                .unwrap();
            self.event_sender
                .send(Event::SetPalettes([self.p1_color, self.p2_color]))
                .unwrap();
            self.event_sender
                .send(Event::ChangeScreen(ScreenType::Fight))
                .unwrap();
        }
    }

    fn on_press(&mut self, key: piston_window::prelude::Key) {
        match key {
            Key::Up | Key::Down | Key::Left | Key::Right | Key::Z => {
                if key == Key::Z {
                    if !self.p1_selected {
                        self.p1_selected = true;
                    } else if self.p1_color == 0 {
                        self.p1_color = self.p1_index_color;
                        self.ticks = 0;
                    }
                }
                if self.p1_selected && self.p1_color == 0 {
                    if self.p1_index_color > 1 && key == Key::Left {
                        self.p1_index_color -= 1;
                    }
                    if self.p1_index_color < 6 && key == Key::Right {
                        self.p1_index_color += 1;
                    }
                }
            }
            Key::J | Key::I | Key::K | Key::L | Key::F => {
                if key == Key::F {
                    if !self.p2_selected {
                        self.p2_selected = true;
                    } else if self.p2_color == 0 {
                        self.p2_color = self.p2_index_color;
                        self.ticks = 0;
                    }
                }
                if self.p2_selected && self.p2_color == 0 {
                    if self.p2_index_color > 1 && key == Key::J {
                        self.p2_index_color -= 1;
                    }
                    if self.p2_index_color < 6 && key == Key::L {
                        self.p2_index_color += 1;
                    }
                }
            }
            Key::Escape => {
                self.event_sender
                    .send(Event::ChangeScreen(ScreenType::Main))
                    .unwrap();
            }
            _ => (),
        }
    }

    fn on_release(&mut self, key: piston_window::prelude::Key) {
        let _ = key;
    }

    fn draw(
        &mut self,
        c: graphics::Context,
        g: &mut piston_window::prelude::G2d,
        device: &mut gfx_device_gl::Device,
    ) {
        clear([1.0; 4], g);
        if self.ticks / TICK_RESET % 2 == 0 {
            gui::draw_selector(c, g, self.p2_selected_index, false);
            gui::draw_selector(c, g, self.p1_selected_index, true);
        } else {
            gui::draw_selector(c, g, self.p1_selected_index, true);
            gui::draw_selector(c, g, self.p2_selected_index, false);
        }
        gui::draw_characters(c, g, &self.roster);

        if self.p1_selected {
            gui::draw_preview(
                c,
                g,
                device,
                &mut self.glyphs,
                &self.roster,
                self.p1_selected_index,
                true,
            );
            gui::draw_color_pick(
                c,
                g,
                device,
                &mut self.glyphs,
                self.p1_index_color,
                true,
                self.p1_color > 0,
            );
        }

        if self.p2_selected {
            gui::draw_preview(
                c,
                g,
                device,
                &mut self.glyphs,
                &self.roster,
                self.p2_selected_index,
                false,
            );
            gui::draw_color_pick(
                c,
                g,
                device,
                &mut self.glyphs,
                self.p2_index_color,
                false,
                self.p2_color > 0,
            );
        }
    }
}
