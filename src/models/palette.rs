use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::color::Color;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Palette {
    name: String,
    colors: HashMap<String, Color>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Palettes {
    #[serde(rename = "palette")]
    palette_collection: Vec<Palette>,
}

// TODO: Add getter for Palettes, I suppose, maybe refactor

impl Palette {
    pub fn new(name: String, colors: HashMap<String, Color>) -> Palette {
        // TODO: Add checks for duplicates
        Self { name, colors }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn colors(&self) -> &HashMap<String, Color> {
        &self.colors
    }

    fn convert_all<F>(&mut self, f: F)
    where
        F: Fn(&Color) -> Color,
    {
        for color in self.colors.values_mut() {
            *color = f(color);
        }
    }

    pub fn convert_all_to_rgb(&mut self) {
        self.convert_all(|c| c.to_rgb());
    }

    pub fn convert_all_to_hsl(&mut self) {
        self.convert_all(|c| c.to_hsl());
    }
}

impl Palettes {
    pub fn palette_collection(&self) -> &Vec<Palette> {
        &self.palette_collection
    }

    pub fn clone_palette(&self, name: &str) -> Option<Palette> {
        self.palette_collection()
            .iter()
            .find(|p| p.name() == name)
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{hsl::Hsl, rgb::Rgb};

    use super::*;

    // Test Helpers
    fn sample_palette(name: &str) -> Palette {
        let col = Color::Rgb(Rgb::new(1, 2, 3));
        let col_name = name.to_string();
        let mut hashm = HashMap::new();
        hashm.insert(col_name, col);
        let test_name = "test".to_string();
        Palette::new(test_name, hashm)
    }

    fn sample_palette_hsl(name: &str) -> Palette {
        let col = Color::Hsl(Hsl::new(350.0, 0.6, 0.3));
        let col_name = name.to_string();
        let mut hashm = HashMap::new();
        hashm.insert(col_name, col);
        let test_name = "test".to_string();
        Palette::new(test_name, hashm)
    }

    fn sample_palettes() -> Palettes {
        Palettes {
            palette_collection: vec![sample_palette("solarized"), sample_palette("gruvbox")],
        }
    }

    #[test]
    fn test_get_name() {
        let p = sample_palette("test");
        assert_eq!("test", p.name())
    }

    #[test]
    fn test_get_palette() {
        let p = sample_palette("test");
        let pcol = p.colors();
        let col = Color::Rgb(Rgb::new(1, 2, 3));
        let col_name = "test".to_string();
        let mut hashm = HashMap::new();
        hashm.insert(col_name, col);
        assert_eq!(pcol, &hashm);
    }

    #[test]
    fn test_convert_all_to_rgb() {
        let mut p1 = sample_palette_hsl("test");
        p1.convert_all_to_rgb();

        let col = Color::Rgb(Rgb::new(122, 31, 46));
        let col_name = "test".to_string();
        let mut hashm = HashMap::new();
        hashm.insert(col_name, col);
        let test_name = "test".to_string();
        let p2 = Palette::new(test_name, hashm);

        assert_eq!(p1, p2)
    }

    #[test]
    fn test_convert_all_to_hsl() {
        let mut p1 = sample_palette("test");
        p1.convert_all_to_hsl();

        let col = Color::Hsl(Hsl::new(210.0, 0.5, 0.01));
        let col_name = "test".to_string();
        let mut hashm = HashMap::new();
        hashm.insert(col_name, col);
        let test_name = "test".to_string();
        let p2 = Palette::new(test_name, hashm);

        assert_eq!(p1, p2)
    }
}
