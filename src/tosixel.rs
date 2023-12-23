//! Writes Sixel data to an output.

use alloc::vec::Vec;

use crate::std::cmp;

pub const DCS_START_7BIT_LEN: usize = 2;
pub const DCS_START_7BIT: [u8; DCS_START_7BIT_LEN] = [0o033, b'P'];

pub const DCS_END_7BIT_LEN: usize = 2;
pub const DCS_END_7BIT: [u8; DCS_END_7BIT_LEN] = [0o033, b'\\'];

pub const DCS_START_8BIT_LEN: usize = 1;
pub const DCS_START_8BIT: [u8; DCS_START_8BIT_LEN] = [0o220];

pub const DCS_END_8BIT_LEN: usize = 1;
pub const DCS_END_8BIT: [u8; DCS_END_8BIT_LEN] = [0o234];

pub const SCREEN_PACKET_SIZE: usize = 256;

/// Wraps sixel data in DCS 7-bit escape sequences.
pub fn dcs_7bit(x: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(DCS_START_7BIT_LEN + x.len() + DCS_END_7BIT_LEN);

    out.extend_from_slice(DCS_START_7BIT.as_ref());
    out.extend_from_slice(x);
    out.extend_from_slice(DCS_END_7BIT.as_ref());

    out
}

/// Wraps sixel data in DCS 8-bit escape sequences.
pub fn dcs_8bit(x: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(DCS_START_8BIT_LEN + x.len() + DCS_END_8BIT_LEN);

    out.push(DCS_START_8BIT[0]);
    out.extend_from_slice(x);
    out.push(DCS_END_8BIT[0]);

    out
}

/// Writes a byte to a byte buffer.
///
/// No-op if passed an empty buffer.
pub fn putc(buf: &mut [u8], val: u8) {
    if !buf.is_empty() {
        buf[0] = val;
    }
}

/// Writes a string to a byte buffer.
///
/// Copies `min(buf_len, val_len)` bytes.
pub fn puts(buf: &mut [u8], val: &str) {
    let len = cmp::min(buf.len(), val.len());

    buf[..len].copy_from_slice(val.as_bytes()[..len].as_ref());
}

/// Copies a byte buffer to a byte buffer.
///
/// Copies `min(buf_len, val_len)` bytes.
pub fn putb(buf: &mut [u8], val: &[u8]) {
    let len = cmp::min(buf.len(), val.len());

    buf[..len].copy_from_slice(val[..len].as_ref());
}

/// Writes a number to a byte buffer.
///
/// No-op if the length of the formatted number is larger than the byte buffer, returns `0`.
///
/// Returns the length of the formatted string on success.
pub fn putnum(buf: &mut [u8], val: u32) -> usize {
    let num = format!("{val}");

    let num_len = num.len();
    let buf_len = buf.len();

    if num_len > buf_len {
        0
    } else {
        buf[..num_len].copy_from_slice(num.as_bytes());
        num_len
    }
}
