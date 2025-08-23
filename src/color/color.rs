use std::error::Error;

use crate::color::hsl::Hsl;
use crate::color::rgb::Rgb;

#[derive(Debug)]
pub enum Color {
    Rgb(Rgb),
    Hsl(Hsl),
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
    fn to_hsl(&self) -> Color {
        match self {
            Color::Rgb(rgb) => todo!(),
            Color::Hsl(hsl) => Color::Hsl(*hsl),
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
    fn grayscale_should_ignore_hue() {
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
}
