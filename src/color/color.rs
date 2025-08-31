use serde::{Deserialize, Serialize};

use crate::color::hsl::Hsl;
use crate::color::rgb::Rgb;

#[derive(Debug, Deserialize, Serialize)]
pub enum Color {
    Rgb(Rgb),
    Hsl(Hsl),
}

impl Color {
    pub fn to_hsl(&self) -> Hsl {
        match self {
            Color::Rgb(rgb) => (*rgb).into(),
            Color::Hsl(hsl) => *hsl,
        }
    }
    pub fn to_rgb(&self) -> Rgb {
        match self {
            Color::Rgb(rgb) => *rgb,
            Color::Hsl(hsl) => (*hsl).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A single test case linking multiple color representations
    struct ColorCase {
        rgb: Rgb,
        hsl: Hsl,
        // future formats can be added here, e.g., cmyk: Cmyk
    }

    #[test]
    fn test_rgb_hsl_roundtrip() {
        let cases = [
            ColorCase {
                rgb: Rgb { r: 0, g: 0, b: 0 },
                hsl: Hsl {
                    h: 0.0,
                    s: 0.0,
                    l: 0.0,
                },
            },
            ColorCase {
                rgb: Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                },
                hsl: Hsl {
                    h: 0.0,
                    s: 0.0,
                    l: 100.0,
                },
            },
            ColorCase {
                rgb: Rgb {
                    r: 38,
                    g: 138,
                    b: 210,
                },
                hsl: Hsl {
                    h: 205.0,
                    s: 69.4,
                    l: 48.6,
                },
            },
            ColorCase {
                rgb: Rgb {
                    r: 128,
                    g: 128,
                    b: 128,
                },
                hsl: Hsl {
                    h: 0.0,
                    s: 0.0,
                    l: 50.2,
                },
            },
            ColorCase {
                rgb: Rgb { r: 255, g: 0, b: 0 },
                hsl: Hsl {
                    h: 0.0,
                    s: 100.0,
                    l: 50.0,
                },
            },
            ColorCase {
                rgb: Rgb { r: 0, g: 255, b: 0 },
                hsl: Hsl {
                    h: 120.0,
                    s: 100.0,
                    l: 50.0,
                },
            },
            ColorCase {
                rgb: Rgb { r: 0, g: 0, b: 255 },
                hsl: Hsl {
                    h: 240.0,
                    s: 100.0,
                    l: 50.0,
                },
            },
        ];

        for case in cases.iter() {
            // Test RGB -> HSL
            let computed_hsl: Hsl = case.rgb.into();
            assert!(
                (computed_hsl.h - case.hsl.h).abs() < 0.5,
                "Hue mismatch: {:?} vs {:?}",
                computed_hsl,
                case.hsl
            );
            assert!(
                (computed_hsl.s - case.hsl.s).abs() < 0.5,
                "Saturation mismatch: {:?} vs {:?}",
                computed_hsl,
                case.hsl
            );
            assert!(
                (computed_hsl.l - case.hsl.l).abs() < 0.5,
                "Lightness mismatch: {:?} vs {:?}",
                computed_hsl,
                case.hsl
            );

            // Test HSL -> RGB
            let computed_rgb: Rgb = case.hsl.into();
            assert_eq!(computed_rgb, case.rgb);
        }
    }
}
