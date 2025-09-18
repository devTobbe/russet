use serde::{Deserialize, Serialize};

use crate::models::hsl::Hsl;
use crate::models::rgb::Rgb;

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

pub fn identify(s: &str) -> ColorFormat {
    match s.to_lowercase().as_str() {
        "rgb" => ColorFormat::Rgb,
        "hsl" => ColorFormat::Hsl,
        _ => todo!(),
    }
}
impl ColorFormat {}

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

    #[test]
    fn test_identify() {
        let result = identify("hsl");
        let hsl = ColorFormat::Hsl;
        assert_eq!(result, hsl)
    }
}
