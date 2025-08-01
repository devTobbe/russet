use std::{io::{self, Error}, path::Path};

fn main() {
    println!("Hello, world!");
}

fn read_file(file_path: &str) -> Result<String, Error> {

    let path = Path::new(&file_path);

    if !path.exists(){
        return Err(io::Error::new(io::ErrorKind::NotFound, "File doesn't exist"));
    }


    std::fs::read_to_string(file_path)
}
