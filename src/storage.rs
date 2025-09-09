use std::{
    fs::File,
    io::{self, Error},
    path::Path,
};

use toml::de::Error as TomlDeError;
use toml::ser::Error as TomlSerError;

use crate::models::palette::{Palette, Palettes};


pub fn read_file(file_path: &str) -> Result<String, Error> {
    let path = Path::new(&file_path);

    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File doesn't exist",
        ));
    }

    std::fs::read_to_string(file_path)
}

pub fn write_file(file_path: &str, contents: &str) -> Result<(), Error> {
    if !Path::new(file_path).exists() {
        let _ = File::create(file_path);
    }

    std::fs::write(file_path, contents)
}

pub fn serialize_palettes(p: &Palettes) -> Result<String, TomlSerError> {
    toml::to_string_pretty(p)
}

pub fn deserialize_palettes(s: &str) -> Result<Palettes, TomlDeError> {
    toml::from_str(s)
}

pub fn deserialize_palette(s: &str) -> Result<Palette, TomlDeError> {
    toml::from_str(s)
}
