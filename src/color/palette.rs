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

    fn convert_all<F>(&mut self, f: F)
    where
        F: Fn(&Color) -> Color,
    {
        self.colors = self
            .colors
            .iter()
            .map(|(name, color)| (name.clone(), f(color)))
            .collect::<HashMap<String, Color>>();
    }

    // TODO: Refactor this to be more easily extensible. A lot of reused code
    pub fn convert_all_to_rgb(&mut self) {
        self.convert_all(|c| c.to_rgb());
    }

    pub fn convert_all_to_hsl(&mut self) {
        self.convert_all(|c| c.to_hsl());
    }
}

impl Palette {

}

