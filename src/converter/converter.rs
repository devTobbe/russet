use crate::color::color::Color;

struct ReplacementRule {
    source_color: Color,
    target_color: Color,
    regex: String,
}

struct Converter {
    replacements : Vec<ReplacementRule>
}

impl ReplacementRule {
}

impl Converter {
}
