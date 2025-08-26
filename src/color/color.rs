use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::color::hsl::Hsl;
use crate::color::rgb::Rgb;

#[derive(Debug, Deserialize, Serialize)]
pub enum Color {
    Rgb(Rgb),
    Hsl(Hsl),
}

// Helper
fn round_to_nearest_hsl(f: f32) -> f32 {
    ((f * 10.0).round()) / 10.0
}

impl Color {
    fn to_rgb(&self) -> Result<Color, Box<dyn Error>> {
        match self {
            Color::Rgb(rgb) => Ok(Color::Rgb(*rgb)),

            // https://www.rapidtables.com/convert/color/hsl-to-rgb.html
            // Used this as a reference for the equation
            Color::Hsl(hsl) => {
                let h = hsl.h;
                let s = hsl.s;
                let l = hsl.l;

                // Normalize, h is already fine and does not need to be normalized
                let s = s / 100.0;
                let l = l / 100.0;

                let c = (1.0 - (2.0 * l - 1.0).abs()) * s;

                let x = c * (1.0 - ((h / 60.0).rem_euclid(2.0) - 1.0).abs());

                let m = l - c / 2.0;

                let (r1, g1, b1) = match h {
                    0.0..=59.0 => (c, x, 0.0),
                    60.0..=119.0 => (x, c, 0.0),
                    120.0..=179.0 => (0.0, c, x),
                    180.0..=239.0 => (0.0, x, c),
                    240.0..=299.0 => (x, 0.0, c),
                    300.0..=359.0 => (c, 0.0, x),
                    _ => return Err("Hue out of range 0–359".into()),
                };

                // Construct the final RGB
                let r = ((r1 + m) * 255.0).round() as u8;
                let g = ((g1 + m) * 255.0).round() as u8;
                let b = ((b1 + m) * 255.0).round() as u8;

                let final_rgb = Rgb { r, g, b };

                Ok(Color::Rgb(final_rgb))
            }
        }
    }
    fn to_hsl(&self) -> Result<Color, Box<dyn Error>> {
        match self {
            Color::Rgb(rgb) => {
                let r = rgb.r;
                let g = rgb.g;
                let b = rgb.b;

                // Casting
                let r_f = r as f32;
                let g_f = g as f32;
                let b_f = b as f32;

                // Normalize
                let r_f = r_f / 255.0;
                let g_f = g_f / 255.0;
                let b_f = b_f / 255.0;

                // Get CMax
                let mut cmax = r_f;
                if cmax < g_f {
                    cmax = g_f;
                }
                if cmax < b_f {
                    cmax = b_f;
                }

                // Get CMin
                let mut cmin = r_f;
                if cmin > g_f {
                    cmin = g_f;
                }
                if cmin > b_f {
                    cmin = g_f;
                }

                let delta = cmax - cmin;

                println!("cmax: {cmax}");
                println!("cmin: {cmin}");
                println!("DELTA: {delta}");

                // HUE
                let mut h: f32 = if delta == 0.0 {
                    0.0
                } else if cmax == r_f {
                    (g_f - b_f) / delta
                } else if cmax == g_f {
                    ((b_f - r_f) / delta) + 2.0
                } else {
                    ((r_f - g_f) / delta) + 4.0
                };

                h = h.rem_euclid(6.0);
                h *= 60.0;

                // LIGHTNESS / VALUE
                let l = (cmax + cmin) / 2.0;

                // SATURATION
                let s = if delta == 0.0 {
                    0.0
                } else {
                    delta / (1.0 - (2.0 * l - 1.0).abs())
                };

                println!("h: {h}");
                let h_final = h.round();
                let s_final = round_to_nearest_hsl(s * 100.0);
                let l_final = round_to_nearest_hsl(l * 100.0);

                let final_hsl = Hsl {
                    h: h_final,
                    s: s_final,
                    l: l_final,
                };
                Ok(Color::Hsl(final_hsl))
            }
            Color::Hsl(hsl) => Ok(Color::Hsl(*hsl)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_black_to_rgb_zero() {
        let hsl = Hsl {
            h: 0.0,
            s: 0.0,
            l: 0.0,
        };

        let hsl_color = Color::Hsl(hsl);

        let black_rgb = Rgb { r: 0, g: 0, b: 0 };

        let result_color = hsl_color.to_rgb().unwrap();

        let result_rgb = match result_color {
            Color::Rgb(rgb) => rgb,
            _ => panic!("Expected RGB color"),
        };

        assert_eq!(black_rgb, result_rgb);
    }

    #[test]
    fn converts_sample_hsl_to_rgb() {
        let hsl = Hsl {
            h: 205.0,
            s: 69.4,
            l: 48.6,
        };

        let hsl_color = Color::Hsl(hsl);

        let sample_rgb = Rgb {
            r: 38,
            g: 138,
            b: 210,
        };

        let result_color = hsl_color.to_rgb().unwrap();

        let result_rgb = match result_color {
            Color::Rgb(rgb) => rgb,
            _ => panic!("Expected RGB color"),
        };

        assert_eq!(sample_rgb, result_rgb);
    }
    /// Helper to extract the RGB struct from a Color::Rgb
    fn extract_rgb(color: Color) -> Rgb {
        match color {
            Color::Rgb(rgb) => rgb,
            _ => panic!("Expected Color::Rgb variant"),
        }
    }

    #[test]
    fn black_should_convert_to_rgb_zero() {
        let hsl = Hsl {
            h: 0.0,
            s: 0.0,
            l: 0.0,
        };
        let hsl_color = Color::Hsl(hsl);

        let expected = Rgb { r: 0, g: 0, b: 0 };
        let result_rgb = extract_rgb(hsl_color.to_rgb().unwrap());

        assert_eq!(expected, result_rgb);
    }

    #[test]
    fn sample_hsl_should_convert_to_rgb() {
        let hsl = Hsl {
            h: 205.0,
            s: 69.4,
            l: 48.6,
        };
        let hsl_color = Color::Hsl(hsl);

        let expected = Rgb {
            r: 38,
            g: 138,
            b: 210,
        };
        let result_rgb = extract_rgb(hsl_color.to_rgb().unwrap());

        assert_eq!(expected, result_rgb);
    }

    #[test]
    fn white_should_convert_to_rgb_255() {
        let hsl = Hsl {
            h: 0.0,
            s: 0.0,
            l: 100.0,
        };
        let hsl_color = Color::Hsl(hsl);

        let expected = Rgb {
            r: 255,
            g: 255,
            b: 255,
        };
        let result_rgb = extract_rgb(hsl_color.to_rgb().unwrap());

        assert_eq!(expected, result_rgb);
    }

    #[test]
    fn primary_colors() {
        let red = Hsl {
            h: 0.0,
            s: 100.0,
            l: 50.0,
        };
        let green = Hsl {
            h: 120.0,
            s: 100.0,
            l: 50.0,
        };
        let blue = Hsl {
            h: 240.0,
            s: 100.0,
            l: 50.0,
        };

        assert_eq!(
            Rgb { r: 255, g: 0, b: 0 },
            extract_rgb(Color::Hsl(red).to_rgb().unwrap())
        );
        assert_eq!(
            Rgb { r: 0, g: 255, b: 0 },
            extract_rgb(Color::Hsl(green).to_rgb().unwrap())
        );
        assert_eq!(
            Rgb { r: 0, g: 0, b: 255 },
            extract_rgb(Color::Hsl(blue).to_rgb().unwrap())
        );
    }

    #[test]
    fn grayscale_should_ignore_hue_rgb() {
        let gray = Hsl {
            h: 50.0,
            s: 0.0,
            l: 50.0,
        };
        let expected = Rgb {
            r: 128,
            g: 128,
            b: 128,
        };

        let result_rgb = extract_rgb(Color::Hsl(gray).to_rgb().unwrap());
        assert_eq!(expected, result_rgb);
    }

    /// Helper to extract the HSL struct from a Color::Hsl
    fn extract_hsl(color: Color) -> Hsl {
        match color {
            Color::Hsl(hsl) => hsl,
            _ => panic!("Expected Color::Hsl variant"),
        }
    }

    #[test]
    fn black_should_convert_to_hsl_zero() {
        let rgb = Rgb { r: 0, g: 0, b: 0 };
        let rgb_color = Color::Rgb(rgb);

        let expected = Hsl {
            h: 0.0,
            s: 0.0,
            l: 0.0,
        };
        let result_hsl = extract_hsl(rgb_color.to_hsl().unwrap());

        assert_eq!(expected, result_hsl);
    }

    #[test]
    fn white_should_convert_to_hsl_100() {
        let rgb = Rgb {
            r: 255,
            g: 255,
            b: 255,
        };
        let rgb_color = Color::Rgb(rgb);

        let expected = Hsl {
            h: 0.0,
            s: 0.0,
            l: 100.0,
        };
        let result_hsl = extract_hsl(rgb_color.to_hsl().unwrap());

        assert_eq!(expected, result_hsl);
    }

    #[test]
    fn sample_rgb_should_convert_to_hsl() {
        let rgb = Rgb {
            r: 38,
            g: 138,
            b: 210,
        };
        let rgb_color = Color::Rgb(rgb);

        let expected = Hsl {
            h: 205.0,
            s: 69.4,
            l: 48.6,
        };
        let result_hsl = extract_hsl(rgb_color.to_hsl().unwrap());

        assert_eq!(expected, result_hsl);
    }

    #[test]
    fn primary_colors_to_hsl() {
        let red = Rgb { r: 255, g: 0, b: 0 };
        let green = Rgb { r: 0, g: 255, b: 0 };
        let blue = Rgb { r: 0, g: 0, b: 255 };

        assert_eq!(
            Hsl {
                h: 0.0,
                s: 100.0,
                l: 50.0
            },
            extract_hsl(Color::Rgb(red).to_hsl().unwrap())
        );
        assert_eq!(
            Hsl {
                h: 120.0,
                s: 100.0,
                l: 50.0
            },
            extract_hsl(Color::Rgb(green).to_hsl().unwrap())
        );
        assert_eq!(
            Hsl {
                h: 240.0,
                s: 100.0,
                l: 50.0
            },
            extract_hsl(Color::Rgb(blue).to_hsl().unwrap())
        );
    }

    #[test]
    fn grayscale_should_ignore_hue_hsl() {
        let gray = Rgb {
            r: 128,
            g: 128,
            b: 128,
        };
        let expected = Hsl {
            h: 0.0,
            s: 0.0,
            l: 50.2, // Depending on your rounding, it might be exactly 50.0
        };

        let result_hsl = extract_hsl(Color::Rgb(gray).to_hsl().unwrap());
        assert_eq!(expected, result_hsl);
    }
}
