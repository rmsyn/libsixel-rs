use crate::std::fmt;

/// Terminal control characters used in Sixel input mode.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SixelControl {
    #[default]
    Parameter = b';',
    CarriageReturn = b'$',
    NewLine = b'-',
    Repeat = b'!',
    Color = b'#',
    Raster = b'"',
}

impl SixelControl {
    /// Creates a new [SixelControl].
    pub const fn new() -> Self {
        Self::Parameter
    }
}

impl From<SixelControl> for u8 {
    fn from(val: SixelControl) -> Self {
        val as u8
    }
}

impl From<&SixelControl> for u8 {
    fn from(val: &SixelControl) -> Self {
        (*val).into()
    }
}

impl From<SixelControl> for char {
    fn from(val: SixelControl) -> Self {
        (val as u8) as char
    }
}

impl From<&SixelControl> for char {
    fn from(val: &SixelControl) -> Self {
        (*val).into()
    }
}

impl fmt::Display for SixelControl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(self))
    }
}
