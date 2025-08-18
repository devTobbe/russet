use core::fmt;
use std::error::Error;

#[derive(Debug)]
struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    fn to_hex(&self) -> Result<String, Box<dyn Error>> {
        todo!()
    }

    fn from_hex(s: &str) -> Result<Rgb, Box<dyn Error>> {
        // TODO: Set as self maybe? Has to be mut then...
        todo!()
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RGB({}, {}, {})", self.r, self.g, self.b)
    }
}
