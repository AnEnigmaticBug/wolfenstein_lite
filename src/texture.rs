use std::fmt;
use std::fs::File;
use std::io;
use std::path::Path;

use png::Decoder;
use thiserror::Error;

/// Represents a texture which can be used to draw stuff like walls, roof etc.
///
/// It is simply a collection of byte values. Whether RGB/RGBA is used depends
/// upon the source image file. The current assumption is that:
/// * sprite textures will be RGBA since they need transparency
/// * all other stuff (wall & roof textures) will be RGB
pub struct Texture {
    pub wd: usize,
    pub ht: usize,
    pub buf: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum TextureLoadError {
    #[error("Couldn't open texture")]
    IoError(#[from] io::Error),
    #[error("Error in decoding png")]
    DecodingError(#[from] png::DecodingError),
}

impl Texture {
    /// Loads a `Texture` from `path`.
    pub fn load<P: AsRef<Path> + fmt::Debug>(path: P) -> Result<Self, TextureLoadError> {
        info!("Loading texture at {:?}", path);

        let file = File::open(path)?;
        let (info, mut reader) = Decoder::new(file).read_info()?;
        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf)?;

        info!("Texture is {:?}", info.color_type);

        Ok(Texture {
            wd: info.width as usize,
            ht: info.height as usize,
            buf,
        })
    }
}
