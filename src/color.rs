use std::{fmt::Display, str::FromStr, num::ParseIntError};

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    transparent: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum ColorError {
    #[error("Invalid color format. Color should be an X11 color name, a hex color, or \"None\".")]
    InvalidColorFormat,

    #[error("Could not parse hex color string.")]
    ParseIntError(#[from] ParseIntError),
}

pub type RgbTriple = (u8, u8, u8);

impl Color {
    pub fn new(color: Option<RgbTriple>) -> Self {
        if let Some((r, g, b)) = color {
            Self { r, g, b, transparent: false }
        } else {
            Self { r: 0, g: 0, b: 0, transparent: true }
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.transparent {
            write!(f, "\x1b[48;2;{};{};{}m  \x1b[0m", self.r, self.g, self.b)
        } else {
            write!(f, "  ")
        }
    }
}

impl FromStr for Color {
    type Err = ColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        // X11 color
        if let Ok([r, g, b]) = color_name::Color::val().by_string(s.to_owned()) {
            return Ok(Color::new(Some((r, g, b))));
        }

        // Transparent
        if s.to_lowercase() == "none" {
            return Ok(Color::new(None));
        }

        // Hex
        if !s.starts_with("#") || s.len() != 7 {
            return Err(ColorError::InvalidColorFormat);
        }

        let rgb = u32::from_str_radix(&s[1..], 16)?;
        let [_, r, g, b] = rgb.to_be_bytes();
        
        Ok(Color::new(Some((r, g, b))))
    }
}