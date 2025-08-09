use std::{
    fs::File,
    io::{self, Error},
    path::Path,
};

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

