use serde::{Deserialize, Serialize};

use crate::models::hsl::Hsl;
use crate::models::rgb::Rgb;

// Represents a color and contains it's various representations

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Color {
    Rgb(Rgb),
    Hsl(Hsl),
}

#[derive(Debug, PartialEq)]
pub enum ColorFormat {
    Rgb,
    Hsl,
}

impl Color {
    // TODO: Add new function and getters, like get ColorFormat or smth
    // maybe rework this entire thing..?
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

impl From<Color> for String {
    fn from(c: Color) -> String {
        match c {
            Color::Rgb(rgb) => format!(
                "rgb({},{},{})",
                rgb.get_red(),
                rgb.get_green(),
                rgb.get_blue()
            )
            .to_string(),
            Color::Hsl(hsl) => format!(
                "hsl({},{},{})",
                hsl.get_hue(),
                hsl.get_saturation(),
                hsl.get_lightness()
            )
            .to_string(),
        }
    }
}

impl From<String> for ColorFormat {
    fn from(s: String) -> ColorFormat {
        match s.to_lowercase().as_str() {
            "rgb" => ColorFormat::Rgb,
            "hsl" => ColorFormat::Hsl,
            _ => todo!(),
        }
    }
}

impl From<&str> for ColorFormat {
    fn from(s: &str) -> ColorFormat {
        match s.to_lowercase().as_str() {
            "rgb" => ColorFormat::Rgb,
            "hsl" => ColorFormat::Hsl,
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Add more tests
    #[test]
    fn test_to_hsl() {
        let rgbcolor = Color::Rgb(Rgb::new(255, 255, 255));
        let hslassert = Color::Hsl(Hsl::new(0.0, 0.0, 1.0));
        let hsl = rgbcolor.to_hsl();

        assert_eq!(hsl, hslassert)
    }

    #[test]
    fn test_to_rgb() {
        let hslcolor = Color::Hsl(Hsl::new(0.0, 0.0, 1.0));
        let rgbassert = Color::Rgb(Rgb::new(255, 255, 255));
        let rgb = hslcolor.to_rgb();

        assert_eq!(rgb, rgbassert)
    }
}
