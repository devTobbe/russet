use crate::color::hsl::Hsl;
use crate::color::rgb::Rgb;

#[derive(Debug)]
pub enum Color {
    Rgb(Rgb),
    Hsl(Hsl),
}

impl Color {
    fn to_rgb(&self) -> Color {
        match self {
            Color::Rgb(rgb) => Color::Rgb(*rgb),
            Color::Hsl(hsl) => todo!(),
        }
    }
    fn to_hsl(&self) -> Color {
        match self {
            Color::Rgb(rgb) => todo!(),
            Color::Hsl(hsl) => Color::Hsl(*hsl),
        }
    }
}
