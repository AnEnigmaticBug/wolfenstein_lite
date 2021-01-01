use std::fmt;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;
use std::str::{Split, SplitTerminator};

use super::Map;
use MapReadError::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MapReadError {
    #[error("Couldn't read map file")]
    IoError(#[from] io::Error),
    #[error("Format/sntax error at line {line_no:?}")]
    ParsingError { line_no: usize },
    #[error("Missing grid entry in line {line_no:?}")]
    MissingEntry { line_no: usize },
}

/// Attempts to read a `Map` from `path`.
///
/// All map files start with a one line header containing space separated width
/// and height of the map. This is followed by a height x width grid specifying
/// textures of each cell of the map.
///
/// Each row's elements are concatenated together i.e there is no separator.
///
/// The grid's elements can be either `' '` (space) or a hex-digit. Space means
/// that the cell is empty and a hex-digit means the cell holds a wall with the
/// texture id equal to the value of hex-digit.
///
/// Use of hex-digits means that we can only have a maximum of 16 textures. IMO
/// this is not a problem.
pub fn read_map<P: AsRef<Path> + fmt::Debug>(path: P) -> Result<Map, MapReadError> {
    info!("Loading map at {:?}", path);

    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let mut lines = contents.split_terminator('\n');
    let header = lines.next().ok_or(ParsingError { line_no: 1 })?.split(' ');

    let (wd, ht) = read_dims(header)?;

    debug!("Map is {} by {}", ht, wd);

    let grid = read_grid(wd, ht, lines)?;

    Ok(Map { wd, ht, grid })
}

fn read_dims(mut header: Split<char>) -> Result<(usize, usize), MapReadError> {
    let wd = header
        .next()
        .ok_or(ParsingError { line_no: 1 })?
        .parse::<usize>()
        .map_err(|_| {
            error!("wd is not a +ve integer");
            ParsingError { line_no: 1 }
        })?;

    let ht = header
        .next()
        .ok_or(ParsingError { line_no: 1 })?
        .parse::<usize>()
        .map_err(|_| {
            error!("ht is not a +ve integer");
            ParsingError { line_no: 1 }
        })?;

    Ok((wd, ht))
}

fn read_grid(
    wd: usize,
    ht: usize,
    mut lines: SplitTerminator<char>,
) -> Result<Vec<Option<u8>>, MapReadError> {
    let mut grid = Vec::with_capacity(wd * ht);

    for y in 0..ht {
        let line_no = y + 2; // We've already read the header
        let mut chars = lines.next().ok_or(MissingEntry { line_no })?.chars();

        for _ in 0..wd {
            let c = chars.next().ok_or(MissingEntry { line_no })?;
            if c == ' ' {
                grid.push(None);
            } else {
                let d = c.to_digit(16).ok_or(ParsingError { line_no })?;
                grid.push(Some(d as u8));
            }
        }
    }

    Ok(grid)
}
