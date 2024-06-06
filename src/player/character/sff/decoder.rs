const MAX_PAL_NO: usize = 10;

use std::{
    collections::HashMap,
    io::{self, Cursor, Read, Seek, SeekFrom},
    path::PathBuf,
    rc::Rc,
};

use byteorder::{LittleEndian, ReadBytesExt};
use flate2::read::ZlibDecoder;
use gfx_device_gl::Resources;
use image::RgbaImage;
use piston_window::{G2dTextureContext, TextureSettings};

use crate::error::sff_error::SffError;

/// Estructura que representa un identificador de sprite.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SpriteId {
    pub group: u16,
    pub image: u16,
}

/// Estructura que representa una versión con cuatro componentes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version(u8, u8, u8, u8);

/// Estructura que representa la cabecera de un archivo SFF.
#[derive(Debug, Clone, Copy)]
struct SffHeader {
    ver0: u8,
    ver1: u8,
    ver2: u8,
    ver3: u8,
    first_sprite_header_offset: u32,
    first_palette_header_offset: u32,
    number_of_sprites: u32,
    number_of_palettes: u32,
    lofs: u32,
    tofs: u32,
}

/// Representa un decodificador para archivos SFF, utilizado para manejar datos comprimidos
/// como sprites, paletas y grupos.
///
/// La estructura `Sff` contiene la cabecera del archivo SFF, una lista de sprites, una lista de paletas,
/// el nombre del archivo y el contexto gráfico utilizado para renderizar los sprites.
pub struct Sff {
    header: SffHeader,
    pub sprites: HashMap<[i16; 2], Sprite>,
    pub pal_list: PaletteList,
    filename: String,
    name: String,
    context: G2dTextureContext,
}

/// Estructura que representa un color con componentes RGBA.
#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

/// Estructura que representa una lista de paletas.
#[derive(Debug, Clone)]
pub struct PaletteList {
    palettes: Vec<Vec<Color>>,
    palette_map: Vec<i32>,
    pal_table: HashMap<[i16; 2], i32>,
}

/// Representa un sprite en un archivo SFF.
///
/// Un sprite contiene la información necesaria para su representación gráfica,
/// incluyendo la paleta de colores, textura, datos sin procesar y metadatos
/// relacionados con su grupo, número y dimensiones.
#[derive(Debug, Clone)]
pub struct Sprite {
    pub pal: Vec<Color>,
    pub tex: Option<Rc<piston_window::Texture<Resources>>>,
    pub raw: Vec<u8>,
    pub group: i16,
    pub number: i16,
    pub size: [u16; 2],
    pub offset: [i16; 2],
    pub pal_idx: i32,
    pub rle: i32,
    pub col_depth: u8,
    pub pal_temp: Vec<Color>,
}

impl From<Version> for (u8, u8, u8, u8) {
    /// Convierte un `Version` a una tupla de cuatro elementos (u8, u8, u8, u8).
    ///
    /// # Argumentos
    ///
    /// * `v` - La versión a convertir.
    ///
    /// # Retorna
    ///
    /// Una tupla de cuatro elementos que representa la versión.
    fn from(v: Version) -> Self {
        (v.0, v.1, v.2, v.3)
    }
}

impl From<SpriteId> for (u16, u16) {
    /// Convierte un `SpriteId` a una tupla de dos elementos (u16, u16).
    ///
    /// # Argumentos
    ///
    /// * `sprite_id` - El identificador de sprite a convertir.
    ///
    /// # Retorna
    ///
    /// Una tupla de dos elementos que representa el identificador de sprite.
    fn from(SpriteId { group, image }: SpriteId) -> Self {
        (group, image)
    }
}

impl From<(u16, u16)> for SpriteId {
    /// Convierte una tupla de dos elementos (u16, u16) a un `SpriteId`.
    ///
    /// # Argumentos
    ///
    /// * `tuple` - La tupla a convertir.
    ///
    /// # Retorna
    ///
    /// Un `SpriteId` que representa la tupla.
    fn from((group, image): (u16, u16)) -> Self {
        SpriteId { group, image }
    }
}

impl Color {
    /// Crea un nuevo `Color` con valores por defecto (blanco opaco).
    ///
    /// # Retorna
    ///
    /// Un `Color` con componentes RGBA todos establecidos en 255.
    fn new() -> Self {
        Self {
            red: 255,
            green: 255,
            blue: 255,
            alpha: 255,
        }
    }

    /// Obtiene una representación vectorial del color.
    ///
    /// # Retorna
    ///
    /// Un `Vec<u8>` que contiene los componentes RGBA del color.
    pub fn get_vec(&self) -> Vec<u8> {
        vec![self.red, self.green, self.blue, self.alpha]
    }
}

impl Sprite {
    /// Crea un nuevo sprite vacío.
    ///
    /// # Retorna
    ///
    /// Un nuevo sprite con valores por defecto.
    pub fn new() -> Self {
        Self {
            pal: Vec::new(),
            tex: None,
            raw: Vec::new(),
            group: 0,
            number: 0,
            size: [0, 0],
            offset: [0, 0],
            pal_idx: -1,
            rle: 0,
            col_depth: 0,
            pal_temp: Vec::new(),
        }
    }

    /// Comparte y copia los datos de otro sprite.
    ///
    /// # Argumentos
    ///
    /// * `src` - El sprite del cual se copiarán los datos.
    fn share_copy(&mut self, src: Sprite) {
        self.pal = src.pal;
        self.tex = src.tex;
        self.size = src.size;
        if self.pal_idx < 0 {
            self.pal_idx = src.pal_idx;
        }
        self.col_depth = src.col_depth;
        self.raw = src.raw;
    }

    /// Lee la cabecera de un sprite versión 1 desde un flujo de bytes (SFF en bytes).
    ///
    /// # Argumentos
    ///
    /// * `bytes` - El flujo de bytes desde el cual leer.
    /// * `ofs` - Desplazamiento de los datos del sprite.
    /// * `size` - Tamaño de los datos del sprite.
    /// * `link` - Enlace del sprite.
    ///
    /// # Retorna
    ///
    /// El sprite modificado.
    fn read_header_v1(
        &mut self,
        bytes: &mut Cursor<&Vec<u8>>,
        ofs: &mut u32,
        size: &mut u32,
        link: &mut u16,
    ) -> Result<&mut Self, SffError> {
        *ofs = bytes.read_u32::<LittleEndian>()?;
        *size = bytes.read_u32::<LittleEndian>()?;
        for ele in &mut self.offset {
            *ele = bytes.read_i16::<LittleEndian>()?;
        }
        self.group = bytes.read_i16::<LittleEndian>()?;
        self.number = bytes.read_i16::<LittleEndian>()?;
        *link = bytes.read_u16::<LittleEndian>()?;
        Ok(self)
    }

    /// Lee los datos de un sprite versión 1 desde un flujo de bytes (SFF en bytes).
    ///
    /// # Argumentos
    ///
    /// * `bytes` - El flujo de bytes desde el cual leer.
    /// * `offset` - Desplazamiento actual en el flujo de bytes.
    /// * `datasize` - Tamaño de los datos del sprite.
    /// * `next_subheader` - Desplazamiento del siguiente subencabezado.
    /// * `prev` - Sprite anterior.
    /// * `pl` - Lista de paletas.
    /// * `c00` - Indicador para el manejo de paletas.
    ///
    /// # Retorna
    ///
    /// El sprite modificado.
    fn read_v1(
        &mut self,
        mut bytes: &mut Cursor<&Vec<u8>>,
        mut offset: &mut i64,
        mut datasize: u32,
        next_subheader: u32,
        prev: &Option<Sprite>,
        pl: &mut PaletteList,
        c00: bool,
    ) -> Result<&mut Self, SffError> {
        if next_subheader as i64 > *offset {
            datasize = next_subheader - *offset as u32;
        }
        let ps = bytes.read_u8()?;
        let palette_same = ps != 0 && !prev.is_none();

        self.read_pcx_header(&mut bytes, &mut offset)?;

        bytes.seek(SeekFrom::Start((*offset + 128) as u64))?;

        let pal_size: u32;

        if c00 || palette_same {
            pal_size = 0;
        } else {
            pal_size = 768;
        }
        if datasize < 128 + pal_size {
            datasize = 128 + pal_size;
        }

        let mut px = vec![0u8; (datasize - (128 + pal_size)) as usize];
        for ele in &mut px {
            *ele = bytes.read_u8()?;
        }

        if palette_same {
            if let Some(_prev) = prev {
                self.pal_idx = _prev.pal_idx;
            }

            if self.pal_idx < 0 {
                self.pal_idx = pl.new_pal().0 as i32;
            }
        } else {
            let result = pl.new_pal();
            self.pal_idx = result.0 as i32;
            if c00 {
                bytes.seek(SeekFrom::Start((*offset + datasize as i64 - 768) as u64))?;
            }
            for _ in 0..result.1.len() {
                let mut color = Color::new();

                color.red = bytes.read_u8()?;
                color.green = bytes.read_u8()?;
                color.blue = bytes.read_u8()?;
                color.alpha = bytes.read_u8()?;
            }
        }
        px = self.rle_pcx_decode(px);
        self.set_pxl(px);
        Ok(self)
    }

    /// Establece los píxeles del sprite.
    ///
    /// # Argumentos
    ///
    /// * `px` - Vector de bytes que representa los píxeles.
    fn set_pxl(&mut self, px: Vec<u8>) {
        if px.len() == 0 {
            return;
        }
        if (px.len() as i64) != (self.size[0] as i64) * (self.size[1] as i64) {
            return;
        }
        self.raw = px;
    }

    /// Decodifica datos RLE (Run-Length Encoding) en formato PCX.
    ///
    /// # Argumentos
    ///
    /// * `rle` - Datos RLE a decodificar.
    ///
    /// # Retorna
    ///
    /// Vector de bytes decodificados.
    fn rle_pcx_decode(&mut self, rle: Vec<u8>) -> Vec<u8> {
        if rle.len() == 0 || self.rle <= 0 {
            return rle;
        }
        let mut p = vec![0u8; (self.size[0] * self.size[1]) as usize];
        let (mut i, mut j, mut k, w) = (0, 0, 0, self.size[0] as usize);
        while j < p.len() {
            let (mut n, mut d) = (1, rle[i]);
            if i < rle.len() - 1 {
                i += 1;
            }
            if d >= 0xc0 {
                n = (d & 0x3f) as usize;
                d = rle[i];
                if i < rle.len() - 1 {
                    i += 1;
                }
            }
            while n > 0 {
                if k < w && j < p.len() {
                    p[j] = d;
                    j += 1;
                }
                k += 1;
                if k == self.rle as usize {
                    k = 0;
                    n = 1;
                }
                n -= 1;
            }
        }
        self.rle = 0;
        p
    }

    /// Lee la cabecera de un archivo PCX.
    ///
    /// # Argumentos
    ///
    /// * `bytes` - El flujo de bytes desde el cual leer.
    /// * `offset` - Desplazamiento actual en el flujo de bytes.
    ///
    /// # Retorna
    ///
    /// Resultado vacío en caso de éxito o un error en caso contrario.
    fn read_pcx_header(
        &mut self,
        bytes: &mut Cursor<&Vec<u8>>,
        offset: &mut i64,
    ) -> Result<(), io::Error> {
        bytes.seek(SeekFrom::Start(*offset as u64))?;

        bytes.read_u16::<LittleEndian>()?;

        let enconding = bytes.read_u8()?;
        let bpp = bytes.read_u8()?;

        if bpp != 8 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "bpp is not equal to 8",
            ));
        }

        let mut rect = [0u16; 4];
        for ele in &mut rect {
            *ele = bytes.read_u16::<LittleEndian>()?;
        }

        bytes.seek(SeekFrom::Start((*offset + 66) as u64))?;

        let bpl = bytes.read_u16::<LittleEndian>()?;

        self.size[0] = rect[2] - rect[0] + 1;
        self.size[1] = rect[3] - rect[1] + 1;

        if enconding == 1 {
            self.rle = bpl as i32;
        } else {
            self.rle = 0;
        }
        Ok(())
    }

    /// Lee la cabecera de un sprite versión 2 desde un flujo de bytes.
    ///
    /// # Argumentos
    ///
    /// * `bytes` - El flujo de bytes desde el cual leer.
    /// * `ofs` - Desplazamiento de los datos del sprite.
    /// * `size` - Tamaño de los datos del sprite.
    /// * `lofs` - Desplazamiento del enlace.
    /// * `tofs` - Desplazamiento de la textura.
    /// * `link` - Enlace del sprite.
    /// * `pl` - Lista de paletas.
    ///
    /// # Retorna
    ///
    /// El sprite modificado.
    fn read_header_v2(
        &mut self,
        bytes: &mut Cursor<&Vec<u8>>,
        ofs: &mut u32,
        size: &mut u32,
        lofs: u32,
        tofs: u32,
        link: &mut u16,
        pl: &PaletteList,
    ) -> Result<&mut Self, SffError> {
        self.group = bytes.read_i16::<LittleEndian>()?;
        self.number = bytes.read_i16::<LittleEndian>()?;
        for ele in &mut self.size {
            *ele = bytes.read_u16::<LittleEndian>()?;
        }
        for ele in &mut self.offset {
            *ele = bytes.read_i16::<LittleEndian>()?;
        }
        *link = bytes.read_u16::<LittleEndian>()?;

        let format = bytes.read_u8()?;

        self.rle = -(format as i32);

        self.col_depth = bytes.read_u8()?;
        *ofs = bytes.read_u32::<LittleEndian>()?;
        *size = bytes.read_u32::<LittleEndian>()?;

        let tmp = bytes.read_u16::<LittleEndian>()?;
        self.pal_idx = tmp as i32;
        self.pal = pl.get(self.pal_idx as usize).clone();
        let tmp = bytes.read_u16::<LittleEndian>()?;

        if tmp & 1 == 0 {
            *ofs += lofs;
        } else {
            *ofs += tofs;
        }
        Ok(self)
    }

    /// Lee los datos de un sprite versión 2 desde un flujo de bytes.
    ///
    /// # Argumentos
    ///
    /// * `bytes` - El flujo de bytes desde el cual leer.
    /// * `offset` - Desplazamiento actual en el flujo de bytes.
    /// * `datasize` - Tamaño de los datos del sprite.
    /// * `context` - Contexto de textura para la creación de texturas.
    ///
    /// # Retorna
    ///
    /// El sprite modificado.
    fn read_v2(
        &mut self,
        bytes: &mut Cursor<&Vec<u8>>,
        offset: i64,
        mut datasize: u32,
        context: &mut G2dTextureContext,
    ) -> Result<&mut Self, SffError> {
        let is_raw = false;
        let mut px: Vec<u8>;

        if self.rle > 0 {
            return Ok(self);
        } else if self.rle == 0 {
            bytes.seek(SeekFrom::Start(offset as u64))?;
            px = vec![0u8; datasize as usize];
            for ele in &mut px {
                *ele = bytes.read_u8()?;
            }
            match self.col_depth {
                8 => {}
                24 | 32 => {
                    //is_raw = true;
                    self.set_raw(px);
                }
                _ => {
                    return Err(SffError::UnknownColorDepth(self.col_depth));
                }
            }
        } else {
            bytes.seek(SeekFrom::Start((offset + 4) as u64))?;
            let format = -self.rle;

            if 2 <= format && format <= 4 {
                if datasize < 4 {
                    datasize = 4;
                }
                px = vec![0u8; (datasize - 4) as usize];
                for ele in &mut px {
                    *ele = bytes.read_u8()?;
                }

                match format {
                    2 => {
                        px = self.rle8_decode(px);
                    }
                    3 => {
                        px = self.rle5_decode(px);
                    }
                    4 => {
                        px = self.lz5_decode(px);
                    }
                    _ => {}
                }

                if !is_raw {
                    self.set_pxl(px)
                }
            }

            match format {
                10 => {
                    let iend_position;
                    let mut idat_position = 0;
                    let mut buffer = [0; 4];

                    loop {
                        match bytes.read_exact(&mut buffer) {
                            Ok(_) => {
                                if buffer == [73, 68, 65, 84] {
                                    idat_position = bytes.seek(SeekFrom::Current(-3))?;
                                } else {
                                    bytes.seek(SeekFrom::Current(-3))?;
                                }
                                if buffer == [73, 69, 78, 68] {
                                    iend_position = bytes.seek(SeekFrom::Current(-4))?;
                                    break;
                                }
                            }
                            Err(err) => return Err(SffError::InvalidData(err)),
                        }
                    }

                    let mut cloned_bytes = vec![0u8; (iend_position - idat_position) as usize];
                    bytes.seek(SeekFrom::Start(idat_position + 3 as u64))?;
                    bytes.read_exact(&mut cloned_bytes)?;

                    let cursor = Cursor::new(cloned_bytes);
                    let mut decoder = ZlibDecoder::new(cursor);

                    let mut new_vector = vec![0u8; self.size[0] as usize * self.size[1] as usize];
                    for (i, pixel) in new_vector.iter_mut().enumerate() {
                        if i % self.size[0] as usize == 0 {
                            decoder.read_u16::<LittleEndian>()?;
                        } else {
                            *pixel = decoder.read_u8()?;
                        }
                    }

                    self.set_pxl(new_vector);
                }
                _ => {}
            }
            //     11 | 12 => {
            //         is_raw = true;
            //     }
            //     _ => {
            //         return Err(SffError::InvalidSignature);
            //     }
            // }
        }
        if !self.raw.is_empty() {
            self.set_texture(context);
        }
        Ok(self)
    }

    /// Decodifica datos RLE8 (Run-Length Encoding 8) en formato PCX.
    ///
    /// # Argumentos
    ///
    /// * `rle` - Datos RLE a decodificar.
    ///
    /// # Retorna
    ///
    /// Vector de bytes decodificados.
    fn rle8_decode(&mut self, rle: Vec<u8>) -> Vec<u8> {
        if rle.is_empty() {
            return rle;
        }
        let mut p = vec![0u8; (self.size[0] * self.size[1]) as usize];
        let mut i = 0;
        let mut j = 0;
        while j < p.len() {
            let mut n = 1;
            let mut d = rle[i];
            if i < rle.len() - 1 && (d & 0xc0) == 0x40 {
                n = d as usize & 0x3f;
                i += 1;
                d = rle[i];
            }
            i += 1;
            for _ in 0..n {
                if j < p.len() {
                    p[j] = d;
                    j += 1;
                }
            }
        }
        p
    }

    /// Decodifica datos RLE5 (Run-Length Encoding 5) en formato PCX.
    ///
    /// # Argumentos
    ///
    /// * `rle` - Datos RLE a decodificar.
    ///
    /// # Retorna
    ///
    /// Vector de bytes decodificados.
    fn rle5_decode(&self, rle: Vec<u8>) -> Vec<u8> {
        if rle.is_empty() {
            return rle;
        }
        let mut p = vec![0u8; (self.size[0] * self.size[1]) as usize];
        let mut i = 0;
        let mut j = 0;
        while j < p.len() {
            let mut rl = rle[i] as usize;
            if i < rle.len() - 1 {
                i += 1;
            }
            let dl = (rle[i] & 0x7f) as usize;
            let mut c = 0;
            if rle[i] >> 7 != 0 {
                if i < rle.len() - 1 {
                    i += 1;
                }
                c = rle[i] as usize;
            }
            if i < rle.len() - 1 {
                i += 1;
            }
            for _ in 0..dl {
                for _ in 0..rl {
                    if j < p.len() {
                        p[j] = c as u8;
                        j += 1;
                    }
                }
                if i < rle.len() - 1 {
                    i += 1;
                }
                if i >= rle.len() {
                    break;
                }
                c = rle[i] as usize & 0x1f;
                rl = (rle[i] >> 5) as usize;
            }
        }
        p
    }

    /// Decodifica datos LZ5 (Lempel-Ziv 5) en formato PCX.
    ///
    /// # Argumentos
    ///
    /// * `rle` - Datos RLE a decodificar.
    ///
    /// # Retorna
    ///
    /// Vector de bytes decodificados.
    fn lz5_decode(&self, rle: Vec<u8>) -> Vec<u8> {
        if rle.is_empty() {
            return rle;
        }
        let mut p = vec![0u8; (self.size[0] * self.size[1]) as usize];
        let (mut i, mut j) = (0, 0);
        let mut n;
        let (mut ct, mut cts, mut rb, mut rbc) = (rle[i], 0u32, 0u8, 0u32);
        if i < rle.len() - 1 {
            i += 1;
        }
        while j < p.len() {
            let mut d = rle[i] as i32;
            if i < rle.len() - 1 {
                i += 1;
            }
            if ct & (1 << cts) as u8 != 0 {
                if d & 0x3f == 0 {
                    d = (d << 2 | rle[i] as i32) + 1;
                    if i < rle.len() - 1 {
                        i += 1;
                    }
                    n = rle[i] as i32 + 2;
                    if i < rle.len() - 1 {
                        i += 1;
                    }
                } else {
                    rb |= ((d & 0xc0) >> rbc) as u8;
                    rbc += 2;
                    n = (d & 0x3f) as i32;
                    if rbc < 8 {
                        d = rle[i] as i32 + 1;
                        if i < rle.len() - 1 {
                            i += 1;
                        }
                    } else {
                        d = rb as i32 + 1;
                        rb = 0;
                        rbc = 0;
                    }
                }
                while n >= 0 {
                    if j < p.len() {
                        if j > d as usize {
                            p[j] = p[j - d as usize];
                        }
                        j += 1;
                    }
                    n -= 1;
                }
            } else {
                if d & 0xe0 == 0 {
                    n = rle[i] as i32 + 8;
                    if i < rle.len() - 1 {
                        i += 1;
                    }
                } else {
                    n = d >> 5;
                    d &= 0x1f;
                }
                while n > 0 {
                    if j < p.len() {
                        p[j] = d as u8;
                    }
                    j += 1;
                    n -= 1;
                }
            }
            cts += 1;
            if cts >= 8 {
                ct = rle[i];
                cts = 0;
                if i < rle.len() - 1 {
                    i += 1;
                }
            }
        }
        p
    }

    /// Establece los datos crudos del sprite.
    ///
    /// # Argumentos
    ///
    /// * `px` - Vector de datos crudos.
    fn set_raw(&mut self, px: Vec<u8>) {
        self.raw = px;
    }

    /// Establece la paleta del sprite.
    ///
    /// # Argumentos
    ///
    /// * `palette` - Paleta de colores.
    /// * `context` - Contexto de textura.
    fn set_palette(&mut self, palette: &Vec<Color>, context: &mut G2dTextureContext) {
        self.pal_temp = palette.clone();
        self.set_texture(context);
    }

    /// Establece la textura del sprite.
    ///
    /// # Argumentos
    ///
    /// * `context` - Contexto de textura.
    fn set_texture(&mut self, context: &mut G2dTextureContext) {
        let mut rgba: Vec<u8> = Vec::new();
        let palette;

        if !self.pal_temp.is_empty() {
            palette = &self.pal_temp;
        } else {
            palette = &self.pal;
        }

        for &index in &self.raw {
            if let Some(color) = palette.get(index as usize) {
                rgba.extend_from_slice(&color.get_vec());
            }
        }

        let img = &RgbaImage::from_raw(self.size[0] as u32, self.size[1] as u32, rgba).unwrap();

        let texture = piston_window::Texture::from_image(
            context,
            img,
            &TextureSettings::new().filter(piston_window::Filter::Nearest),
        )
        .unwrap();

        self.tex = Some(Rc::new(texture));
    }
}

impl PaletteList {
    /// Crea una nueva `PaletteList` vacía.
    ///
    /// # Retorna
    ///
    /// Una `PaletteList` con listas y mapas vacíos.
    fn new() -> Self {
        Self {
            palettes: Vec::new(),
            palette_map: Vec::new(),
            pal_table: HashMap::new(),
        }
    }

    /// Establece la fuente de una paleta en una posición dada.
    ///
    /// # Argumentos
    ///
    /// * `i` - El índice en el mapa de paletas.
    /// * `p` - Una referencia a un `Vec<Color>` que representa la paleta.
    fn set_source(&mut self, i: usize, p: &Vec<Color>) {
        if i < self.palette_map.len() {
            self.palette_map[i as usize] = i as i32;
        } else {
            while i > self.palette_map.len() {
                self.palette_map.push(self.palette_map.len() as i32);
            }
            self.palette_map.push(i as i32);
        }

        if i < self.palettes.len() {
            self.palettes[i as usize] = p.clone();
        } else {
            while i > self.palettes.len() {
                self.palettes.push(vec![]);
            }
            self.palettes.push(p.clone());
        }
    }

    /// Crea una nueva paleta vacía y la agrega a la lista.
    ///
    /// # Retorna
    ///
    /// Una tupla con el índice de la nueva paleta y la paleta vacía.
    fn new_pal(&mut self) -> (usize, Vec<Color>) {
        let i = self.palettes.len();
        let p = Vec::new();
        self.set_source(i, &p);
        (i, p)
    }

    /// Obtiene una referencia a la paleta en una posición dada.
    ///
    /// # Argumentos
    ///
    /// * `i` - El índice de la paleta en el mapa.
    ///
    /// # Retorna
    ///
    /// Una referencia a un `Vec<Color>` que representa la paleta.
    fn get(&self, i: usize) -> &Vec<Color> {
        return &self.palettes[self.palette_map[i] as usize];
    }
}

impl Sff {
    /// Crea una nueva instancia de `Sff`.
    ///
    /// Inicializa una nueva instancia de `Sff` con un contexto gráfico proporcionado. La lista de paletas se inicializa
    /// con valores predeterminados.
    ///
    /// # Argumentos
    ///
    /// * `context` - El contexto gráfico utilizado para renderizar los sprites.
    ///
    /// # Retorna
    ///
    /// Una nueva instancia de `Sff` con valor por defectos y contexto enviado.
    pub fn new(context: G2dTextureContext) -> Self {
        let mut pal_list = PaletteList::new();
        for i in 1..MAX_PAL_NO as i16 {
            let (index_map, _) = pal_list.new_pal();
            pal_list.pal_table.insert([1, i], index_map as i32);
        }
        Self {
            header: SffHeader::new(),
            sprites: HashMap::new(),
            pal_list,
            filename: String::new(),
            context,
            name: String::new(),
        }
    }

    /// Precarga un archivo SFF para un personaje específico.
    ///
    /// Esta función carga y analiza el archivo SFF, configurando las paletas y los sprites. El archivo SFF debe estar ubicado
    /// en el directorio 'src/chars/<char_name>'.
    ///
    /// # Argumentos
    ///
    /// * `char_name` - El nombre del personaje asociado con el archivo SFF.
    /// * `filename` - El nombre del archivo SFF.
    /// * `char` - Un indicador booleano que especifica si el archivo es de un personaje.
    /// * `context` - El contexto gráfico utilizado para renderizar los sprites.
    ///
    /// # Retorna
    ///
    /// Una instancia de `Sff` si se carga correctamente, o un `SffError` en caso de error.
    pub fn preload_sff(
        char_name: &str,
        filename: String,
        char: bool,
        context: G2dTextureContext,
    ) -> Result<Sff, SffError> {
        let mut sff = Sff::new(context);
        sff.filename = filename.clone();
        sff.name = char_name.to_string();

        let assets = std::env::current_dir()
            .unwrap()
            .join("src")
            .join("chars")
            .join(char_name);
        let sff_path = assets.join(filename);

        if !sff_path.exists() {
            return Err(SffError::NotFound(sff_path));
        }

        match sff.header.read(sff_path.clone()) {
            Ok(_) => println!("Header was read"),
            Err(err) => return Err(err),
        };

        if sff.header.ver0 != 1 {
            match sff.configure_pals_v2(sff_path.clone()) {
                Ok(_) => println!("Configured and added all the palettes"),
                Err(err) => return Err(err),
            };
        }

        match sff.configure_sprite(sff_path.clone(), char, true) {
            Ok(_) => println!("Readed and created all the sprites"),
            Err(err) => return Err(err),
        };
        Ok(sff)
    }

    /// Carga un archivo SFF para un personaje específico.
    ///
    /// Esta función carga y analiza el archivo SFF, configurando las paletas y los sprites. El archivo SFF debe estar ubicado
    /// en el directorio 'src/chars/<char_name>'.
    ///
    /// # Argumentos
    ///
    /// * `char_name` - El nombre del personaje asociado con el archivo SFF.
    /// * `filename` - El nombre del archivo SFF.
    /// * `char` - Un indicador booleano que especifica si el archivo es de un personaje.
    /// * `context` - El contexto gráfico utilizado para renderizar los sprites.
    ///
    /// # Retorna
    ///
    /// Una instancia de `Sff` si se carga correctamente, o un `SffError` en caso de error.
    pub fn load_sff(
        char_name: &str,
        filename: String,
        char: bool,
        context: G2dTextureContext,
    ) -> Result<Sff, SffError> {
        let mut sff = Sff::new(context);
        sff.filename = filename.clone();
        sff.name = char_name.to_string();

        let assets = std::env::current_dir()
            .unwrap()
            .join("src")
            .join("chars")
            .join(char_name);
        let sff_path = assets.join(filename);

        if !sff_path.exists() {
            return Err(SffError::NotFound(sff_path));
        }

        match sff.header.read(sff_path.clone()) {
            Ok(_) => println!("Header was read"),
            Err(err) => return Err(err),
        };

        if sff.header.ver0 != 1 {
            match sff.configure_pals_v2(sff_path.clone()) {
                Ok(_) => println!("Configured and added all the palettes"),
                Err(err) => return Err(err),
            };
        }

        match sff.configure_sprite(sff_path.clone(), char, false) {
            Ok(_) => println!("Readed and created all the sprites"),
            Err(err) => return Err(err),
        };
        Ok(sff)
    }

    /// Configura los sprites desde el archivo SFF.
    ///
    /// Esta función lee y configura los sprites desde el archivo SFF, actualizando la lista de sprites de la instancia.
    ///
    /// # Argumentos
    ///
    /// * `sff_path` - La ruta al archivo SFF.
    /// * `char` - Un indicador booleano que especifica si el archivo es de un personaje.
    /// * `preload` - Un indicador booleano que especifica si los sprites deben precargarse.
    ///
    /// # Retorna
    ///
    /// Un `Result` vacío indicando éxito o un `SffError` en caso de error.
    fn configure_sprite(
        &mut self,
        sff_path: PathBuf,
        char: bool,
        preload: bool,
    ) -> Result<(), SffError> {
        let mut sprite_list: Vec<Sprite> =
            vec![Sprite::new(); self.header.number_of_sprites as usize];
        let mut prev: Option<Sprite> = None;
        let mut shofs = self.header.first_sprite_header_offset as u64;
        let number_sprites;
        if preload {
            number_sprites = 2;
        } else {
            number_sprites = sprite_list.len()
        }

        let data = std::fs::read(sff_path)?;

        for i in 0..number_sprites {
            let mut bytes = Cursor::new(&data);
            bytes.seek(SeekFrom::Start(shofs))?;
            let mut xofs: u32 = 0;
            let mut size: u32 = 0;
            let mut index_of_previous: u16 = 0;
            match self.header.ver0 {
                1 => {
                    sprite_list[i].read_header_v1(
                        &mut bytes,
                        &mut xofs,
                        &mut size,
                        &mut index_of_previous,
                    )?;
                }
                2 => {
                    sprite_list[i].read_header_v2(
                        &mut bytes,
                        &mut xofs,
                        &mut size,
                        self.header.lofs,
                        self.header.tofs,
                        &mut index_of_previous,
                        &self.pal_list,
                    )?;
                }
                _ => {
                    return Err(SffError::UnsupportedHeaderVersion(self.header.ver0));
                }
            }
            if size == 0 {
                if index_of_previous < i as u16 {
                    let src = sprite_list[index_of_previous as usize].clone();
                    sprite_list[i].share_copy(src);
                } else {
                    sprite_list[i].pal_idx = 0;
                }
            } else {
                let mut bytes = Cursor::new(&data);
                match self.header.ver0 {
                    1 => {
                        let c00 = char
                            && (prev.is_none()
                                || sprite_list[i].group == 0 && sprite_list[i].number == 0);
                        sprite_list[i].read_v1(
                            &mut bytes,
                            &mut ((shofs + 32) as i64),
                            size,
                            xofs,
                            &prev,
                            &mut self.pal_list,
                            c00,
                        )?;
                    }
                    2 => {
                        sprite_list[i].read_v2(&mut bytes, xofs as i64, size, &mut self.context)?;
                    }
                    _ => {
                        return Err(SffError::UnsupportedHeaderVersion(self.header.ver0));
                    }
                }
                if let Some(sprite) = sprite_list.get(i) {
                    prev = Some(sprite.clone());
                } else {
                    prev = None;
                }
            }
            if self
                .sprites
                .get_mut(&[sprite_list[i].group, sprite_list[i].number])
                .is_none()
            {
                self.sprites.insert(
                    [sprite_list[i].group, sprite_list[i].number],
                    sprite_list[i].clone(),
                );
            }

            if self.header.ver0 == 1 {
                shofs = xofs as u64;
            } else {
                shofs += 28;
            }
        }
        Ok(())
    }

    /// Obtiene el nombre del personaje asociado con el archivo SFF.
    ///
    /// # Retorna
    ///
    /// Una cadena que representa el nombre del personaje.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Establece una nueva paleta para los sprites.
    ///
    /// Esta función establece una nueva paleta para todos los sprites en la lista de sprites.
    ///
    /// # Argumentos
    ///
    /// * `palette_index` - El índice de la paleta a establecer.
    pub fn set_palette(&mut self, palette_index: usize) {
        let palette = self.pal_list.get(palette_index);
        for (_, sprite) in self.sprites.iter_mut() {
            sprite.set_palette(palette, &mut self.context);
        }
    }

    /// Configura las paletas para la versión 2 del archivo SFF.
    ///
    /// Esta función lee y configura las paletas desde el archivo SFF para la versión 2 del formato.
    ///
    /// # Argumentos
    ///
    /// * `sff_path` - La ruta al archivo Sff.
    ///
    /// # Retorna
    ///
    /// Un `Result` vacío indicando éxito o un `SffError` en caso de error.
    fn configure_pals_v2(&mut self, sff_path: PathBuf) -> Result<(), SffError> {
        let mut unique_pals: HashMap<[i16; 2], u16> = HashMap::new();
        let data = std::fs::read(sff_path)?;
        let mut bytes = Cursor::new(&data);
        bytes.seek(SeekFrom::Start(
            self.header.first_palette_header_offset as u64,
        ))?;

        for i in 0..self.header.number_of_palettes {
            bytes.seek(SeekFrom::Start(
                (self.header.first_palette_header_offset + (i * 16)) as u64,
            ))?;
            let mut gn_ = [0i16; 3];
            for ele in &mut gn_ {
                *ele = bytes.read_i16::<LittleEndian>()?;
            }
            let link = bytes.read_u16::<LittleEndian>()?;
            let ofs = bytes.read_u32::<LittleEndian>()?;
            let siz = bytes.read_u32::<LittleEndian>()?;

            let mut pal: Vec<Color>;
            let idx: i32;

            if let Some(&old) = unique_pals.get(&[gn_[0], gn_[1]]) {
                idx = old as i32;
                pal = self.pal_list.get(old as usize).clone();
            } else if siz == 0 {
                idx = link as i32;
                pal = self.pal_list.get(idx as usize).clone();
            } else {
                bytes.seek(SeekFrom::Start((self.header.lofs + ofs) as u64))?;
                pal = vec![Color::new(); 256];
                for i in 0..(siz as i32 / 4) {
                    if i >= pal.len() as i32 {
                        break;
                    }
                    let mut new_color = Color::new();

                    new_color.red = bytes.read_u8()?;
                    new_color.green = bytes.read_u8()?;
                    new_color.blue = bytes.read_u8()?;
                    new_color.alpha = bytes.read_u8()?;

                    if self.header.ver2 != 0 {
                        pal[i as usize] = new_color;
                    }
                }
                idx = i as i32;
            }

            unique_pals.insert([gn_[0], gn_[1]], idx as u16);
            self.pal_list.set_source(i as usize, &pal);
            self.pal_list.pal_table.insert([gn_[0], gn_[1]], idx);

            if i <= MAX_PAL_NO as u32
                && self.pal_list.pal_table[&[1, i as i16 + 1]]
                    == self.pal_list.pal_table[&[gn_[0] as i16, gn_[1] as i16]]
                && gn_[0] != 1
                && gn_[1] != i as i16 + 1
            {
                self.pal_list.pal_table.insert([1, i as i16 + 1], -1);
            }

            if i <= MAX_PAL_NO as u32 && i + 1 == self.header.number_of_palettes {
                for j in i + 1..MAX_PAL_NO as u32 {
                    self.pal_list.pal_table.remove(&[1, j as i16 + 1]);
                }
            }
        }

        Ok(())
    }
}

impl<'a> SffHeader {
    /// Crea un nuevo `SffHeader` con valores por defecto.
    ///
    /// # Retorna
    ///
    /// Retorna una instancia de `SffHeader` con todos los campos inicializados a cero.
    fn new() -> Self {
        Self {
            ver0: 0,
            ver1: 0,
            ver2: 0,
            ver3: 0,
            first_sprite_header_offset: 0,
            first_palette_header_offset: 0,
            number_of_sprites: 0,
            number_of_palettes: 0,
            lofs: 0,
            tofs: 0,
        }
    }

    /// Lee y analiza un archivo SFF y actualiza los campos del `SffHeader`.
    ///
    /// # Argumentos
    ///
    /// * `sff_path` - Una ruta al archivo SFF.
    ///
    /// # Retorna
    ///
    /// Retorna `Ok(())` si la lectura y el análisis fueron exitosos, o un `SffError` si hubo algún problema.
    ///
    /// # Errores
    ///
    /// Retorna `SffError::InvalidSignature` si la firma del archivo es inválida.
    /// Retorna `SffError::UnsupportedVersion` si la versión del archivo no es compatible.
    fn read(&mut self, sff_path: PathBuf) -> Result<(), SffError> {
        let data = std::fs::read(sff_path)?;

        if &data[0..12] != b"ElecbyteSpr\0" {
            return Err(SffError::InvalidSignature);
        }

        let mut bytes = Cursor::new(&data);
        bytes.set_position(12);

        self.ver3 = bytes.read_u8()?;
        self.ver2 = bytes.read_u8()?;
        self.ver1 = bytes.read_u8()?;
        self.ver0 = bytes.read_u8()?;

        bytes.read_u32::<LittleEndian>()?;

        let version = Version(self.ver0, self.ver1, self.ver2, self.ver3);

        match &self.ver0 {
            1 => {
                self.number_of_sprites = bytes.read_u32::<LittleEndian>()?;
                self.first_sprite_header_offset = bytes.read_u32::<LittleEndian>()?;

                bytes.read_u32::<LittleEndian>()?;
            }
            2 => {
                for _ in 0..4 {
                    bytes.read_u32::<LittleEndian>()?;
                }
                self.first_sprite_header_offset = bytes.read_u32::<LittleEndian>()?;
                self.number_of_sprites = bytes.read_u32::<LittleEndian>()?;
                self.first_palette_header_offset = bytes.read_u32::<LittleEndian>()?;
                self.number_of_palettes = bytes.read_u32::<LittleEndian>()?;

                self.lofs = bytes.read_u32::<LittleEndian>()?;

                bytes.read_u32::<LittleEndian>()?;

                self.tofs = bytes.read_u32::<LittleEndian>()?;
            }
            _ => {
                return Err(SffError::UnsupportedVersion(version));
            }
        }

        Ok(())
    }
}
