use std::{
    fs::File,
    io::{self, Error},
    path::Path,
};

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

fn write_file(file_path: &str, contents: &str) {
    if !Path::new(file_path).exists() {
        let _ = File::create(file_path);
    }

    std::fs::write(file_path, contents).expect("Cannot write to file")
}

fn serialize_palette(p: Palette) -> String {
    match toml::to_string_pretty(&p) {
        Ok(toml_str) => toml_str,
        Err(e) => {
            eprint!("Failed to serialize the palette {}: {}", p.name, e);
            std::process::exit(1);
        }
    }
}
