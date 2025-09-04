use serde::{Deserialize, Serialize};

use crate::color::hsl::Hsl;
use crate::color::rgb::Rgb;

#[derive(Debug, Deserialize, Serialize)]
pub enum Color {
    Rgb(Rgb),
    Hsl(Hsl),
}

impl Color {
    pub fn to_hsl(&self) -> Hsl {
        match self {
            Color::Rgb(rgb) => (*rgb).into(),
            Color::Hsl(hsl) => *hsl,
        }
    }
    pub fn to_rgb(&self) -> Rgb {
        match self {
            Color::Rgb(rgb) => *rgb,
            Color::Hsl(hsl) => (*hsl).into(),
        }
    }
}
