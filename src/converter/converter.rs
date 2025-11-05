// Problem Statement:
// Given a String s, Palette to and from and format f, Go through the string s
// and replace every instance of color in palette to to a corresponding color in
// palette from in the correct given color format f
//
// Potential string matches
// Problem Information:
// This is the information that must be represented in data
// "background: #ff0000;"
// "color: rgb(255, 0, 0);"
// "border-color: red;"
// "fill: hsl(0, 100%, 50%);"
//
// Format:
// RGB
// RGBA (Later)
// HEX
// HSL
//
// String s (config file)

// String, Palette, Palette, Format -> String
// Orchstrates the conversion between one color to the next, top level function.
// fn main(s : String, to Palette, from : Palette, f : format) -> String {
//    todo!();
//}

// Format -> List(Regex)
// Turns a given format f to a list of regex depending on the corresponding
// format.
// fn format_to_regex(f : format) -> Regex {
//    match f {
//      RGB -> {todo!();},
//      RGBA -> {todo!();},
//      HSL -> {todo!();},
//    }
//}

// String, List(Regex), Palette, Palette -> String
// Given a string s, applies regex replace from palette to into palette from and
// returns the resulting string.
// fn apply_replace(lr : String s, lr : Vec<Regex>, to : Palette, from : Palette -> String {
// for each i in lr {
//  todo!();
// }
//}
//
// Input Data:
// "background: #ff0000;"
// "color: rgb(255, 0, 0);"
// "border-color: red;"
// "fill: hsl(0, 100%, 50%);"
//
// Ouput Data (Different):
// "background: #ff0000;"
// "color: rgb(255, 0, 0);"
// "border-color: red;"
// "fill: hsl(0, 100%, 50%);"
//
// FLOW OF CONTROL:
// 1. PARSE FILE - Storage.rs
// 2. FIND COLORS - Use Regex
// 3. PARSE EACH COLOR
// 4. REPLACE COLORS
// 5. GENERATE NEW FILE
//
//Proceed with REGEX plan, maybe refactor later on

use std::error::Error;

use regex::Regex;

use crate::{
    models::{color::ColorFormat, config::Config},
    storage::read_file,
};

// Config -> _
// Takes a configuration conf file and produces a new file according to the
// configuration.
fn conversion(conf: Config) -> Result<(), Box<dyn Error>> {
    // Line up data
    let content = read_file(conf.output())?;
    let format: ColorFormat = conf.format().into();
    let re_str = get_regex(format);
    // NOTE: CREATE REGEX
    let re = Regex::new(&re_str)?;
    //       TODO:
    //       Fetch From and To Palette
    //       Loop through from colors and create captures
    //       for each corresponding to color, replace it
    //       Save to new file

    todo!();
}

// ColorFormat -> String
// Takes a colorformat format and returns a Regex String.
fn get_regex(f: ColorFormat) -> String {
    // TODO: Add more patterns later and break down RGB pattern maybe?
    let rgb_pattern = r"rgba?\(\s*(?:\d{1,3}%?\s*,\s*){2}\d{1,3}%?(?:\s*,\s*(?:0|1|0?\.\d+))?\s*\)";
    let hsl_pattern = r"hsla?\(\s*\d{1,3}(?:\.\d+)?(?:deg|rad|grad|turn)?\s*,\s*\d{1,3}%\s*,\s*\d{1,3}%\s*(?:,\s*(?:0|1|0?\.\d+))?\s*\)";

    match f {
        ColorFormat::Rgb => rgb_pattern.to_string(),
        ColorFormat::Hsl => hsl_pattern.to_string(),
        _ => {
            todo!()
        }
    }
}
