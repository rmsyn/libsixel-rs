//! Types and functions for Sixel colors.

/// Represents a Sixel color name.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ColorName {
    /// Reserved color variant for undefined RGB values.
    #[default]
    Reserved,
    /// Unnamed [Color] variant.
    Rgb(u32),
    /// Named [Color] variant.
    Variant(ColorVariant),
}

impl ColorName {
    /// Creates a new [ColorName].
    pub const fn new() -> ColorName {
        ColorName::Reserved
    }
}

/// Represents a named [Color] variant.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ColorVariant {
    #[default]
    Black = 0x000000,
}

impl From<ColorName> for u32 {
    fn from(val: ColorName) -> Self {
        match val {
            ColorName::Reserved => 0,
            ColorName::Rgb(color) => color,
            ColorName::Variant(color) => color as u32,
        }
    }
}

impl From<&str> for ColorName {
    fn from(val: &str) -> Self {
        if val.starts_with("rgb:") && val.len() >= 10 {
            if let Ok(rgb) = u32::from_str_radix(&val[4..].to_lowercase()[..6], 16) {
                Self::Rgb(rgb)
            } else {
                Self::Reserved
            }
        } else if val.starts_with('#') && val.len() >= 7 {
            if let Ok(rgb) = u32::from_str_radix(&val[1..].to_lowercase()[..6], 16) {
                Self::Rgb(rgb)
            } else {
                Self::Reserved
            }
        } else {
            match val {
                "black" => Self::Variant(ColorVariant::Black),
                _ => Self::Reserved,
            }
        }
    }
}

/// Represents the red, green, and blue components of a [Color].
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    /// Creates a new [Rgb].
    pub const fn new() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    /// Creates a new [Rgb] from the given values.
    pub const fn create(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Gets the red [Rgb] value.
    pub const fn r(&self) -> u8 {
        self.r
    }

    /// Sets the red [Rgb] value.
    pub fn set_r(&mut self, r: u8) {
        self.r = r;
    }

    /// Gets the green [Rgb] value.
    pub const fn g(&self) -> u8 {
        self.g
    }

    /// Sets the green [Rgb] value.
    pub fn set_g(&mut self, g: u8) {
        self.g = g;
    }

    /// Gets the blue [Rgb] value.
    pub const fn b(&self) -> u8 {
        self.b
    }

    /// Sets the blue [Rgb] value.
    pub fn set_b(&mut self, b: u8) {
        self.b = b;
    }
}

impl From<ColorName> for Rgb {
    fn from(val: ColorName) -> Self {
        let rgb = u32::from(val);

        Self {
            r: ((rgb & 0xff0000) >> 16) as u8,
            g: ((rgb & 0xff00) >> 8) as u8,
            b: (rgb & 0xff) as u8,
        }
    }
}

/// Represents a Sixel color.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color {
    pub name: ColorName,
    pub rgb: Rgb,
}

impl Color {
    /// Creates a new [Color].
    pub const fn new() -> Self {
        Self {
            name: ColorName::new(),
            rgb: Rgb::new(),
        }
    }
}

impl From<&str> for Color {
    fn from(val: &str) -> Self {
        let name = ColorName::from(val);
        let rgb = Rgb::from(name);

        Self { name, rgb }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    fn test_color_parsing() -> Result<()> {
        let color_str = "black";
        let rgb0_str = "rgb:AB1122";
        let rgb1_str = "#A1c022";
        let invalid_str = "not a RGB color";
        let too_short0_str = "rgb:1";
        let too_short1_str = "#1";
        let rgb_trailing_str = "#A1c022whatisallthistrailgarbage";

        assert_eq!(
            ColorName::from(color_str),
            ColorName::Variant(ColorVariant::Black)
        );
        assert_eq!(ColorName::from(rgb0_str), ColorName::Rgb(0xab1122));
        assert_eq!(ColorName::from(rgb1_str), ColorName::Rgb(0xa1c022));
        assert_eq!(ColorName::from(invalid_str), ColorName::Reserved);
        assert_eq!(ColorName::from(rgb_trailing_str), ColorName::Rgb(0xa1c022));
        assert_eq!(ColorName::from(too_short0_str), ColorName::Reserved);
        assert_eq!(ColorName::from(too_short1_str), ColorName::Reserved);

        Ok(())
    }
}
