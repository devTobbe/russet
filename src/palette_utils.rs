use regex::Regex;

use crate::palette::Palette;

fn replace_colors(from_palette: Palette, to_palette: Palette, content: String) -> String {
    let mut result = content;

    for (name, hex) in &from_palette.colors {
        for (to_name, to_hex) in &to_palette.colors {
            if name == to_name {
                let str = format!("(?i){}", regex::escape(hex));
                let re = Regex::new(&str).unwrap();
                result = re.replace_all(&result, to_hex).into_owned();

                println!("Replaced: {name}{hex} →  {to_hex}");
            }
        }
    }
    result
}
