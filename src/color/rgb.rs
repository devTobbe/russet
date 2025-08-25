use core::fmt;
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb {
    pub r: u8, // 0-255
    pub g: u8, // 0-255
    pub b: u8, // 0-255
}

impl Rgb {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    fn to_hex(&self) -> Result<String, Box<dyn Error>> {
        todo!()
    }

    fn from_hex(s: &str) -> Result<Rgb, Box<dyn Error>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "#001419";
        let from_hex = Rgb::from_hex(s).unwrap();
        let test = Rgb::new(0, 20, 25);

        assert_eq!(from_hex, test);
    }
}
