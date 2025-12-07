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
        self.0.a as f64 / 255.0 * 100.0
    }
}
