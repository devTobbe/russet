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
