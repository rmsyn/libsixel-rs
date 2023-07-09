use super::constants::SIXEL_CHAR_START;
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

    /// Converts a [SixelPlane] of monochrome pixels into a [SixelChar].
    ///
    /// Parameters:
    ///
    /// - `plane`: six vertical pixel bytes representing a single hue intensity.
    /// - `plane_idx`: hue index for the [SixelPlane], e.g. R=0 for RGB
    pub fn from_plane(plane: &SixelPlane, plane_idx: u8) -> Self {
        if !(0..8).contains(&plane_idx) {
            Self::new()
        } else {
            let mut sorted = *plane;
            sorted.sort();

            let lo_range = sorted[0]..=sorted[3];
            let hi_range = sorted[4]..=sorted[5];

            (((lo_range.contains(&plane[0]) && hi_range.contains(&plane[0])) as u8)
                | (((lo_range.contains(&plane[1]) && hi_range.contains(&plane[1])) as u8) << 1)
                | (((lo_range.contains(&plane[2]) && hi_range.contains(&plane[2])) as u8) << 2)
                | (((lo_range.contains(&plane[3]) && hi_range.contains(&plane[3])) as u8) << 3)
                | (((lo_range.contains(&plane[4]) && hi_range.contains(&plane[4])) as u8) << 4)
                | (((lo_range.contains(&plane[5]) && hi_range.contains(&plane[5])) as u8) << 5))
                .into()
        }
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
        Self((val % SIXEL_CHAR_START) + SIXEL_CHAR_START)
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
