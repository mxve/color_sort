use crate::constants::{
    EPS, HUE_BLUE_END, HUE_CYAN_END, HUE_GREEN_END, HUE_RED_END, HUE_RED_START, HUE_YELLOW_END,
};
use crate::error::ParseError;

pub fn is_hue_in_range(hue: f32, start: f32, end: f32) -> bool {
    if start > end {
        hue >= start || hue < end
    } else {
        hue >= start && hue < end
    }
}

pub fn is_red_hue(hue: f32) -> bool {
    is_hue_in_range(hue, HUE_RED_START, HUE_RED_END)
}

pub fn is_green_hue(hue: f32) -> bool {
    is_hue_in_range(hue, HUE_YELLOW_END, HUE_GREEN_END)
}

pub fn is_blue_hue(hue: f32) -> bool {
    is_hue_in_range(hue, HUE_CYAN_END, HUE_BLUE_END)
}

pub fn parse_hex_to_rgba(hex: &str) -> Result<(u8, u8, u8, f32), ParseError> {
    let hex = hex.trim_start_matches('#');

    match hex.len() {
        3 => {
            let r =
                u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|_| ParseError::InvalidHex)?;
            let g =
                u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|_| ParseError::InvalidHex)?;
            let b =
                u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|_| ParseError::InvalidHex)?;
            Ok((r, g, b, 1.0))
        }
        4 => {
            let r =
                u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|_| ParseError::InvalidHex)?;
            let g =
                u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|_| ParseError::InvalidHex)?;
            let b =
                u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|_| ParseError::InvalidHex)?;
            let a = u8::from_str_radix(&hex[3..4].repeat(2), 16)
                .map_err(|_| ParseError::InvalidHex)? as f32
                / 255.0;
            Ok((r, g, b, a))
        }
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ParseError::InvalidHex)?;
            let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ParseError::InvalidHex)?;
            let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ParseError::InvalidHex)?;
            Ok((r, g, b, 1.0))
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ParseError::InvalidHex)?;
            let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ParseError::InvalidHex)?;
            let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ParseError::InvalidHex)?;
            let a = u8::from_str_radix(&hex[6..8], 16).map_err(|_| ParseError::InvalidHex)? as f32
                / 255.0;
            Ok((r, g, b, a))
        }
        _ => Err(ParseError::InvalidHex),
    }
}

pub fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let lightness = (max + min) / 2.0;

    if delta < EPS {
        return (0.0, 0.0, lightness);
    }

    let saturation = delta / (1.0 - (2.0 * lightness - 1.0).abs());

    let hue = match max {
        _ if max == r => (g - b) / delta + if g < b { 6.0 } else { 0.0 },
        _ if max == g => (b - r) / delta + 2.0,
        _ => (r - g) / delta + 4.0,
    };

    (hue * 60.0, saturation, lightness)
}

pub fn format_alpha(a: f32) -> String {
    if (a - 1.0).abs() < EPS {
        "1".to_string()
    } else if a < EPS {
        "0".to_string()
    } else {
        format!("{:.2}", a)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}
