use color::{AlphaColor, Srgb};

pub fn hex_to_alpha_color(hex: &str) -> Result<AlphaColor<Srgb>, String> {
    let hex = hex.trim_start_matches('#');

    let (r, g, b, a) = match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid red component")?;
            let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid green component")?;
            let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid blue component")?;
            (r, g, b, 255)
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid red component")?;
            let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid green component")?;
            let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid blue component")?;
            let a = u8::from_str_radix(&hex[6..8], 16).map_err(|_| "Invalid alpha component")?;
            (r, g, b, a)
        }
        _ => {
            return Err(format!(
                "Invalid hex color length: expected 6 or 8 characters, got {}",
                hex.len()
            ));
        }
    };

    Ok(AlphaColor::from_rgba8(r, g, b, a))
}
