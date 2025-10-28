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
// RGBA
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
// 1. PARSE FILE
// 2. FIND COLORS
// 3. PARSE EACH COLOR
// 4. REPLACE COLORS
// 5. GENERATE NEW FILE
//
