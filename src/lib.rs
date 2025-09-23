mod constants;
mod error;
mod types;
mod utils;

use crate::constants::{EPS, PRECISION, PRECISION_I32};
use crate::utils::{
    format_alpha, is_blue_hue, is_green_hue, is_red_hue, parse_hex_to_rgba, rgb_to_hsl,
};
pub use error::ParseError;
pub use types::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ColorFormat {
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, f32),
    Hsl(f32, f32, f32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub original_input: String,
    pub format: ColorFormat,
}

fn normalize_alpha(alpha: f32) -> Result<f32, ParseError> {
    if !(-EPS..=1.0 + EPS).contains(&alpha) {
        Err(ParseError::InvalidAlpha)
    } else {
        Ok(alpha.clamp(0.0, 1.0))
    }
}

impl ColorFormat {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        ColorFormat::Rgb(r, g, b)
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        ColorFormat::Rgba(r, g, b, a.clamp(0.0, 1.0))
    }

    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        let h = h.rem_euclid(360.0);
        let s = s.clamp(0.0, 1.0);
        let l = l.clamp(0.0, 1.0);
        ColorFormat::Hsl(h, s, l)
    }

    pub fn from_hex(hex: &str) -> Result<Self, ParseError> {
        let (r, g, b, a) = parse_hex_to_rgba(hex)?;
        if a < 1.0 {
            Ok(ColorFormat::Rgba(r, g, b, a))
        } else {
            Ok(ColorFormat::Rgb(r, g, b))
        }
    }

    pub fn to_rgba(&self) -> (u8, u8, u8, f32) {
        match self {
            ColorFormat::Rgb(r, g, b) => (*r, *g, *b, 1.0),
            ColorFormat::Rgba(r, g, b, a) => (*r, *g, *b, *a),
            ColorFormat::Hsl(h, s, l) => {
                let (r, g, b) = Color::hsl_to_rgb(*h, *s, *l);
                (r, g, b, 1.0)
            }
        }
    }

    fn to_hsl(&self) -> (f32, f32, f32, f32) {
        match self {
            ColorFormat::Hsl(h, s, l) => (*h, *s, *l, 1.0),
            ColorFormat::Rgb(r, g, b) => {
                let (h, s, l) = rgb_to_hsl(*r, *g, *b);
                (h, s, l, 1.0)
            }
            ColorFormat::Rgba(r, g, b, a) => {
                let (h, s, l) = rgb_to_hsl(*r, *g, *b);
                (h, s, l, *a)
            }
        }
    }

    pub fn matches_filter(&self, filter: &FilterOptions) -> bool {
        let (hue, saturation, _lightness, alpha) = self.to_hsl();
        let is_grayscale = saturation < 0.1 + EPS;

        if let Some(opacity_filter) = filter.opacity {
            let is_opaque = alpha >= filter.transparency_threshold;
            match opacity_filter {
                OpacityFilter::Opaque if !is_opaque => return false,
                OpacityFilter::Transparent if is_opaque => return false,
                _ => (),
            }
        }

        if let Some(saturation_filter) = filter.saturation {
            match saturation_filter {
                SaturationFilter::Grayscale if !is_grayscale => return false,
                SaturationFilter::Saturated if is_grayscale => return false,
                _ => (),
            }
        }

        if let Some(hue_filters) = &filter.hues
            && !hue_filters.is_empty()
        {
            let normalized_hue = hue.rem_euclid(360.0);

            let is_red = is_red_hue(normalized_hue);
            let is_green = is_green_hue(normalized_hue);
            let is_blue = is_blue_hue(normalized_hue);

            let matches_hue = hue_filters.iter().any(|&hue_filter| match hue_filter {
                HueFilter::Red => is_red,
                HueFilter::Green => is_green,
                HueFilter::Blue => is_blue,
            });

            if !is_grayscale && !matches_hue {
                return false;
            }
        }

        true
    }

    pub fn to_sortable_value(&self) -> (i32, i32, i32) {
        self.to_sortable_value_by_option(SortOption::Spectrum)
    }

    pub fn to_sortable_value_by_option(&self, sort_option: SortOption) -> (i32, i32, i32) {
        let (hue, _saturation, lightness, alpha) = self.to_hsl();

        let luminance_key = lightness.mul_add(PRECISION, 0.0).round() as i32;
        let opacity_key = alpha.mul_add(PRECISION, 0.0).round() as i32;

        match sort_option {
            SortOption::Spectrum => {
                let hue_order = Self::hue_to_order(hue);
                (
                    hue_order,
                    PRECISION_I32 - luminance_key,
                    PRECISION_I32 - opacity_key,
                )
            }
            SortOption::Luminance => (PRECISION_I32 - luminance_key, 0, 0),
            SortOption::Opacity => (PRECISION_I32 - opacity_key, 0, 0),
        }
    }

    fn hue_to_order(hue: f32) -> i32 {
        let normalized_hue = hue.rem_euclid(360.0);
        (normalized_hue * 1000.0).round() as i32
    }
}

impl PartialOrd for ColorFormat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ColorFormat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_sortable_value().cmp(&other.to_sortable_value())
    }
}

impl Eq for ColorFormat {}

impl Color {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let trimmed = input.trim();

        let format = if trimmed.starts_with('#') {
            Self::parse_hex(trimmed)?
        } else if trimmed.starts_with("rgb(") || trimmed.starts_with("rgba(") {
            Self::parse_rgb(trimmed)?
        } else if trimmed.starts_with("hsl(") {
            Self::parse_hsl(trimmed)?
        } else {
            return Err(ParseError::InvalidFormat);
        };

        Ok(Color {
            original_input: input.to_string(),
            format,
        })
    }

    fn parse_hex(hex: &str) -> Result<ColorFormat, ParseError> {
        ColorFormat::from_hex(hex)
    }

    fn parse_rgb(rgb: &str) -> Result<ColorFormat, ParseError> {
        let is_rgba = rgb.starts_with("rgba(");
        let content = if is_rgba {
            rgb.strip_prefix("rgba(").and_then(|s| s.strip_suffix(')'))
        } else {
            rgb.strip_prefix("rgb(").and_then(|s| s.strip_suffix(')'))
        }
        .ok_or(ParseError::InvalidRgb)?;

        let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();

        let expected_len = if is_rgba { 4 } else { 3 };
        if parts.len() != expected_len {
            return Err(ParseError::InvalidRgb);
        }

        let r = parts[0].parse::<u8>().map_err(|_| ParseError::InvalidRgb)?;
        let g = parts[1].parse::<u8>().map_err(|_| ParseError::InvalidRgb)?;
        let b = parts[2].parse::<u8>().map_err(|_| ParseError::InvalidRgb)?;

        if is_rgba {
            let a = parts[3]
                .parse::<f32>()
                .map_err(|_| ParseError::InvalidAlpha)?;
            let normalized_a = normalize_alpha(a)?;
            Ok(ColorFormat::Rgba(r, g, b, normalized_a))
        } else {
            Ok(ColorFormat::Rgb(r, g, b))
        }
    }

    fn parse_hsl(hsl: &str) -> Result<ColorFormat, ParseError> {
        let content = hsl
            .strip_prefix("hsl(")
            .and_then(|s| s.strip_suffix(')'))
            .ok_or(ParseError::InvalidHsl)?;

        let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();

        if parts.len() != 3 {
            return Err(ParseError::InvalidHsl);
        }

        let h = parts[0]
            .trim_end_matches("deg")
            .parse::<f32>()
            .map_err(|_| ParseError::InvalidHsl)?;
        let s = parts[1]
            .trim_end_matches('%')
            .parse::<f32>()
            .map_err(|_| ParseError::InvalidHsl)?
            / 100.0;
        let l = parts[2]
            .trim_end_matches('%')
            .parse::<f32>()
            .map_err(|_| ParseError::InvalidHsl)?
            / 100.0;

        if !(-EPS..=1.0 + EPS).contains(&s) || !(-EPS..=1.0 + EPS).contains(&l) {
            return Err(ParseError::InvalidHsl);
        }

        Ok(ColorFormat::Hsl(h.rem_euclid(360.0), s, l))
    }

    pub fn to_rgba(&self) -> (u8, u8, u8, f32) {
        self.format.to_rgba()
    }

    pub fn to_format(&self, target: TargetFormat) -> String {
        let (r, g, b, a) = self.to_rgba();

        match target {
            TargetFormat::Hex => {
                if a < 1.0 {
                    format!(
                        "#{:02x}{:02x}{:02x}{:02x}",
                        r,
                        g,
                        b,
                        (a * 255.0).round() as u8
                    )
                } else {
                    format!("#{:02x}{:02x}{:02x}", r, g, b)
                }
            }
            TargetFormat::Rgb => format!("rgb({}, {}, {})", r, g, b),
            TargetFormat::Rgba => {
                let alpha = format_alpha(a);
                format!("rgba({}, {}, {}, {})", r, g, b, alpha)
            }
            TargetFormat::Hsl => {
                let (h, s, l, _) = self.format.to_hsl();
                format!("hsl({:.0}, {:.0}%, {:.0}%)", h, s * 100.0, l * 100.0)
            }
            TargetFormat::Hsla => {
                let (h, s, l, _) = self.format.to_hsl();
                let alpha = format_alpha(a);
                format!(
                    "hsla({:.0}, {:.0}%, {:.0}%, {})",
                    h,
                    s * 100.0,
                    l * 100.0,
                    alpha
                )
            }
        }
    }

    fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r_prime, g_prime, b_prime) = match h {
            h if h < 60.0 => (c, x, 0.0),
            h if h < 120.0 => (x, c, 0.0),
            h if h < 180.0 => (0.0, c, x),
            h if h < 240.0 => (0.0, x, c),
            h if h < 300.0 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        let r = ((r_prime + m) * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = ((g_prime + m) * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = ((b_prime + m) * 255.0).round().clamp(0.0, 255.0) as u8;

        (r, g, b)
    }

    pub fn matches_filter(&self, filter: &FilterOptions) -> bool {
        self.format.matches_filter(filter)
    }
}

pub fn parse_colors(color_strings: &[String]) -> Result<Vec<Color>, ParseError> {
    color_strings.iter().map(|s| Color::parse(s)).collect()
}

pub fn convert_colors(colors: &[Color], target_format: TargetFormat) -> Vec<String> {
    colors
        .iter()
        .map(|color| color.to_format(target_format))
        .collect()
}

pub fn filter_colors(colors: &[Color], filter: &FilterOptions) -> Vec<Color> {
    colors
        .iter()
        .filter(|color| color.matches_filter(filter))
        .cloned()
        .collect()
}

pub fn sort_colors(colors: &mut [Color]) {
    colors.sort_by_key(|color| color.format.to_sortable_value());
}

pub fn sort_colors_by(colors: &mut [Color], sort_by: SortOption) {
    colors.sort_by_key(|color| color.format.to_sortable_value_by_option(sort_by));
}

pub fn sort_color_strings(
    color_strings: &mut [String],
    sort_by: SortOption,
) -> Result<(), ParseError> {
    let mut parsed_colors: Vec<_> = color_strings
        .iter()
        .map(|s| Color::parse(s).map(|color| (s.clone(), color)))
        .collect::<Result<Vec<_>, ParseError>>()?;

    parsed_colors.sort_by_key(|(_, color)| color.format.to_sortable_value_by_option(sort_by));

    for (i, (original, _)) in parsed_colors.into_iter().enumerate() {
        color_strings[i] = original;
    }

    Ok(())
}
