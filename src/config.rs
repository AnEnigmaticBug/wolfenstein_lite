use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

use serde::Deserialize;
use toml::de;
use thiserror::Error;

use crate::primitive::Vec2;

#[derive(Deserialize)]
pub struct Config {
    pub screen: ScreenConfig,
    pub assets: AssetsConfig,
    pub player: PlayerConfig,
    pub misc: MiscConfig,
}

#[derive(Deserialize)]
pub struct ScreenConfig {
    pub wd: u32,
    pub ht: u32,
}

#[derive(Deserialize)]
pub struct AssetsConfig {
    pub tex: Vec<String>,
    pub map: String,
}

#[derive(Deserialize)]
pub struct PlayerConfig {
    pub fov: Option<f32>,
    pub speed: f32,
    pub initial_dir: Vec2,
    pub initial_pos: Vec2,
}

#[derive(Deserialize)]
pub struct MiscConfig {
    pub floor_tex: usize,
    pub wall_ht_scale: Option<f32>,
}

#[derive(Debug, Error)]
pub enum ConfigReadError {
    #[error("Couldn't read config file")]
    IoError(#[from] io::Error),
    #[error("Format error")]
    ParsingError(#[from] de::Error),
}

impl Config {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Config, io::Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(toml::from_str(&contents)?)
    }
}
