use super::constants::{SIXEL_CHAR_END, SIXEL_CHAR_START};
use crate::std::fmt;

/// Convenience alias for RGB pixel bytes.
pub type RgbBytes = [u8; 3];
/// Convenience alias for RGB sixel bytes.
pub type RgbSixelBytes = [RgbBytes; 6];
/// Convenience alias for sixel plane (six monochrome pixels).
pub type SixelPlane = [u8; 6];

/// Represents a valid Sixel character.
///
/// The sixel data characters are characters in the range of `? (hex 3F)` to `~ (hex 7E)`.
///
/// Each sixel data character represents six vertical pixels of data.
///
/// Each sixel data character represents a binary value equal to the character code value minus hex 3F.
///
/// Examples
///
/// - `? (hex 3F)` represents the binary value `000000`.
/// - `t (hex 74)` represents the binary value `110101`.
/// - `~ (hex 7E)` represents the binary value `111111`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SixelChar(u8);

impl SixelChar {
    /// Creates a new [SixelChar].
    pub const fn new() -> Self {
        Self(SIXEL_CHAR_START)
    }

    /// Creates a full [SixelChar].
    pub const fn full() -> Self {
        Self(SIXEL_CHAR_END)
    }

    /// Converts a [RgbSixelBytes] of monochrome pixels into a [SixelChar].
    ///
    /// Parameters:
    ///
    /// - `plane`: six vertical pixel bytes representing a single hue intensity.
    /// - `six_idx`: sixel index for the [RgbSixelBytes]
    pub fn from_plane(plane: &RgbSixelBytes, six_idx: usize, hits: &[u8], threshold: u8) -> Self {
        if six_idx < plane.len() && six_idx < hits.len() && (1..=threshold).contains(&hits[six_idx])
        {
            (1u8 << six_idx).into()
        } else {
            Self::new()
        }
    }

    /// Converts a [RgbBytes] into a [SixelChar].
    ///
    /// Parameters:
    ///
    /// - `plane`: six vertical pixel bytes representing a single hue intensity.
    /// - `six_idx`: sixel index for the [RgbSixelBytes]
    pub fn from_index(six_idx: usize) -> Self {
        (1u8 << six_idx).into()
    }
}

impl Default for SixelChar {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&SixelChar> for u8 {
    fn from(val: &SixelChar) -> Self {
        val.0
    }
}

impl From<SixelChar> for u8 {
    fn from(val: SixelChar) -> Self {
        (&val).into()
    }
}

impl From<u8> for SixelChar {
    fn from(val: u8) -> Self {
        if val < SIXEL_CHAR_START {
            Self(SIXEL_CHAR_START + val)
        } else if (SIXEL_CHAR_START..=SIXEL_CHAR_END).contains(&val) {
            Self(val)
        } else {
            Self((val % SIXEL_CHAR_START) + SIXEL_CHAR_START)
        }
    }
}

impl From<&SixelChar> for char {
    fn from(val: &SixelChar) -> Self {
        val.0 as char
    }
}

impl From<SixelChar> for char {
    fn from(val: SixelChar) -> Self {
        (&val).into()
    }
}

impl From<char> for SixelChar {
    fn from(val: char) -> Self {
        (val as u8).into()
    }
}

impl From<&RgbSixelBytes> for SixelChar {
    fn from(val: &RgbSixelBytes) -> Self {
        // FIXME: this is a very naive algorithm, actually use the diffusion functions
        let mut sorted = *val;
        sorted.sort();

        // consider RGB values that are >= median value as switched on
        //
        // shifts values so that:
        //
        // - top pixel encodes to LSB
        // - bottom pixel encodes to the MSB
        (((sorted[0] <= val[0] && val[0] < sorted[5]) as u8
            | (((sorted[0] <= val[1] && val[1] < sorted[5]) as u8) << 1)
            | (((sorted[0] <= val[2] && val[2] < sorted[5]) as u8) << 2)
            | (((sorted[0] <= val[3] && val[3] < sorted[5]) as u8) << 3)
            | (((sorted[0] <= val[4] && val[4] < sorted[5]) as u8) << 4)
            | (((sorted[0] <= val[5] && val[5] < sorted[5]) as u8) << 5))
            + SIXEL_CHAR_START)
            .into()
    }
}

impl From<RgbSixelBytes> for SixelChar {
    fn from(val: RgbSixelBytes) -> Self {
        (&val).into()
    }
}

impl fmt::Display for SixelChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(self))
    }
}
