use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::color::color::Color;

#[derive(Debug, Serialize, Deserialize)]
pub struct Palette {
    pub name: String,
    pub colors: HashMap<String, Color>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Palettes {
    #[serde(rename = "palette")]
    pub palette_collection: Vec<Palette>,
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

