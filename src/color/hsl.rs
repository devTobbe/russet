use serde::{Deserialize, Serialize};

use crate::color::rgb::Rgb;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct Hsl {
    pub h: f32, // 360, TODO: make sure to clamp this when making a new HSL item
    pub s: f32, // 0.0-1.0
    pub l: f32, // 0.0-1.0
}

impl Hsl {
    fn new(h: f32, s: f32, l: f32) -> Self {
        Self { h, s, l }
    }
}

impl From<Rgb> for Hsl {
    fn from(rgb: Rgb) -> Self {
        const RGB_MAX: f32 = 255.0;
        const GREEN_SECTOR_OFFSET: f32 = 2.0;
        const BLUE_SECTOR_OFFSET: f32 = 4.0;
        const SECTOR_SIZE: f32 = 60.0;
        const HUE_SECTOR_COUNT: f32 = 6.0;
        const MAX_CHROMA: f32 = 1.0;
        const PERCENT_SCALE: f32 = 100.0;

        fn round_to_nearest_hsl(f: f32) -> f32 {
            const PRECISION_SCALE: f32 = 10.0;

            ((f * PRECISION_SCALE).round()) / PRECISION_SCALE
        }

        // Normalize
        let r = rgb.r as f32 / RGB_MAX;
        let g = rgb.g as f32 / RGB_MAX;
        let b = rgb.b as f32 / RGB_MAX;

        // Get CMax and CMin
        let cmax = r.max(g).max(b);
        let cmin = r.min(g).min(b);

        let delta = cmax - cmin;

        // HUE
        let mut h: f32 = if delta == 0.0 {
            0.0
        } else if cmax == r {
            (g - b) / delta
        } else if cmax == g {
            ((b - r) / delta) + GREEN_SECTOR_OFFSET
        } else {
            ((r - g) / delta) + BLUE_SECTOR_OFFSET
        };

        h = h.rem_euclid(HUE_SECTOR_COUNT);
        h *= SECTOR_SIZE;

        // LIGHTNESS / VALUE
        let l = (cmax + cmin) / 2.0;

        // SATURATION
        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (MAX_CHROMA - (2.0 * l - MAX_CHROMA).abs())
        };

        let h_final = h.round();
        let s_final = round_to_nearest_hsl(s * PERCENT_SCALE);
        let l_final = round_to_nearest_hsl(l * PERCENT_SCALE);

        Hsl {
            h: h_final,
            s: s_final,
            l: l_final,
        }
    }
}
