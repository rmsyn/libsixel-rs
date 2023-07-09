/// DCS escape sequence length (8-bit mode)
pub const DCS_8BIT_LEN: usize = 1;
/// DCS escape sequence (8-bit mode)
pub const DCS_8BIT: [u8; DCS_8BIT_LEN] = [0o220];
/// ST escape sequence (8-bit mode)
pub const ST_8BIT: [u8; DCS_8BIT_LEN] = [0o234];

/// DCS escape sequence length (7-bit mode)
pub const DCS_7BIT_LEN: usize = 2;
/// DCS escape sequence (7-bit mode)
pub const DCS_7BIT: [u8; DCS_7BIT_LEN] = [0o033, b'P'];
/// ST escape sequence (7-bit mode)
pub const ST_7BIT: [u8; DCS_7BIT_LEN] = [0o033, b'\\'];

/// ESC terminal escape code
pub const ESC: char = 0x1b as char;

/// Beginning of the Sixel data character range.
pub const SIXEL_CHAR_START: u8 = b'?';
/// Ending of the Sixel data character range.
pub const SIXEL_CHAR_END: u8 = b'~';

pub const RGB_ZERO: [u8; 3] = [0u8; 3];
pub const RGB_QUART: [u8; 3] = [0x3fu8; 3];
pub const RGB_HALF: [u8; 3] = [0x7fu8; 3];
pub const RGB_FULL: [u8; 3] = [0xffu8; 3];
