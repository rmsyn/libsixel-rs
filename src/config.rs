//! Configuration settings for the library and binary.

use crate::std::fmt;

/// Loop mode
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum LoopMode {
    /// Honor the GIF header setting.
    #[default]
    Auto = 0,
    /// Always enable loop mode.
    Force,
    /// Always disable loop mode.
    Disable,
}

impl LoopMode {
    /// Creates a new [LoopMode].
    pub const fn new() -> Self {
        Self::Auto
    }
}

/// Configuration settings for choosing the palette color space.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum PaletteType {
    /// Choose palette type automatically.
    #[default]
    Auto = 0,
    /// HLS color space.
    Hls,
    /// RGB color space.
    Rgb,
}

impl PaletteType {
    /// Creates a new [PaletteType].
    pub const fn new() -> Self {
        Self::Auto
    }
}

/// Color map settings
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ColorOption {
    /// Use default settings.
    #[default]
    Default = 0,
    /// Use monochrome palette.
    Monochrome,
    /// Use builtin palette.
    Builtin,
    /// Use mapfile option.
    Mapfile,
    /// Use high color option.
    HighColor,
}

impl ColorOption {
    /// Creates a new [ColorOption].
    pub const fn new() -> Self {
        Self::Default
    }
}

/// Builtin palette selection settings
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum BuiltinPalette {
    /// Monochrome terminal with dark background.
    #[default]
    MonoDark = 0,
    /// Monochrome terminal with light background
    MonoLight,
    /// xterm 16color
    Xterm16,
    /// xterm 256color
    Xterm256,
    /// VT340 monochrome
    Vt340Mono,
    /// VT340 color
    Vt340Color,
    /// 1bit grayscale
    G1,
    /// 2bit grayscale
    G2,
    /// 4bit grayscale
    G4,
    /// 8bit grayscale
    G8,
}

impl BuiltinPalette {
    /// Creates a new [BuiltinPalette].
    pub const fn new() -> Self {
        Self::MonoDark
    }
}

impl From<u8> for BuiltinPalette {
    fn from(val: u8) -> Self {
        match val {
            v if v == 0 => Self::MonoDark,
            v if v == 1 => Self::MonoLight,
            v if v == 2 => Self::Xterm16,
            v if v == 3 => Self::Xterm256,
            v if v == 4 => Self::Vt340Mono,
            v if v == 5 => Self::Vt340Color,
            v if v == 6 => Self::G1,
            v if v == 7 => Self::G2,
            v if v == 8 => Self::G4,
            v if v == 9 => Self::G8,
            _ => Self::MonoDark,
        }
    }
}

impl From<bool> for BuiltinPalette {
    fn from(val: bool) -> Self {
        (val as u8).into()
    }
}

impl From<&BuiltinPalette> for &'static str {
    fn from(val: &BuiltinPalette) -> Self {
        match val {
            BuiltinPalette::MonoDark => "mono dark",
            BuiltinPalette::MonoLight => "mono light",
            BuiltinPalette::Xterm16 => "xterm 16-bit",
            BuiltinPalette::Xterm256 => "xterm 256-bit",
            BuiltinPalette::Vt340Mono => "vt340 monochrome",
            BuiltinPalette::Vt340Color => "vt340 color",
            BuiltinPalette::G1 => "grayscale 1-bit",
            BuiltinPalette::G2 => "grayscale 2-bit",
            BuiltinPalette::G4 => "grayscale 4-bit",
            BuiltinPalette::G8 => "grayscale 8-bit",
        }
    }
}

impl From<BuiltinPalette> for &'static str {
    fn from(val: BuiltinPalette) -> Self {
        (&val).into()
    }
}

impl fmt::Display for BuiltinPalette {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&str>::from(self))
    }
}
