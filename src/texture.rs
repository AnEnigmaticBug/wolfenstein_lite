use std::fs::File;
use std::io;
use std::path::Path;

use png::Decoder;

pub struct Texture {
    pub wd: usize,
    pub ht: usize,
    pub buf: Vec<u8>,
}

#[derive(Debug)]
pub enum TextureLoadError {
    IoError(io::Error),
    DecodingError(png::DecodingError),
}

impl From<io::Error> for TextureLoadError {
    fn from(src: io::Error) -> Self {
        TextureLoadError::IoError(src)
    }
}

impl From<png::DecodingError> for TextureLoadError {
    fn from(src: png::DecodingError) -> Self {
        TextureLoadError::DecodingError(src)
    }
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
