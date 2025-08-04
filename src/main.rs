use std::{
    fs::File,
    io::{self, Error},
    path::Path,
};

#[derive(Debug)]
struct Palette {
    name: String,
    base03: String,
    base02: String,
    base01: String,
    base00: String,
    base1: String,
    base2: String,
    base3: String,
    yellow: String,
    orange: String,
    red: String,
    magenta: String,
    violet: String,
    blue: String,
    cyan: String,
    green: String,
}

fn main() {
    let _solarized = Palette {
        name: "Solarized".to_string(),
        base03: "#002b36".to_string(),
        base02: "#073642".to_string(),
        base01: "#586e75".to_string(),
        base00: "#657b83".to_string(),
        base1: "#839496".to_string(),
        base2: "#93a1a1".to_string(),
        base3: "#fdf6e3".to_string(),
        yellow: "#b58900".to_string(),
        orange: "#cb4b16".to_string(),
        red: "#dc322f".to_string(),
        magenta: "#d33682".to_string(),
        violet: "#6c71c4".to_string(),
        blue: "#268bd2".to_string(),
        cyan: "#2aa198".to_string(),
        green: "#859900".to_string(),
    };

    let catppuccin = Palette {
        name: "Catppuccin Mocha".to_string(),
        base03: "#1e1e2e".to_string(), 
        base02: "#181825".to_string(), 
        base01: "#313244".to_string(), 
        base00: "#45475a".to_string(), 
        base1: "#585b70".to_string(),  
        base2: "#6c7086".to_string(),  
        base3: "#cdd6f4".to_string(),  
        yellow: "#f9e2af".to_string(),
        orange: "#fab387".to_string(),
        red: "#f38ba8".to_string(),
        magenta: "#f5c2e7".to_string(),
        violet: "#cba6f7".to_string(),
        blue: "#89b4fa".to_string(),
        cyan: "#94e2d5".to_string(),
        green: "#a6e3a1".to_string(),
    };
}

fn read_file(file_path: &str) -> Result<String, Error> {
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
