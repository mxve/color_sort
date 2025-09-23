#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseError {
    InvalidHex,
    InvalidRgb,
    InvalidHsl,
    InvalidFormat,
    InvalidAlpha,
    InvalidValue,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidHex => write!(f, "Invalid hexadecimal color format"),
            ParseError::InvalidRgb => write!(f, "Invalid RGB values"),
            ParseError::InvalidHsl => write!(f, "Invalid HSL values"),
            ParseError::InvalidFormat => write!(f, "Invalid color format"),
            ParseError::InvalidAlpha => write!(f, "Invalid alpha value"),
            ParseError::InvalidValue => write!(f, "Invalid color value"),
        }
    }
}

impl std::error::Error for ParseError {}
