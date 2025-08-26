use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct Hsl {
    pub h: f32, // 360, TODO: make sure to clamp this when making a new HSL item
    pub s: f32, // 0.0-1.0
    pub l: f32, // 0.0-1.0
}

impl Hsl {
    fn new(h : f32, s : f32, l : f32,) -> Self {
        Self { h, s, l }
   }
}
