use serde::{Deserialize, Serialize};

use crate::models::rgb::Rgb;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct Hsl {
    h: f32, // 360e
    s: f32, // 0.0-1.0
    l: f32, // 0.0-1.0
}

impl Hsl {
    pub fn new(h: f32, s: f32, l: f32) -> Self {
        let h = h.clamp(0.0, 360.0);
        let s = s.clamp(0.0, 1.0);
        let l = l.clamp(0.0, 1.0);
        Self { h, s, l }
    }
    pub fn get_hue(&self) -> f32 {
        self.h
    }
    pub fn set_hue(&mut self, h_new: f32) {
        self.h = h_new.clamp(0.0, 360.0);
    }
    pub fn get_saturation(&self) -> f32 {
        self.s
    }
    pub fn set_saturation(&mut self, s_new: f32) {
        self.s = s_new.clamp(0.0, 1.0);
    }
    pub fn get_lightness(&self) -> f32 {
        self.l
    }
    pub fn set_lightness(&mut self, l_new: f32) {
        self.l = l_new.clamp(0.0, 1.0);
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

        fn round_to_nearest_hsl(f: f32) -> f32 {
            const PRECISION_SCALE: f32 = 100.0;

            ((f * PRECISION_SCALE).round()) / PRECISION_SCALE
        }

        // Normalize
        let r = rgb.get_red() as f32 / RGB_MAX;
        let g = rgb.get_green() as f32 / RGB_MAX;
        let b = rgb.get_blue() as f32 / RGB_MAX;

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
        let s_final = round_to_nearest_hsl(s);
        let l_final = round_to_nearest_hsl(l);

        Hsl {
            h: h_final,
            s: s_final,
            l: l_final,
        }
    }
}
