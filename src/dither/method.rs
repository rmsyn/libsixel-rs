//! Dither methods
//!
//! Functions for applying different dither algorithms.

use crate::{dimension::SpaceDimension, quant::MethodForDiffuse, Error, Result};

/// Convenience alias for a dither method callback.
pub type DitherFn = fn(&mut [u8], usize) -> Result<()>;

/// No-op dither method
pub fn dither_func_none(_data: &mut [u8], _width: usize) -> Result<()> {
    Ok(())
}

/// Floyd-Steinberg method
///
/// | Transform coefficients |       |       |      |
/// |------------------------|-------|-------|------|
/// |                        |       |  curr | 7/16 |
/// |                        | 3/16  |  5/48 | 1/16 |
pub fn dither_func_fs(data: &mut [u8], width: usize) -> Result<()> {
    let data_len = data.len();
    let max_offset = width * 3 + 3;

    if max_offset > data_len {
        Err(Error::Dither(format!(
            "dither_func_fs: max offset({max_offset}) is out-of-bounds, max: {data_len}"
        )))
    } else {
        let (error_r, error_g, error_b) = (data[0] & 0x7, data[1] & 0x7, data[2] & 0x7);

        // FIXME: this is a direct port of the math from libsixel, refactored to idiomatic Rust.
        //
        // It only includes transforms for 5/16, 3/16, and 5/16.
        //
        // The coefficients should include 7/16, 3/16, 5/48, 1/16.
        //
        // Unclear why they were omitted from the original...

        let mut offset = 3;

        // r
        data[offset] = data[offset].saturating_add((error_r * 5) >> 4);
        // g
        data[offset + 1] = data[offset + 1].saturating_add((error_g * 5) >> 4);
        // b
        data[offset + 2] = data[offset + 2].saturating_add((error_b * 5) >> 4);

        offset = width * offset - 3;

        // r
        data[offset] = data[offset].saturating_add((error_r * 3) >> 4);
        // g
        data[offset + 1] = data[offset + 1].saturating_add((error_g * 3) >> 4);
        // b
        data[offset + 2] = data[offset + 2].saturating_add((error_b * 3) >> 4);

        offset += 3;

        // r
        data[offset] = data[offset].saturating_add((error_r * 5) >> 4);
        // g
        data[offset + 1] = data[offset + 1].saturating_add((error_g * 5) >> 4);
        // b
        data[offset + 2] = data[offset + 2].saturating_add((error_b * 5) >> 4);

        Ok(())
    }
}

/// Atkinson's method
///
/// | Transform coefficients |     |       |     |     |
/// |------------------------|-----|-------|-----|-----|
/// |                        |     |  curr | 1/8 | 1/8 |
/// |                        | 1/8 |  1/8  | 1/8 |     |
/// |                        |     |  1/8  |     |     |
pub fn dither_func_atkinson(data: &mut [u8], width: usize) -> Result<()> {
    let data_len = data.len();
    let max_offset = width * 6 + 3;
    if max_offset > data_len {
        Err(Error::Dither(format!(
            "dither_func_fs: max offset({max_offset}) is out-of-bounds, max: {data_len}"
        )))
    } else {
        let (error_r, error_g, error_b) = (data[0] & 0x7, data[1] & 0x7, data[2] & 0x7);

        let (coeff_r, coeff_g, coeff_b) = (error_r >> 3, error_g >> 3, error_b >> 3);

        let mut offset = 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_b);

        offset *= 2;

        // r
        data[offset] = data[offset].saturating_add(coeff_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_b);

        offset += 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_b);

        offset = (width - 1) * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_b);

        offset = width * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_b);

        offset = (width + 1) * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_b);

        offset = width * 6;

        // r
        data[offset] = data[offset].saturating_add(coeff_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_b);

        Ok(())
    }
}

/// Jajuni method
///
/// | Transform coefficients |      |      |       |      |      |
/// |------------------------|------|------|-------|------|------|
/// |                        |      |      |  curr | 7/48 | 5/48 |
/// |                        | 3/48 | 5/48 |  7/48 | 5/48 | 3/48 |
/// |                        | 1/48 | 3/48 |  5/48 | 3/48 | 1/48 |
pub fn dither_func_jajuni(data: &mut [u8], width: usize) -> Result<()> {
    let data_len = data.len();
    let max_offset = (width * 2 + 2) * 3 + 3;
    if max_offset > data_len {
        Err(Error::Dither(format!(
            "dither_func_fs: max offset({max_offset}) is out-of-bounds, max: {data_len}"
        )))
    } else {
        let (error_r, error_g, error_b) = (data[0] & 0x7, data[1] & 0x7, data[2] & 0x7);

        let (coeff_1_48_r, coeff_1_48_g, coeff_1_48_b) = (error_r / 48, error_g / 48, error_b / 48);
        let (coeff_3_48_r, coeff_3_48_g, coeff_3_48_b) = (error_r / 16, error_g / 16, error_b / 16);
        let (coeff_5_48_r, coeff_5_48_g, coeff_5_48_b) =
            (error_r * 5 / 48, error_g * 5 / 48, error_b * 5 / 48);
        let (coeff_7_48_r, coeff_7_48_g, coeff_7_48_b) =
            (error_r * 7 / 48, error_g * 7 / 48, error_b * 7 / 48);

        let mut offset = 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_7_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_7_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_7_48_b);

        offset *= 2;

        // r
        data[offset] = data[offset].saturating_add(coeff_5_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_5_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_5_48_b);

        offset = (width - 2) * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_3_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_3_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_3_48_b);

        offset = (width - 1) * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_5_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_5_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_5_48_b);

        offset = width * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_7_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_7_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_7_48_b);

        offset = (width + 1) * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_5_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_5_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_5_48_b);

        offset = (width + 2) * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_3_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_3_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_3_48_b);

        offset = (width * 2 - 2) * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_1_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_1_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_1_48_b);

        offset = (width * 2 - 1) * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_3_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_3_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_3_48_b);

        offset = width * 6;

        // r
        data[offset] = data[offset].saturating_add(coeff_5_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_5_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_5_48_b);

        offset = (width * 2 + 1) * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_3_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_3_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_3_48_b);

        offset = (width * 2 + 2) * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_1_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_1_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_1_48_b);

        Ok(())
    }
}

/// Stucki's method
///
/// | Transform coefficients |      |      |       |      |      |
/// |------------------------|------|------|-------|------|------|
/// |                        |      |      |  curr | 8/48 | 4/48 |
/// |                        | 2/48 | 4/48 |  8/48 | 4/48 | 2/48 |
/// |                        | 1/48 | 2/48 |  4/48 | 2/48 | 1/48 |
pub fn dither_func_stucki(data: &mut [u8], width: usize) -> Result<()> {
    let data_len = data.len();
    let max_offset = (width * 2 + 2) * 3 + 3;
    if max_offset > data_len {
        Err(Error::Dither(format!(
            "dither_func_fs: max offset({max_offset}) is out-of-bounds, max: {data_len}"
        )))
    } else {
        let (error_r, error_g, error_b) = (data[0] & 0x7, data[1] & 0x7, data[2] & 0x7);

        let (coeff_1_48_r, coeff_1_48_g, coeff_1_48_b) = (error_r / 48, error_g / 48, error_b / 48);
        let (coeff_2_48_r, coeff_2_48_g, coeff_2_48_b) = (error_r / 24, error_g / 24, error_b / 24);
        let (coeff_4_48_r, coeff_4_48_g, coeff_4_48_b) = (error_r / 12, error_g / 12, error_b / 12);
        let (coeff_8_48_r, coeff_8_48_g, coeff_8_48_b) = (error_r / 6, error_g / 6, error_b / 6);

        let mut offset = 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_8_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_8_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_8_48_b);

        offset *= 2;

        // r
        data[offset] = data[offset].saturating_add(coeff_4_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_4_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_4_48_b);

        offset = (width - 2) * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_2_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_2_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_2_48_b);

        offset = (width - 1) * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_4_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_4_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_4_48_b);

        offset = width * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_8_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_8_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_8_48_b);

        offset = (width + 1) * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_4_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_4_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_4_48_b);

        offset = (width + 2) * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_2_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_2_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_2_48_b);

        offset = (width * 2 - 2) * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_1_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_1_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_1_48_b);

        offset = (width * 2 - 1) * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_2_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_2_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_2_48_b);

        offset = width * 6;

        // r
        data[offset] = data[offset].saturating_add(coeff_4_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_4_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_4_48_b);

        offset = (width * 2 + 1) * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_2_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_2_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_2_48_b);

        offset = (width * 2 + 2) * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_1_48_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_1_48_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_1_48_b);

        Ok(())
    }
}

/// Burkes' method
///
/// | Transform coefficients |      |      |       |      |      |
/// |------------------------|------|------|-------|------|------|
/// |                        |      |      |  curr | 4/16 | 2/16 |
/// |                        | 1/16 | 2/16 |  4/16 | 2/16 | 1/16 |
pub fn dither_func_burkes(data: &mut [u8], width: usize) -> Result<()> {
    let data_len = data.len();
    let max_offset = (width + 2) * 3 + 3;
    if max_offset > data_len {
        Err(Error::Dither(format!(
            "dither_func_fs: max offset({max_offset}) is out-of-bounds, max: {data_len}"
        )))
    } else {
        let (error_r, error_g, error_b) = (data[0] & 0x7, data[1] & 0x7, data[2] & 0x7);

        let (coeff_1_16_r, coeff_1_16_g, coeff_1_16_b) = (error_r / 16, error_g / 16, error_b / 16);
        let (coeff_2_16_r, coeff_2_16_g, coeff_2_16_b) = (error_r / 8, error_g / 8, error_b / 8);
        let (coeff_4_16_r, coeff_4_16_g, coeff_4_16_b) = (error_r / 4, error_g / 4, error_b / 4);

        let mut offset = 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_4_16_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_4_16_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_4_16_b);

        offset *= 2;

        // r
        data[offset] = data[offset].saturating_add(coeff_2_16_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_2_16_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_2_16_b);

        offset = (width - 2) * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_1_16_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_1_16_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_1_16_b);

        offset = (width - 1) * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_2_16_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_2_16_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_2_16_b);

        offset = width * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_4_16_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_4_16_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_4_16_b);

        offset = (width + 1) * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_2_16_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_2_16_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_2_16_b);

        offset = (width + 2) * 3;

        // r
        data[offset] = data[offset].saturating_add(coeff_1_16_r);
        // g
        data[offset + 1] = data[offset + 1].saturating_add(coeff_1_16_g);
        // b
        data[offset + 2] = data[offset + 2].saturating_add(coeff_1_16_b);

        Ok(())
    }
}

/// `A` dither method
pub fn dither_func_a_dither(data: &mut [u8], _width: usize, x: usize, y: usize) -> Result<()> {
    let data_len = data.len();

    if data_len < 3 {
        Err(Error::Dither(format!(
            "depth(3) is out-of-bounds, max: {data_len}"
        )))
    } else {
        for c in data.iter_mut().take(3) {
            let mut mask = ((((x + *c as usize * 17) + y * 236) * 119) & 255) as f32;
            mask = (mask - 128.0) / 256.0;
            let value = (*c as f32) + mask;

            *c = if (0.0..=255.0).contains(&value) {
                value as u8
            } else if value < 0.0
                || value.is_nan()
                || (value.is_infinite() && value.is_sign_negative())
            {
                0
            } else if value > 255.0 || (value.is_infinite() && value.is_sign_positive()) {
                255
            } else {
                0
            };
        }

        Ok(())
    }
}

/// `X` dither method
pub fn dither_func_x_dither(data: &mut [u8], _width: usize, x: usize, y: usize) -> Result<()> {
    let data_len = data.len();

    if data_len < 3 {
        Err(Error::Dither(format!(
            "depth(3) is out-of-bounds, max: {data_len}"
        )))
    } else {
        for c in data.iter_mut().take(3) {
            let mut mask = ((((x + *c as usize * 17) + y * 236) * 1234) & 511) as f32;
            mask = (mask - 128.0) / 512.0;
            let value = (*c as f32) + mask;

            *c = if (0.0..=255.0).contains(&value) {
                value as u8
            } else if value < 0.0
                || value.is_nan()
                || (value.is_infinite() && value.is_sign_negative())
            {
                0
            } else if value > 255.0 || (value.is_infinite() && value.is_sign_positive()) {
                255
            } else {
                0
            };
        }

        Ok(())
    }
}

/// Apply a dither method based on [`MethodForDiffuse`] variant.
pub fn apply_15bpp_dither(
    pixels: &mut [u8],
    sd: SpaceDimension,
    method_for_diffuse: MethodForDiffuse,
) -> Result<()> {
    let (x, y, width, height) = (sd.x, sd.y, sd.width, sd.height);

    match method_for_diffuse {
        MethodForDiffuse::Fs if x < width - 1 && y < height - 1 => dither_func_fs(pixels, width),
        MethodForDiffuse::Atkinson if x < width - 2 && y < height - 2 => {
            dither_func_atkinson(pixels, width)
        }
        MethodForDiffuse::Jajuni if x < width - 2 && y < height - 2 => {
            dither_func_jajuni(pixels, width)
        }
        MethodForDiffuse::Stucki if x < width - 2 && y < height - 2 => {
            dither_func_stucki(pixels, width)
        }
        MethodForDiffuse::Burkes if x < width - 2 && y < height - 2 => {
            dither_func_burkes(pixels, width)
        }
        MethodForDiffuse::ADither => dither_func_a_dither(pixels, width, x, y),
        MethodForDiffuse::XDither => dither_func_x_dither(pixels, width, x, y),
        _ => dither_func_none(pixels, width),
    }
}
