use hex_color::{HexColor, ParseHexColorError};

/// A color and opacity
#[derive(Debug, Clone, Copy)]
pub struct Color(HexColor);

impl Color {
    /// Define a new color
    #[inline]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(HexColor { r, g, b, a })
    }

    /// Define a new color from a hex string
    #[inline]
    pub fn from_hex(hex: &str) -> Result<Self, ParseHexColorError> {
        Ok(Self(HexColor::parse(hex)?))
    }

    #[inline]
    pub(crate) fn to_svg_color(self) -> String {
        format!("rgb({},{},{})", self.0.r, self.0.g, self.0.b,)
    }

    #[inline]
    pub(crate) fn to_opacity_percent(self) -> f64 {
        f64::from(self.0.a) / 255.0
    }
}

impl TryFrom<&str> for Color {
    type Error = ParseHexColorError;

    fn try_from(hex_str: &str) -> Result<Self, Self::Error> {
        Ok(Self(HexColor::parse(hex_str)?))
    }
}
