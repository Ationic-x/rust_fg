use std::fs;

use gfx_device_gl::Resources;
use piston_window::{Flip, Glyphs, PistonWindow, Texture, TextureSettings};

use crate::{
    chars,
    error::{pop_up::show_error_popup, preload_error::PreloadError},
    player::character::sff::decoder::Sff,
};

/// Struct que contiene los recursos precargados para el juego, como fondos, fuentes y personajes.
pub struct Preloads {
    /// Lista de fondo precargados
    backgrounds: Vec<Texture<Resources>>,
    /// Lista de fuentes precargadas
    fonts: Vec<Glyphs>,
    /// Lista de archivos SFF de personajes precargados
    roster: Vec<Sff>,
}

impl Preloads {
    /// Crea una nueva instancia de `Preloads` cargando los recursos necesarios.
    ///
    /// # Argumentos
    ///
    /// * `window` - Referencia mutable a la ventana de Piston donde se cargan los recursos.
    ///
    /// # Retorna
    ///
    /// Una nueva instancia de `Preloads` con los recursos cargados, o un error si no se pudieron cargar.    
    pub fn new(window: &mut PistonWindow) -> Result<Self, PreloadError> {
        let mut fonts = Vec::new();
        let mut backgrounds = Vec::new();

        let font = "assets\\fonts\\OpenSans-ExtraBold.ttf";
        match Glyphs::new(
            font,
            window.create_texture_context(),
            TextureSettings::new(),
        ) {
            Ok(glyphs) => {
                fonts.push(glyphs);
            }
            Err(_) => return Err(PreloadError::FontNotFound(font.to_string())),
        }

        let font = "assets\\fonts\\OpenSans-Regular.ttf";
        match Glyphs::new(
            font,
            window.create_texture_context(),
            TextureSettings::new(),
        ) {
            Ok(glyphs) => {
                fonts.push(glyphs);
            }
            Err(_) => return Err(PreloadError::FontNotFound(font.to_string())),
        }

        let background = "assets\\images\\background_1.png";
        match Texture::from_path(
            &mut window.create_texture_context(),
            background,
            Flip::None,
            &TextureSettings::new(),
        ) {
            Ok(texture) => backgrounds.push(texture),
            Err(_) => return Err(PreloadError::BackgroundNotFound(background.to_string())),
        };

        let background = "assets\\images\\background_3.png";
        match Texture::from_path(
            &mut window.create_texture_context(),
            background,
            Flip::None,
            &TextureSettings::new(),
        ) {
            Ok(texture) => backgrounds.push(texture),
            Err(_) => return Err(PreloadError::BackgroundNotFound(background.to_string())),
        };

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
                                std::process::exit(1);
                            }
                        }
                    }
                }
            }
        }

        Ok(Self {
            backgrounds,
            fonts,
            roster,
        })
    }

    /// Obtiene una referencia mutable a los fondos precargados.
    ///
    /// # Retorna
    ///
    /// Una referencia mutable a los fondos precargados.
    pub fn get_mut_ref_background(&self) -> &Vec<Texture<Resources>> {
        &self.backgrounds
    }

    /// Obtiene una referencia a los personajes precargados.
    ///
    /// # Retorna
    ///
    /// Una referencia a los personajes precargados.
    pub fn get_ref_roster(&self) -> &Vec<Sff> {
        &self.roster
    }

    /// Obtiene una referencia mutable a las fuentes precargadas.
    ///
    /// # Retorna
    ///
    /// Una referencia mutable a las fuentes precargadas.
    pub fn get_mut_ref_fonts(&mut self) -> &mut Vec<Glyphs> {
        &mut self.fonts
    }
}
