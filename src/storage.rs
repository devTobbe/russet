use std::{
    error::Error,
    fs::File,
    io::{self},
    path::Path,
};

use crate::models::palette::{Palette, Palettes};

pub fn read_file(file_path: &str) -> Result<String, io::Error> {
    let path = Path::new(&file_path);

    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File doesn't exist",
        ));
    }

    std::fs::read_to_string(file_path)
}

pub fn write_file(file_path: &str, contents: &str) -> Result<(), io::Error> {
    if !Path::new(file_path).exists() {
        let _ = File::create(file_path);
    }

    std::fs::write(file_path, contents)
}

pub fn serialize_palettes(p: &Palettes) -> Result<String, toml::ser::Error> {
    toml::to_string_pretty(p)
}

pub fn deserialize_palettes(s: &str) -> Result<Palettes, toml::de::Error> {
    toml::from_str(s)
}

pub fn deserialize_palette(s: &str) -> Result<Palette, toml::de::Error> {
    toml::from_str(s)
}

// Get a palette from a file by name
pub fn get_palette_from_name(s: &str, name: &str) -> Result<Palette, Box<dyn Error>> {
    let palettes = deserialize_palettes(s)?;
    let palette = palettes.get_palette(name).ok_or_else(|| {
        Box::<dyn std::error::Error>::from(format!("Palette '{}' not found", name))
    })?;
    Ok(palette.clone())
}
