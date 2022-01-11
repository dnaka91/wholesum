use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
};

use anyhow::{Context, Result};

use crate::{Algorithm, Mode};

#[derive(Debug)]
pub struct HashFile {
    pub algorithm: Option<Algorithm>,
    pub entries: Vec<HashEntry>,
}

impl HashFile {
    pub fn parse(file: PathBuf) -> Result<Self> {
        Ok(Self {
            algorithm: None,
            entries: BufReader::new(File::open(file)?)
                .lines()
                .map(|line| line?.parse())
                .collect::<Result<_>>()?,
        })
    }
}

#[derive(Debug)]
pub struct HashEntry {
    pub algorithm: Option<Algorithm>,
    pub hash: String,
    pub mode: Mode,
    pub file: String,
}

impl FromStr for HashEntry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let algo = parts.next().context("line too short")?;
        let hash = parts.next().context("line too short")?;
        let file = parts.next();

        let (algorithm, hash, file) = if let Some(file) = file {
            (Some(algo.parse()?), hash, file)
        } else {
            (None, algo, hash)
        };

        let hash = hash.to_owned();

        let mode = if file.starts_with('*') {
            Mode::Binary
        } else {
            Mode::Text
        };

        let file = file.strip_prefix('*').unwrap_or(file).to_owned();

        Ok(Self {
            algorithm,
            hash,
            mode,
            file,
        })
    }
}
