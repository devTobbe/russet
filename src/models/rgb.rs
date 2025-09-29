use core::fmt;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::models::hsl::Hsl;

// RGB is a (u8, u8, 8)
// Uses several u8 to represent a the RGB Color Space
// Interpretation: 
// r represents the amount of red in the color (0-255)
// g represents the amount of green in the color (0-255)
// b represents the amount of blue in the color (0-255)
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct Rgb {
    r: u8, // 0-255
    g: u8, // 0-255
    b: u8, // 0-255
}

impl Rgb {
    // u8, u8, u8 -> RGB
    // Creates a new RGB object
    // given 255, 255, 255, Expect: (255,255,255)
    // given 23, 45, 56, Expect: (23,45,56)
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        let r = r.clamp(0, 255);
        let g = g.clamp(0, 255);
        let b = b.clamp(0, 255);
        Self { r, g, b }
    }

    pub fn get_red(&self) -> u8 {
        self.r
    }

    pub fn get_green(&self) -> u8 {
        self.g
    }

    pub fn get_blue(&self) -> u8 {
        self.b
    }

    // - -> String
    // Translates RGB to Hex Notation in the form of a string.
    // Given 255, 255, 255, Expect: "#ffffff"
    // Given 23, 45, 56, Expect: "#172d38"
    pub fn in_hex(&self) -> Result<String, Box<dyn Error>> {
        // TODO: Finish this
        todo!()
    }

    // &Str -> RGB
    // Creates a new RGB object with a Hex Notation in the form of a string.
    // Given: "#ffffff" Expect: 255, 255, 255 
    // Given: "#172d38" Expect 23, 45, 56, 
    pub fn new_from_hex(s: &str) -> Result<Rgb, Box<dyn Error>> {
        // TODO: ADD ERROR CHECKING ON LENGTH
        let s = s.trim_start_matches('#');
        let r_h = &s[0..2];
        let g_h = &s[2..4];
        let b_h = &s[4..6];

        let r = u8::from_str_radix(r_h, 16)?;
        let g = u8::from_str_radix(g_h, 16)?;
        let b = u8::from_str_radix(b_h, 16)?;

        Ok(Self { r, g, b })
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RGB({}, {}, {})", self.r, self.g, self.b)
    }
}

impl From<Hsl> for Rgb {
    // HSL -> RGB
    // Converts a HSL Color Space object to a RGB Color Space object.
    // Given: (123, 0.1, 0.65) Expect: (157, 175, 158)
    // Given: (360, 1.0, 1.0) Expect: (255, 255, 255)
    fn from(hsl: Hsl) -> Self {
        const NORM: f32 = 100.0;
        const SECTOR_SIZE: f32 = 60.0;
        const RGB_MAX: f32 = 255.0;
        const LIGHTNESS_SCALE: f32 = 2.0;
        const MAX_CHROMA: f32 = 1.0;

        // f32, f32 -> u8
        // Scales a float together with offset 'm' onto
        // an integer with range 0-255, rounding to nearest
        // RBG Value.
        fn scale_to_rgb(value: f32, m: f32) -> u8 {
            ((value + m) * RGB_MAX).round() as u8
        }

        // https://www.rapidtables.com/convert/color/hsl-to-rgb.html
        // Used this as a reference for the equation
        let h = hsl.get_hue();
        let s = hsl.get_saturation();
        let l = hsl.get_lightness();

        // Chroma, color intensity
        let c = (MAX_CHROMA - (LIGHTNESS_SCALE * l - MAX_CHROMA).abs()) * s;

        // Intermediate Value, Used for hue alignment
        let x = c * (MAX_CHROMA - ((h / SECTOR_SIZE).rem_euclid(2.0) - MAX_CHROMA).abs());

        // Match, Light adjustment offset
        let m = l - c / 2.0;

        // Calculates the size of the sector in which different RGB
        // mapping apply. Each sector corresponds to 60 deg segments.
        let sector = ((h / SECTOR_SIZE).floor() as i32).rem_euclid(6);

        // Choose outcomes depending on sector as specified in the
        // conversion from hsl to RGB
        let (r1, g1, b1) = match sector {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            5 => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };

        // Construct the final RGB
        let r = scale_to_rgb(r1, m);
        let g = scale_to_rgb(g1, m);
        let b = scale_to_rgb(b1, m);
        Rgb { r, g, b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEX: &str = "#001419";

    // TODO: Add more tests
    #[test]
    fn test_new_from_hex() {
        let s = HEX;
        let new_from_hex = Rgb::new_from_hex(s).unwrap();
        let test = Rgb::new(0, 20, 25);

        assert_eq!(new_from_hex, test);
    }

    #[test]
    fn test_get_red() {
        let test = Rgb::new(1, 2, 3);
        assert_eq!(test.get_red(), 1)
    }

    #[test]
    fn test_get_green() {
        let test = Rgb::new(1, 2, 3);
        assert_eq!(test.get_green(), 2)
    }

    #[test]
    fn test_get_blue() {
        let test = Rgb::new(1, 2, 3);
        assert_eq!(test.get_blue(), 3)
    }

    #[test]
    fn test_display() {
        let test = Rgb::new(1, 2, 3);
        assert_eq!(test.to_string(), "RGB(1, 2, 3)")
    }

    #[test]
    fn test_to_hsl() {
        let test = Rgb::new(156, 53, 23);
        let hsl_test: Hsl = test.into();
        let assert_hsl = Hsl::new(14.0, 0.74, 0.35);

        assert_eq!(hsl_test, assert_hsl)
    }
}
