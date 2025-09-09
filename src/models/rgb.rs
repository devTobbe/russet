use core::fmt;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::models::hsl::Hsl;


#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct Rgb {
    r: u8, // 0-255
    g: u8, // 0-255
    b: u8, // 0-255
}

impl Rgb {
    fn new(r: u8, g: u8, b: u8) -> Self {
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

    pub fn in_hex(&self) -> Result<String, Box<dyn Error>> {
        todo!()
    }

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
    fn from(hsl: Hsl) -> Self {
        const NORM: f32 = 100.0;
        const SECTOR_SIZE: f32 = 60.0;
        const RGB_MAX: f32 = 255.0;
        const LIGHTNESS_SCALE: f32 = 2.0;
        const MAX_CHROMA: f32 = 1.0;

        fn scale_to_rgb(value: f32, m: f32) -> u8 {
            ((value + m) * RGB_MAX).round() as u8
        }
        // https://www.rapidtables.com/convert/color/hsl-to-rgb.html
        // Used this as a reference for the equation
        let h = hsl.get_hue();
        let s = hsl.get_saturation();
        let l = hsl.get_lightness();

        // Normalize, h is already fine and does not need to be normalized
        let s = s / NORM;
        let l = l / NORM;

        // Chroma
        let c = (MAX_CHROMA - (LIGHTNESS_SCALE * l - MAX_CHROMA).abs()) * s;

        // Intermediate Value
        let x = c * (MAX_CHROMA - ((h / SECTOR_SIZE).rem_euclid(2.0) - MAX_CHROMA).abs());

        // Match
        let m = l - c / 2.0;

        // Calculates the size of the sector in which different RGB
        // mapping apply. Each sector corresponds to 60 deg segments.
        let sector = (h / SECTOR_SIZE).floor() as i32;

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

    // TODO: Add more tests
    #[test]
    fn test() {
        let s = "#001419";
        let new_from_hex = Rgb::new_from_hex(s).unwrap();
        let test = Rgb::new(0, 20, 25);

        assert_eq!(new_from_hex, test);
    }
}
