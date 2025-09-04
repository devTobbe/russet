use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::color::color::{Color};

#[derive(Debug, Serialize, Deserialize)]
pub struct Palette {
    name: String,
    colors: HashMap<String, Color>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Palettes {
    #[serde(rename = "palette")]
    palette_collection: Vec<Palette>,
}

impl Palette {
    pub fn new(name: String, colors: HashMap<String, Color>) -> Palette {
        // TODO: Add checks for duplicates
        Self { name, colors }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_palette(&self) -> &HashMap<String, Color> {
        &self.colors
    }

    pub fn convert_all_to_rgb(&mut self) {
        self.colors = self
            .colors
            .iter()
            .map(|(name, color)| (name.clone(), Color::Rgb(color.to_rgb())))
            .collect::<HashMap<String, Color>>();
    }

    pub fn convert_all_to_hsl(&mut self) {
        self.colors = self
            .colors
            .iter()
            .map(|(name, color)| (name.clone(), Color::Hsl(color.to_hsl())))
            .collect::<HashMap<String, Color>>();
    }
}

impl Palette {
    fn convert_all<F>(&mut self, f: F)
    where
        F: Fn(&Color) -> Color,
    {
        for (_name, color) in self.colors.iter_mut() {
            *color = f(color);
        }
    }
}

