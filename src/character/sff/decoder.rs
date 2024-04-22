use std::{
    borrow::Cow,
    error::Error,
    fmt::Display,
    io::{self, Cursor, Seek, SeekFrom},
    str::{self, Utf8Error},
};

use byteorder::{LittleEndian, ReadBytesExt};


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SpriteId {
    pub group: u16,
    pub image: u16,
}

impl From<(u16, u16)> for SpriteId {
    fn from((group, image): (u16, u16)) -> Self {
        SpriteId { group, image }
    }
}

impl From<SpriteId> for (u16, u16) {
    fn from(SpriteId { group, image }: SpriteId) -> Self {
        (group, image)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PaletteKind {
    Individual = 0,
    Shared = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version(u8, u8, u8, u8);

impl From<Version> for (u8, u8, u8, u8) {
    fn from(v: Version) -> Self {
        (v.0, v.1, v.2, v.3)
    }
}


#[derive(Debug, Clone)]
pub struct Decoder<'a> {
    version: Version,
    groups_count: u32,
    images_count: u32,
    palette_kind: PaletteKind,
    comments: Comments<'a>,
    sprites: Vec<Sprite<'a>>,
}

#[derive(Debug)]
pub enum DecodeError {
    InvalidData(io::Error),
    InvalidSignature,
    UnsuporttedVersion(Version),
    InvalidPaletteKind,
    PreviousPaletteNotFound,
    LinkedSpriteNotFound {
        sprite_id: SpriteId,
        linked_index: u16,
    },
    ImageCountMismatch {
        expected_count: u32,
        found_count: u32,
    },
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeError::InvalidData(err) => err.fmt(f),
            DecodeError::InvalidSignature => write!(f, "invalid signature"),
            DecodeError::UnsuporttedVersion(v) => write!(f, "unsupported version {:?}", v),
            DecodeError::InvalidPaletteKind => write!(f, "invalid palette kind"),
            DecodeError::PreviousPaletteNotFound => write!(f, "previous palette not found"),
            DecodeError::LinkedSpriteNotFound {
                sprite_id,
                linked_index,
            } => write!(
                f,
                "invalid link {} for sprite {}-{}",
                linked_index, sprite_id.group, sprite_id.image
            ),
            DecodeError::ImageCountMismatch {
                expected_count,
                found_count,
            } => write!(f, "expected {expected_count} images, found {found_count}"),
        }
    }
}

impl Error for DecodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DecodeError::InvalidData(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for DecodeError {
    fn from(error: io::Error) -> Self {
        DecodeError::InvalidData(error)
    }
}

impl<'a> Decoder<'a> {
    pub fn decode(data: &'a [u8]) -> Result<Self, DecodeError> {
        if &data[0..12] != b"ElecbyteSpr\0" {
            return Err(DecodeError::InvalidSignature);
        }

        let mut bytes = Cursor::new(&data);
        bytes.set_position(12);

        let version_low3 = bytes.read_u8()?;
        let version_low2 = bytes.read_u8()?;
        let version_low1 = bytes.read_u8()?;
        let version_high = bytes.read_u8()?;
        let version = Version(version_high, version_low1, version_low2, version_low3);
        if version_high != 1 {
            return Err(DecodeError::UnsuporttedVersion(version));
        }

        let groups_count = bytes.read_u32::<LittleEndian>()?;
        let images_count = bytes.read_u32::<LittleEndian>()?;
        let first_subfile_offset = bytes.read_u32::<LittleEndian>()?;
        let subfile_header_size = bytes.read_u32::<LittleEndian>()?;
        let palette_kind = match bytes.read_u8()? {
            0 => PaletteKind::Individual,
            1 => PaletteKind::Shared,
            _ => return Err(DecodeError::InvalidPaletteKind),
        };

        let comments = Comments(&data[36..511]);

        let mut images: Vec<Sprite> = Vec::with_capacity(images_count as usize);
        let mut next_subfile_offset = Some(first_subfile_offset);
        let mut previous_palette_offset = None;
        let total_size = data.len() as u32;
        while let Some(offset) = next_subfile_offset.take() {
            bytes.set_position(offset.into());

            next_subfile_offset = bytes.read_u32::<LittleEndian>().ok().and_then(|n| match n {
                n if n > 0 && n + subfile_header_size <= total_size => Some(n),
                _ => None,
            });

            let size = bytes.read_u32::<LittleEndian>()? as usize;
            let x = bytes.read_i16::<LittleEndian>()?;
            let y = bytes.read_i16::<LittleEndian>()?;
            let group = bytes.read_u16::<LittleEndian>()?;
            let image = bytes.read_u16::<LittleEndian>()?;
            let linked_index = bytes.read_u16::<LittleEndian>()?;
            let use_previous_palette = bytes.read_u8()? != 0;

            bytes.seek(SeekFrom::Current(13))?;

            let sprite_id = SpriteId { group, image };
            let coordinates = Coordinates { x, y };

            let data_offset = bytes.position() as usize;
            let palette_size = 256 * 3;

            let (data_size, palette_offset) = match previous_palette_offset {
                None if use_previous_palette => return Err(DecodeError::PreviousPaletteNotFound),
                Some(offset) if use_previous_palette => (size, offset),
                _ => {
                    let offset = (data_offset + size) - palette_size;
                    previous_palette_offset = Some(offset);
                    (size - palette_size, offset)
                }
            };

            let sprite = if size == 0 {
                let linked_sprite =
                    images
                        .get(linked_index as usize)
                        .ok_or(DecodeError::LinkedSpriteNotFound {
                            sprite_id,
                            linked_index,
                        })?;

                Sprite {
                    id: sprite_id,
                    coordinates,
                    ..linked_sprite.clone()
                }
            } else {
                Sprite {
                    id: sprite_id,
                    data: Cow::Borrowed(&data[data_offset..data_offset + data_size]),
                    palette: Cow::Borrowed(&data[palette_offset..palette_offset + palette_size]),
                    coordinates,
                }
            };

            images.push(sprite);
        }

        if images.len() != images_count as usize {
            return Err(DecodeError::ImageCountMismatch {
                expected_count: images_count,
                found_count: images.len() as u32,
            });
        }

        Ok(Decoder {
            version,
            groups_count,
            images_count,
            palette_kind,
            comments,
            sprites: images,
        })
    }

    pub fn comments(&self) -> &Comments {
        &self.comments
    }

    pub fn palette_kind(&self) -> PaletteKind {
        self.palette_kind
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn groups_count(&self) -> u32 {
        self.groups_count
    }

    pub fn images_count(&self) -> u32 {
        self.images_count
    }

    pub fn sprites(&self) -> impl Iterator<Item = &Sprite> {
        self.sprites.iter()
    }
}

impl<'a> IntoIterator for Decoder<'a> {
    type Item = Sprite<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.sprites.into_iter()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Coordinates {
    pub x: i16,
    pub y: i16,
}

impl From<(i16, i16)> for Coordinates {
    fn from((x, y): (i16, i16)) -> Self {
        Self { x, y }
    }
}

impl From<Coordinates> for (i16, i16) {
    fn from(Coordinates { x, y }: Coordinates) -> Self {
        (x, y)
    }
}

#[derive(Debug, Clone)]
pub struct Sprite<'a> {
    id: SpriteId,
    data: Cow<'a, [u8]>,
    palette: Cow<'a, [u8]>,
    coordinates: Coordinates,
}

impl<'a> Sprite<'a> {
    pub fn id(&self) -> SpriteId {
        self.id
    }

    pub fn coordinates(&self) -> Coordinates {
        self.coordinates
    }

    pub fn raw_data(&self) -> &[u8] {
        &self.data
    }

    pub fn palette(&self) -> &[u8] {
        &self.palette
    }

    pub fn to_owned(&self) -> Sprite<'static> {
        Sprite {
            id: self.id,
            data: Cow::Owned(self.raw_data().to_owned()),
            palette: Cow::Owned(self.palette().to_owned()),
            coordinates: self.coordinates,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comments<'a>(&'a [u8]);

impl<'a> Comments<'a> {
    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        Ok(str::from_utf8(self.0)?.trim_end_matches('\u{0}'))
    }
}