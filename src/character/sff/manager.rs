use super::decoder::{self, DecodeError};

use image::RgbaImage;
use piston_window::{self, G2dTextureContext, Texture, TextureSettings};
use std::rc::Rc;


pub struct SpriteManager {
    sprites: Vec<decoder::Sprite<'static>>,
}

impl SpriteManager {
    pub fn new(char_name: String) -> Result<Self, DecodeError> {
        let assets = std::env::current_dir().unwrap().join("src").join("assets");
        let sff_path = assets.join(char_name + ".sff");
        let sff = std::fs::read(sff_path).expect("Failed to read SFF file");

        let sff = match decoder::Decoder::decode(&sff) {
            Ok(decoded_data) => decoded_data,
            Err(error) => {
                return Err(error);
            }
        };

        let sprites = sff.sprites().map(|sprite| sprite.to_owned()).collect();

        Ok(Self {
            sprites
        })
    }

    pub fn get_texture(&self, context: &mut G2dTextureContext, group: u16, id: u16) -> Rc<Texture<gfx_device_gl::Resources>>{
        let sprite = self
            .sprites
            .iter()
            .find(|sprite| sprite.id().group == group && sprite.id().image == id)
            .unwrap();

        let mut pcx = pcx::Reader::new(sprite.raw_data()).unwrap();

        let width = pcx.width() as usize;
        let height = pcx.height() as usize;

        let palette = sprite
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

        let texture = Texture::from_image(
            context,
            img,
            &TextureSettings::new().filter(piston_window::Filter::Nearest),
        )
        .unwrap();

        Rc::new(texture)
    }
}
