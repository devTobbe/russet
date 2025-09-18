use serde::{Deserialize, Serialize};

use crate::models::hsl::Hsl;
use crate::models::rgb::Rgb;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Color {
    Rgb(Rgb),
    Hsl(Hsl),
}

pub enum ColorFormat {
    Rgb,
    Hsl,
}

impl Color {
    pub fn to_hsl(&self) -> Color {
        match self {
            Color::Rgb(rgb) => Color::Hsl((*rgb).into()),
            Color::Hsl(hsl) => Color::Hsl(*hsl),
        }
    }
    pub fn to_rgb(&self) -> Color {
        match self {
            Color::Rgb(rgb) => Color::Rgb(*rgb),
            Color::Hsl(hsl) => Color::Rgb((*hsl).into()),
        }
    }
}

impl ColorFormat {
    pub fn identify(s: &str) -> ColorFormat {
        match s.to_lowercase().as_str() {
            "rgb" => ColorFormat::Rgb,
            "hsl" => ColorFormat::Hsl,
            _ => todo!(),
        }
    }
}
