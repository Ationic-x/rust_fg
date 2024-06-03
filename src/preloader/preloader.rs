use std::fs;

use gfx_device_gl::Resources;
use piston_window::{Flip, Glyphs, PistonWindow, Texture, TextureSettings};

use crate::{
    chars,
    player::character::sff::{decoder::Sff, error::show_error_popup},
};

pub struct Preloads {
    backgrounds: Vec<Texture<Resources>>,
    fonts: Vec<Glyphs>,
    roster: Vec<Sff>,
}

impl Preloads {
    pub fn new(window: &mut PistonWindow) -> Self {
        let mut fonts = Vec::new();
        fonts.push(
            Glyphs::new(
                "assets\\fonts\\OpenSans-ExtraBold.ttf",
                window.create_texture_context(),
                TextureSettings::new(),
            )
            .unwrap(),
        );
        fonts.push(
            Glyphs::new(
                "assets\\fonts\\OpenSans-Regular.ttf",
                window.create_texture_context(),
                TextureSettings::new(),
            )
            .unwrap(),
        );

        let mut backgrounds = Vec::new();
        backgrounds.push(
            Texture::from_path(
                &mut window.create_texture_context(),
                "assets\\images\\background_1.png",
                Flip::None,
                &TextureSettings::new(),
            )
            .unwrap(),
        );
        backgrounds.push(
            Texture::from_path(
                &mut window.create_texture_context(),
                "assets\\images\\background_3.png",
                Flip::None,
                &TextureSettings::new(),
            )
            .unwrap(),
        );

        let mut roster = Vec::new();

        for entry in fs::read_dir("src\\chars\\").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    if let Some(char_name) = name.to_str() {
                        let char = chars::get_char(char_name).unwrap();
                        let context = window.create_texture_context();
                        match Sff::preload_sff(
                            char_name,
                            char.get_sff_name().to_string() + ".sff",
                            true,
                            context,
                        ) {
                            Ok(sff) => roster.push(sff),
                            Err(err) => {
                                show_error_popup(&err);
                                std::process::exit(0);
                            }
                        }
                    }
                }
            }
        }

        Self {
            backgrounds,
            fonts,
            roster,
        }
    }

    pub fn get_mut_ref_background(&self) -> &Vec<Texture<Resources>> {
        &self.backgrounds
    }

    pub fn get_ref_roster(&self) -> &Vec<Sff> {
        &self.roster
    }

    pub fn get_mut_ref_fonts(&mut self) -> &mut Vec<Glyphs> {
        &mut self.fonts
    }
}
