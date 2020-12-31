use std::fs::File;
use std::io;
use std::path::Path;

use png::Decoder;
use thiserror::Error;

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
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, TextureLoadError> {
        let file = File::open(path)?;
        let (info, mut reader) = Decoder::new(file).read_info()?;
        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf)?;

        Ok(Texture {
            wd: info.width as usize,
            ht: info.height as usize,
            buf,
        })
    }
}
