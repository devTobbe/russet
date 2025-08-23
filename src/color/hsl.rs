#[derive(Debug, Clone, Copy)]
pub struct Hsl {
    pub h: f32, // 360, TODO: make sure to clamp this when making a new HSL item
    pub s: f32, // 0.0-1.0
    pub l: f32, // 0.0-1.0
}
