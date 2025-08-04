use std::{
    collections::HashMap,
    fs::File,
    io::{self, Error},
    path::Path,
};

#[derive(Debug)]
struct Palette {
    name: String,
    colors: HashMap<String, String>,
}

fn main() {
    let solarized = Palette {
        name: "Solarized Osaka".to_string(),
        colors: HashMap::from([
            ("base03".to_string(), "#001419".to_string()),
            ("base02".to_string(), "#002b36".to_string()),
            ("base01".to_string(), "#586e75".to_string()),
            ("base00".to_string(), "#657b83".to_string()),
            ("base0".to_string(), "#839496".to_string()),
            ("base1".to_string(), "#93a1a1".to_string()),
            ("base2".to_string(), "#eee8d5".to_string()),
            ("base3".to_string(), "#fdf6e3".to_string()),
            ("yellow".to_string(), "#b58900".to_string()),
            ("orange".to_string(), "#cb4b16".to_string()),
            ("red".to_string(), "#dc322f".to_string()),
            ("magenta".to_string(), "#d33682".to_string()),
            ("violet".to_string(), "#6c71c4".to_string()),
            ("blue".to_string(), "#268bd2".to_string()),
            ("cyan".to_string(), "#2aa198".to_string()),
            ("green".to_string(), "#859900".to_string()),
        ]),
    };

    let catppuccin = Palette {
        name: "Catppuccin Mocha".to_string(),
        colors: HashMap::from([
            ("rosewater".to_string(), "#f5e0dc".to_string()),
            ("flamingo".to_string(), "#f2cdcd".to_string()),
            ("pink".to_string(), "#f5c2e7".to_string()),
            ("mauve".to_string(), "#cba6f7".to_string()),
            ("red".to_string(), "#f38ba8".to_string()),
            ("maroon".to_string(), "#eba0ac".to_string()),
            ("peach".to_string(), "#fab387".to_string()),
            ("yellow".to_string(), "#f9e2af".to_string()),
            ("green".to_string(), "#a6e3a1".to_string()),
            ("teal".to_string(), "#94e2d5".to_string()),
            ("sky".to_string(), "#89dceb".to_string()),
            ("sapphire".to_string(), "#74c7ec".to_string()),
            ("blue".to_string(), "#89b4fa".to_string()),
            ("lavender".to_string(), "#b4befe".to_string()),
            ("text".to_string(), "#cdd6f4".to_string()),
            ("subtext1".to_string(), "#bac2de".to_string()),
            ("subtext0".to_string(), "#a6adc8".to_string()),
            ("overlay2".to_string(), "#9399b2".to_string()),
            ("overlay1".to_string(), "#7f849c".to_string()),
            ("overlay0".to_string(), "#6c7086".to_string()),
            ("surface2".to_string(), "#585b70".to_string()),
            ("surface1".to_string(), "#45475a".to_string()),
            ("surface0".to_string(), "#313244".to_string()),
            ("base".to_string(), "#1e1e2e".to_string()),
            ("mantle".to_string(), "#181825".to_string()),
            ("crust".to_string(), "#11111b".to_string()),
        ]),
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

fn replace_colors(from_palette: Palette, to_palette: Palette, content: String) -> String {
    todo!()
}
