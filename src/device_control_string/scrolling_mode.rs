use super::constants::ESC;
use crate::std::fmt;

/// Sixel scrolling mode
///
/// You can set the sixel scrolling mode by using the Sixel Scrolling feature in the Graphics Set-Up screen. You can also select this mode by using the sixel display mode (DECSDM) control function.
///
/// When sixel display mode is set, the Sixel Scrolling feature is enabled. When sixel display mode is reset, the Sixel Scrolling feature is disabled.
///
/// To set DECSDM, the control function is.
///
/// | CSI  | ?    | 8   | 0   | h   |
/// |------|------|-----|-----|-----|
/// | 9/11 | 3/15 | 3/8 | 3/0 | 6/8 |
///
/// To reset DECSDM, the control function is.
///
/// | CSI  | ?    | 8    | 0     | 1      |
/// |------|------|------|-------|--------|
/// | 9/11 | 3/15 | 3/8  | 3/0   | 6/12   |
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ScrollingMode {
    Enable = b'h',
    #[default]
    Disable = b'l',
}

impl From<ScrollingMode> for u8 {
    fn from(val: ScrollingMode) -> Self {
        val as u8
    }
}

impl From<&ScrollingMode> for u8 {
    fn from(val: &ScrollingMode) -> Self {
        (*val).into()
    }
}

impl From<ScrollingMode> for char {
    fn from(val: ScrollingMode) -> Self {
        (val as u8) as char
    }
}

impl From<&ScrollingMode> for char {
    fn from(val: &ScrollingMode) -> Self {
        (*val).into()
    }
}

impl fmt::Display for ScrollingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{ESC} [ ? 8 0 {}", char::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let enable = ScrollingMode::Enable;
        let disable = ScrollingMode::Disable;

        let exp_enable = [
            ESC as u8, b' ', b'[', b' ', b'?', b' ', b'8', b' ', b'0', b' ', b'h',
        ];
        let exp_disable = [
            ESC as u8, b' ', b'[', b' ', b'?', b' ', b'8', b' ', b'0', b' ', b'l',
        ];

        assert_eq!(format!("{enable}").as_bytes(), exp_enable.as_ref());
        assert_eq!(format!("{disable}").as_bytes(), exp_disable.as_ref());
    }
}
