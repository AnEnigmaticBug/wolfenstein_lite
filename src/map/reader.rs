use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;
use std::str::{Split, SplitTerminator};

use super::Map;
use MapReadError::*;

#[derive(Debug)]
pub enum MapReadError {
    IoError(io::Error),
    ParsingError { line_no: usize },
    MissingEntry { line_no: usize },
}

impl From<io::Error> for MapReadError {
    fn from(src: io::Error) -> Self {
        IoError(src)
    }
}

pub fn read_map<P: AsRef<Path>>(path: P) -> Result<Map, MapReadError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let mut lines = contents.split_terminator('\n');
    let header = lines.next().ok_or(ParsingError { line_no: 1 })?.split(' ');

    let (wd, ht) = read_dims(header)?;
    let grid = read_grid(wd, ht, lines)?;

    Ok(Map { wd, ht, grid })
}

fn read_dims(mut header: Split<char>) -> Result<(usize, usize), MapReadError> {
    let wd = header
        .next()
        .ok_or(ParsingError { line_no: 1 })?
        .parse::<usize>()
        .map_err(|_| ParsingError { line_no: 1 })?;

    let ht = header
        .next()
        .ok_or(ParsingError { line_no: 1 })?
        .parse::<usize>()
        .map_err(|_| ParsingError { line_no: 1 })?;

    Ok((wd, ht))
}

fn read_grid(
    wd: usize,
    ht: usize,
    mut lines: SplitTerminator<char>,
) -> Result<Vec<u8>, MapReadError> {
    let mut grid = Vec::with_capacity(wd * ht);

    for y in 0..ht {
        let line_no = y + 2; // We've already read the header
        let mut chars = lines.next().ok_or(MissingEntry { line_no })?.chars();

        for _ in 0..wd {
            let c = chars.next().ok_or(MissingEntry { line_no })?;
            if c == ' ' {
                grid.push(0);
            } else {
                let d = c.to_digit(16).ok_or(ParsingError { line_no })?;
                grid.push(d as u8);
            }
        }
    }

    Ok(grid)
}
