use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Palette {
   pub name: String,
   pub colors: HashMap<String, String>,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Palettes {
    #[serde(rename= "palette")]
   pub palette_collection: Vec<Palette>
}
