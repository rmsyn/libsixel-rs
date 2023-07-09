use super::constants::ESC;
use crate::std::fmt;

/// Bitness encoding for the [DeviceControlString](super::DeviceControlString).
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum DcsMode {
    #[default]
    SevenBit = 0,
    EightBit,
}

impl DcsMode {
    /// Creates a new [DcsMode] selector.
    pub const fn new() -> Self {
        Self::SevenBit
    }
}

/// DCS escape sequence function
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum DcsFunction {
    #[default]
    SevenBit = b'P',
    EightBit = 0x90,
}

impl DcsFunction {
    /// Creates a new [DcsFunction].
    pub const fn new() -> Self {
        Self::SevenBit
    }
}

impl From<DcsFunction> for u8 {
    fn from(val: DcsFunction) -> Self {
        val as u8
    }
}

impl From<&DcsFunction> for u8 {
    fn from(val: &DcsFunction) -> Self {
        (*val).into()
    }
}

impl From<DcsFunction> for char {
    fn from(val: DcsFunction) -> Self {
        (val as u8) as char
    }
}

impl From<&DcsFunction> for char {
    fn from(val: &DcsFunction) -> Self {
        (*val).into()
    }
}

impl From<&DcsMode> for DcsFunction {
    fn from(val: &DcsMode) -> Self {
        match val {
            DcsMode::SevenBit => Self::SevenBit,
            DcsMode::EightBit => Self::EightBit,
        }
    }
}

impl From<DcsMode> for DcsFunction {
    fn from(val: DcsMode) -> Self {
        (&val).into()
    }
}

impl From<&DcsFunction> for DcsMode {
    fn from(val: &DcsFunction) -> Self {
        match val {
            DcsFunction::SevenBit => Self::SevenBit,
            DcsFunction::EightBit => Self::EightBit,
        }
    }
}

impl From<DcsFunction> for DcsMode {
    fn from(val: DcsFunction) -> Self {
        (&val).into()
    }
}

impl fmt::Display for DcsFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self == &Self::SevenBit {
            write!(f, "{ESC}")?;
        }
        write!(f, "{}", char::from(self))
    }
}

/// ST escape sequence function
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum StFunction {
    #[default]
    SevenBit = b'\\',
    EightBit = 0x9c,
}

impl StFunction {
    /// Creates a new [StFunction].
    pub const fn new() -> Self {
        Self::SevenBit
    }
}

impl From<StFunction> for u8 {
    fn from(val: StFunction) -> Self {
        val as u8
    }
}

impl From<&StFunction> for u8 {
    fn from(val: &StFunction) -> Self {
        (*val).into()
    }
}

impl From<StFunction> for char {
    fn from(val: StFunction) -> Self {
        (val as u8) as char
    }
}

impl From<&StFunction> for char {
    fn from(val: &StFunction) -> Self {
        (*val).into()
    }
}

impl From<&DcsMode> for StFunction {
    fn from(val: &DcsMode) -> Self {
        match val {
            DcsMode::SevenBit => Self::SevenBit,
            DcsMode::EightBit => Self::EightBit,
        }
    }
}

impl From<DcsMode> for StFunction {
    fn from(val: DcsMode) -> Self {
        (&val).into()
    }
}

impl From<&StFunction> for DcsMode {
    fn from(val: &StFunction) -> Self {
        match val {
            StFunction::SevenBit => Self::SevenBit,
            StFunction::EightBit => Self::EightBit,
        }
    }
}

impl From<StFunction> for DcsMode {
    fn from(val: StFunction) -> Self {
        (&val).into()
    }
}

impl fmt::Display for StFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self == &Self::SevenBit {
            write!(f, "{ESC}")?;
        }
        write!(f, "{}", char::from(self))
    }
}

/// Selects how the terminal draws the background color. You can use one of three values.
///
/// | P2 ([`DcsBackground`]) | Meaning                                                                 |
/// |------------------------|-------------------------------------------------------------------------|
/// | **0** or 2 (default)   | Pixel positions specified as 0 are set to the current background color. |
/// | 1                      | Pixel positions specified as 0 remain at their current color.           |
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum DcsBackground {
    Background = 0,
    Current = 1,
    BackgroundAlt = 2,
    #[default]
    None = 0xff,
}

impl DcsBackground {
    /// Creates a new [DcsBackground] selector.
    pub const fn new() -> Self {
        Self::None
    }
}

impl From<u8> for DcsBackground {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Background,
            1 => Self::Current,
            2 => Self::BackgroundAlt,
            _ => Self::None,
        }
    }
}

impl From<DcsBackground> for u8 {
    fn from(val: DcsBackground) -> Self {
        val as u8
    }
}

impl From<&DcsBackground> for u8 {
    fn from(val: &DcsBackground) -> Self {
        (*val).into()
    }
}

impl From<DcsBackground> for char {
    fn from(val: DcsBackground) -> Self {
        (val as u8) as char
    }
}

impl From<&DcsBackground> for char {
    fn from(val: &DcsBackground) -> Self {
        (*val).into()
    }
}

impl fmt::Display for DcsBackground {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

/// Represents the height-to-width pixel ratio.
///
/// | P1          | Pixel Aspect Ratio (Vertical:Horizontal) |
/// |-------------|------------------------------------------|
/// | **Omitted** | **2:1** (default)                        |
/// | 0, 1        | 2:1                                      |
/// | 2           | 5:1                                      |
/// | 3, 4        | 3:1                                      |
/// | 5, 6        | 2:1                                      |
/// | 7, 8, 9     | 1:1                                      |
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum PixelAspectRatio {
    TwoOneA = 0,
    TwoOneB = 1,
    TwoOneC = 5,
    TwoOneD = 6,
    FiveOne = 2,
    ThreeOneA = 3,
    ThreeOneB = 4,
    OneOneA = 7,
    OneOneB = 8,
    OneOneC = 9,
    #[default]
    None = 0xff,
}

impl PixelAspectRatio {
    /// Creates a new [PixelAspectRatio] selector.
    pub const fn new() -> Self {
        Self::None
    }
}

impl From<PixelAspectRatio> for u8 {
    fn from(val: PixelAspectRatio) -> Self {
        val as u8
    }
}

impl From<&PixelAspectRatio> for u8 {
    fn from(val: &PixelAspectRatio) -> Self {
        (*val).into()
    }
}

impl From<PixelAspectRatio> for char {
    fn from(val: PixelAspectRatio) -> Self {
        (val as u8) as char
    }
}

impl From<&PixelAspectRatio> for char {
    fn from(val: &PixelAspectRatio) -> Self {
        (*val).into()
    }
}

impl fmt::Display for PixelAspectRatio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(self))
    }
}
