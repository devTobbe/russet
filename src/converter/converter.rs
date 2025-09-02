use regex::Regex;

struct ReplacementRule {
    source_color: String,
    target_color: String,
    regex: String,
}

struct Converter {
    replacements: Vec<ReplacementRule>,
}

impl ReplacementRule {
    fn new(&self, source_color: String, target_color: String, regex: String) -> Self {
        Self {
            source_color,
            target_color,
            regex,
        }
    }
}

impl Converter {
    fn new() {
        todo!();
    }
    // FIX THIS
    fn convert(&self, input: &str) -> String {
        todo!()
    }
}

// NOTE: OLD STUFF
//use regex::{Error, Regex};
//
//use crate::color::palette::Palette;
//
//pub fn replace_colors(
//    from_palette: &Palette,
//    to_palette: &Palette,
//    content: &str,
//) -> Result<String, Error> {
//    let result = String::from(content);
//
//    //   // TODO: Rewrite with new color enum
//    //   for (name, hex) in &from_palette.colors {
//    //       for (to_name, to_hex) in &to_palette.colors {
//    //           if name == to_name {
//    //               let str = format!("(?:){}", regex::escape(hex));
//    //               let re = Regex::new(&str)?;
//    //               // FIX: Add error handling, goes through even if regex "fails" technically
//    //               result = re.replace_all(&result, to_hex).into_owned();
//
//    //               println!("Replaced: {name}: {hex} → {to_hex}");
//    //           }
//    //       }
//    //   }
//    Ok(result)
//}
