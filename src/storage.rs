use std::{
    fs::File,
    io::{self, Error},
    path::Path,
};

use toml::de::Error as TomlDeError;
use toml::ser::Error as TomlSerError;

use crate::palette::Palette;

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

fn write_file(file_path: &str, contents: &str) -> Result<(), Error> {
    if !Path::new(file_path).exists() {
        let _ = File::create(file_path);
    }

    std::fs::write(file_path, contents)
}

fn serialize_palette(p: &Palette) -> Result<String, TomlSerError> {
    toml::to_string_pretty(p)
}

fn deserialize_palette(s: &str) -> Result<Palette, TomlDeError> {
    toml::from_str(s)
}
