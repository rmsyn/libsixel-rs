use crate::{color::Rgb, std::fmt, Error, Result};

/// Represents the color format for a [PixelFormat].
#[repr(i32)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum FormatType {
    #[default]
    Color = 0,
    Grayscale = 1 << 6,
    Palette = 1 << 7,
}

/// Represents pixel format modes (bpp = bits per pixel).
#[repr(i32)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum PixelFormat {
    /// 15bpp
    Rgb555 = FormatType::Color as i32 | 0x01,
    /// 16bpp
    Rgb565 = FormatType::Color as i32 | 0x02,
    /// 24bpp
    #[default]
    Rgb888 = FormatType::Color as i32 | 0x03,
    /// 15bpp
    Bgr555 = FormatType::Color as i32 | 0x04,
    /// 16bpp
    Bgr565 = FormatType::Color as i32 | 0x05,
    /// 24bpp
    Bgr888 = FormatType::Color as i32 | 0x06,
    /// 32bpp
    Argb8888 = FormatType::Color as i32 | 0x10,
    /// 32bpp
    Rgba8888 = FormatType::Color as i32 | 0x11,
    /// 32bpp
    Abgr8888 = FormatType::Color as i32 | 0x12,
    /// 32bpp
    Bgra8888 = FormatType::Color as i32 | 0x13,
    /// 1bpp grayscale
    G1 = FormatType::Grayscale as i32,
    /// 2bpp grayscale
    G2 = FormatType::Grayscale as i32 | 0x01,
    /// 4bpp grayscale
    G4 = FormatType::Grayscale as i32 | 0x02,
    /// 8bpp grayscale
    G8 = FormatType::Grayscale as i32 | 0x03,
    /// 16bpp grayscale + alpha
    Ag88 = FormatType::Grayscale as i32 | 0x13,
    /// 16bpp grayscale + alpha
    Ga88 = FormatType::Grayscale as i32 | 0x23,
    /// 1bpp palette
    Pal1 = FormatType::Palette as i32,
    /// 2bpp palette
    Pal2 = FormatType::Palette as i32 | 0x01,
    /// 4bpp palette
    Pal4 = FormatType::Palette as i32 | 0x02,
    /// 8bpp palette
    Pal8 = FormatType::Palette as i32 | 0x03,
}

impl PixelFormat {
    /// Creates a new [PixelFormat].
    pub const fn new() -> Self {
        Self::Rgb888
    }

    /// Compute the pixel depth for the [PixelFormat].
    pub fn compute_depth(&self) -> usize {
        match self {
            Self::Argb8888 | Self::Rgba8888 | Self::Abgr8888 | Self::Bgra8888 => 4,
            Self::Rgb888 | Self::Bgr888 => 3,
            Self::Rgb555 | Self::Rgb565 | Self::Bgr555 | Self::Bgr565 | Self::Ag88 | Self::Ga88 => {
                2
            }
            Self::G1
            | Self::G2
            | Self::G4
            | Self::G8
            | Self::Pal1
            | Self::Pal2
            | Self::Pal4
            | Self::Pal8 => 1,
        }
    }

    /// Gets the [Rgb] values at the given depth from the provide data.
    pub fn rgb(&self, data: &[u8], depth: usize) -> Result<Rgb> {
        let mut pixels = 0u32;

        let data_len = data.len();

        if depth > data_len {
            return Err(Error::PixelFormat(format!("invalid RGB depth larger than data length, data length: {data_len}, depth: {depth}")));
        }

        for &d in data.iter().take(depth) {
            pixels = d as u32 | pixels << 8;
        }

        let ret = match self {
            Self::Rgb555 => Rgb::create(
                (((pixels >> 10) & 0x1f) << 3) as u8,
                (((pixels >> 5) & 0x1f) << 3) as u8,
                ((pixels & 0x1f) << 3) as u8,
            ),
            Self::Rgb565 => Rgb::create(
                (((pixels >> 11) & 0x1f) << 3) as u8,
                (((pixels >> 5) & 0x3f) << 2) as u8,
                ((pixels & 0x1f) << 3) as u8,
            ),
            Self::Rgb888 => Rgb::create(
                ((pixels >> 16) & 0xff) as u8,
                ((pixels >> 8) & 0xff) as u8,
                (pixels & 0xff) as u8,
            ),
            Self::Bgr555 => Rgb::create(
                ((pixels & 0x1f) << 3) as u8,
                (((pixels >> 5) & 0x1f) << 3) as u8,
                (((pixels >> 10) & 0x1f) << 3) as u8,
            ),
            Self::Bgr565 => Rgb::create(
                ((pixels & 0x1f) << 3) as u8,
                (((pixels >> 5) & 0x3f) << 2) as u8,
                (((pixels >> 11) & 0x1f) << 3) as u8,
            ),
            Self::Bgr888 => Rgb::create(
                (pixels & 0xff) as u8,
                ((pixels >> 8) & 0xff) as u8,
                ((pixels >> 16) & 0xff) as u8,
            ),
            Self::Rgba8888 => Rgb::create(
                ((pixels >> 24) & 0xff) as u8,
                ((pixels >> 16) & 0xff) as u8,
                ((pixels >> 8) & 0xff) as u8,
            ),
            Self::Argb8888 => Rgb::create(
                ((pixels >> 16) & 0xff) as u8,
                ((pixels >> 8) & 0xff) as u8,
                (pixels & 0xff) as u8,
            ),
            Self::Bgra8888 => Rgb::create(
                ((pixels >> 8) & 0xff) as u8,
                ((pixels >> 16) & 0xff) as u8,
                ((pixels >> 24) & 0xff) as u8,
            ),
            Self::Abgr8888 => Rgb::create(
                (pixels & 0xff) as u8,
                ((pixels >> 8) & 0xff) as u8,
                ((pixels >> 16) & 0xff) as u8,
            ),
            Self::Ga88 => {
                let color = ((pixels >> 8) & 0xff) as u8;
                Rgb::create(color, color, color)
            }
            Self::G8 | Self::Ag88 => {
                let color = (pixels & 0xff) as u8;
                Rgb::create(color, color, color)
            }
            _ => Rgb::new(),
        };

        Ok(ret)
    }

    /// Expands the [PixelFormat] RGB data.
    pub fn expand_rgb(
        &self,
        dst: &mut [u8],
        src: &[u8],
        width: usize,
        height: usize,
        depth: usize,
    ) -> Result<()> {
        let dst_len = dst.len();
        let src_len = src.len();

        for y in 0..height {
            for x in 0..width {
                let src_offset = depth.saturating_mul(y.saturating_mul(width).saturating_add(x));
                let dst_offset = 3usize.saturating_mul(y.saturating_mul(width).saturating_add(x));

                if src_offset >= src.len() {
                    return Err(Error::PixelFormat(format!(
                        "invalid depth, source length: {src_len}, source offset: {src_offset}"
                    )));
                }

                let rgb = self.rgb(&src[src_offset..], depth)?;

                if dst_offset + 2 >= dst.len() {
                    return Err(Error::PixelFormat(format!(
                        "invalid RGB depth, destination length: {dst_len}, destination offset: {}",
                        dst_offset + 2
                    )));
                }

                dst[dst_offset] = rgb.r();
                dst[dst_offset + 1] = rgb.g();
                dst[dst_offset + 2] = rgb.b();
            }
        }

        Ok(())
    }

    /// Expands the Sixel color palette.
    pub fn expand_palette(
        &self,
        dst: &mut [u8],
        src: &[u8],
        width: usize,
        height: usize,
    ) -> Result<()> {
        // bit per plane
        let mut bpp = 0;

        let mut dst_view = dst;
        let mut src_view = src;

        match self {
            PixelFormat::Pal1 | PixelFormat::G1 => {
                bpp = 1;
            }
            PixelFormat::Pal2 | PixelFormat::G2 => {
                bpp = 2;
            }
            PixelFormat::Pal4 | PixelFormat::G4 => {
                bpp = 4;
            }
            PixelFormat::Pal8 | PixelFormat::G8 => {
                let offset = width.saturating_mul(height).saturating_sub(1);
                let (dst_len, src_len) = (dst_view.len(), src_view.len());

                if offset >= dst_len || offset >= src_len {
                    return Err(Error::PixelFormat(format!("width and height outside the bounds of the palette buffers, dst len: {dst_len}, src len: {src_len}, offset: {offset}")));
                }

                dst_view = &mut dst_view[offset..];
                src_view = &src_view[offset..];
            }
            _ => {
                return Err(Error::PixelFormat(format!("invalid pixel format: {self}")));
            }
        }

        for _y in 0..height {
            let upper_bound = width.saturating_mul(bpp).saturating_div(8);
            for _x in 0..upper_bound {
                for i in 0..8usize.saturating_div(bpp) {
                    dst_view = &mut dst_view[1..];

                    let shift = 8usize
                        .saturating_div(bpp)
                        .saturating_sub(1)
                        .saturating_sub(i);
                    let tail_shift = 1u32.overflowing_shl(bpp as u32).0.saturating_sub(1);

                    dst_view[0] = ((src[0] as u32)
                        .overflowing_shr(shift as u32)
                        .0
                        .saturating_mul(bpp as u32)
                        & tail_shift) as u8;
                }
                src_view = &src_view[1..];
            }

            let x = width - upper_bound.saturating_mul(8).saturating_div(bpp);

            if x > 0 {
                for i in 0..x {
                    dst_view = &mut dst_view[1..];

                    let shift = 8usize.saturating_sub(i + 1).saturating_mul(bpp);
                    let tail_shift = 1u32.overflowing_shl(bpp as u32).0.saturating_sub(1);

                    dst_view[0] =
                        ((src_view[0] as u32).overflowing_shr(shift as u32).0 & tail_shift) as u8;
                }

                src_view = &src_view[1..];
            }
        }

        Ok(())
    }

    /// Normalizes the [PixelFormat].
    pub fn normalize(
        &mut self,
        dst: &mut [u8],
        src: &[u8],
        width: usize,
        height: usize,
    ) -> Result<()> {
        let format = match self {
            PixelFormat::G8 => {
                self.expand_rgb(dst, src, width, height, 1)?;

                PixelFormat::Rgb888
            }
            PixelFormat::Rgb565
            | PixelFormat::Rgb555
            | PixelFormat::Bgr565
            | PixelFormat::Bgr555
            | PixelFormat::Ga88
            | PixelFormat::Ag88 => {
                self.expand_rgb(dst, src, width, height, 2)?;

                PixelFormat::Rgb888
            }
            PixelFormat::Rgb888 | PixelFormat::Bgr888 => {
                self.expand_rgb(dst, src, width, height, 3)?;

                PixelFormat::Rgb888
            }
            PixelFormat::Rgba8888
            | PixelFormat::Argb8888
            | PixelFormat::Bgra8888
            | PixelFormat::Abgr8888 => {
                self.expand_rgb(dst, src, width, height, 4)?;

                PixelFormat::Rgb888
            }
            PixelFormat::Pal1 | PixelFormat::Pal2 | PixelFormat::Pal4 => {
                self.expand_palette(dst, src, width, height)?;

                PixelFormat::Pal8
            }
            PixelFormat::G1 | PixelFormat::G2 | PixelFormat::G4 => {
                self.expand_palette(dst, src, width, height)?;

                PixelFormat::G8
            }
            PixelFormat::Pal8 => {
                let len = width.saturating_mul(height);
                dst[..len].copy_from_slice(src[..len].as_ref());

                *self
            }
        };

        *self = format;

        Ok(())
    }
}

impl From<i32> for PixelFormat {
    fn from(val: i32) -> Self {
        match val {
            pf if pf == Self::Rgb555 as i32 => Self::Rgb555,
            pf if pf == Self::Rgb565 as i32 => Self::Rgb565,
            pf if pf == Self::Rgb888 as i32 => Self::Rgb888,
            pf if pf == Self::Bgr555 as i32 => Self::Bgr555,
            pf if pf == Self::Bgr565 as i32 => Self::Bgr565,
            pf if pf == Self::Bgr888 as i32 => Self::Bgr888,
            pf if pf == Self::Argb8888 as i32 => Self::Argb8888,
            pf if pf == Self::Rgba8888 as i32 => Self::Rgba8888,
            pf if pf == Self::Abgr8888 as i32 => Self::Abgr8888,
            pf if pf == Self::Bgra8888 as i32 => Self::Bgra8888,
            pf if pf == Self::G1 as i32 => Self::G1,
            pf if pf == Self::G2 as i32 => Self::G2,
            pf if pf == Self::G4 as i32 => Self::G4,
            pf if pf == Self::G8 as i32 => Self::G8,
            pf if pf == Self::Ag88 as i32 => Self::Ag88,
            pf if pf == Self::Ga88 as i32 => Self::Ga88,
            pf if pf == Self::Pal1 as i32 => Self::Pal1,
            pf if pf == Self::Pal2 as i32 => Self::Pal2,
            pf if pf == Self::Pal4 as i32 => Self::Pal4,
            pf if pf == Self::Pal8 as i32 => Self::Pal8,
            _ => Self::Rgb888,
        }
    }
}

impl From<PixelFormat> for i32 {
    fn from(val: PixelFormat) -> Self {
        val as i32
    }
}

impl From<PixelFormat> for &'static str {
    fn from(val: PixelFormat) -> Self {
        match val {
            PixelFormat::Rgb555 => "RGB555",
            PixelFormat::Rgb565 => "RGB565",
            PixelFormat::Rgb888 => "RGB888",
            PixelFormat::Bgr555 => "BGR555",
            PixelFormat::Bgr565 => "BGR565",
            PixelFormat::Bgr888 => "BGR888",
            PixelFormat::Argb8888 => "ARGB8888",
            PixelFormat::Rgba8888 => "RGBA8888",
            PixelFormat::Abgr8888 => "ABGR8888",
            PixelFormat::Bgra8888 => "BGRA8888",
            PixelFormat::G1 => "G1",
            PixelFormat::G2 => "G2",
            PixelFormat::G4 => "G4",
            PixelFormat::G8 => "G8",
            PixelFormat::Ag88 => "AG88",
            PixelFormat::Ga88 => "GA88",
            PixelFormat::Pal1 => "PAL1",
            PixelFormat::Pal2 => "PAL2",
            PixelFormat::Pal4 => "PAL4",
            PixelFormat::Pal8 => "PAL8",
        }
    }
}

impl From<&PixelFormat> for &'static str {
    fn from(val: &PixelFormat) -> Self {
        (*val).into()
    }
}

impl fmt::Display for PixelFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&str>::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() -> Result<()> {
        let mut dst = [0u8; 3];
        let src = [0x46, 0xf3, 0xe5];

        let exp_pixel_fmt = PixelFormat::Rgb888;
        let mut pixel_fmt = PixelFormat::Rgb888;

        pixel_fmt.normalize(dst.as_mut(), src.as_ref(), 1, 1)?;

        assert_eq!(pixel_fmt, exp_pixel_fmt);
        assert_eq!(dst, src);

        Ok(())
    }

    #[test]
    fn test_2() -> Result<()> {
        let mut dst = [0u8; 3];
        let src = [0x47, 0x9c];

        let exp_pixel_fmt = PixelFormat::Rgb888;
        let mut pixel_fmt = PixelFormat::Rgb555;

        pixel_fmt.normalize(dst.as_mut(), src.as_ref(), 1, 1)?;

        assert_eq!(pixel_fmt, exp_pixel_fmt);
        assert_eq!(
            ((dst[0] as u32 >> 3) << 10) | ((dst[1] as u32 >> 3) << 5) | (dst[2] as u32 >> 3),
            (src[0] as u32) << 8 | src[1] as u32,
        );

        Ok(())
    }

    #[test]
    fn test_3() -> Result<()> {
        let mut dst = [0u8; 3];
        let src = [0x47, 0x9c];

        let exp_pixel_fmt = PixelFormat::Rgb888;
        let mut pixel_fmt = PixelFormat::Rgb565;

        pixel_fmt.normalize(dst.as_mut(), src.as_ref(), 1, 1)?;

        assert_eq!(pixel_fmt, exp_pixel_fmt);
        assert_eq!(
            ((dst[0] as u32 >> 3) << 11) | ((dst[1] as u32 >> 2) << 5) | (dst[2] as u32 >> 3),
            (src[0] as u32) << 8 | src[1] as u32,
        );

        Ok(())
    }

    #[test]
    fn test_4() -> Result<()> {
        let mut dst = [0u8; 3];
        let src = [0x46, 0xf3, 0xe5];

        let exp_pixel_fmt = PixelFormat::Rgb888;
        let mut pixel_fmt = PixelFormat::Bgr888;

        pixel_fmt.normalize(dst.as_mut(), src.as_ref(), 1, 1)?;

        assert_eq!(pixel_fmt, exp_pixel_fmt);
        assert_eq!(
            ((dst[2] as u32) << 16) | ((dst[1] as u32) << 8) | (dst[0] as u32),
            ((src[0] as u32) << 16) | ((src[1] as u32) << 8) | (src[2] as u32),
        );

        Ok(())
    }

    #[test]
    fn test_5() -> Result<()> {
        let mut dst = [0u8; 3];
        let src = [0x23, 0xc8];

        let exp_pixel_fmt = PixelFormat::Rgb888;
        let mut pixel_fmt = PixelFormat::Bgr555;

        pixel_fmt.normalize(dst.as_mut(), src.as_ref(), 1, 1)?;

        assert_eq!(pixel_fmt, exp_pixel_fmt);
        assert_eq!(
            ((dst[2] as u32 >> 3) << 10) | ((dst[1] as u32 >> 3) << 5) | (dst[0] as u32 >> 3),
            (src[0] as u32) << 8 | src[1] as u32,
        );

        Ok(())
    }

    #[test]
    fn test_6() -> Result<()> {
        let mut dst = [0u8; 3];
        let src = [0x47, 0x88];

        let exp_pixel_fmt = PixelFormat::Rgb888;
        let mut pixel_fmt = PixelFormat::Bgr565;

        pixel_fmt.normalize(dst.as_mut(), src.as_ref(), 1, 1)?;

        assert_eq!(pixel_fmt, exp_pixel_fmt);
        assert_eq!(
            ((dst[2] as u32 >> 3) << 11) | ((dst[1] as u32 >> 2) << 5) | (dst[0] as u32 >> 3),
            (src[0] as u32) << 8 | src[1] as u32,
        );

        Ok(())
    }

    #[test]
    fn test_7() -> Result<()> {
        let mut dst = [0u8; 3];
        let src = [0x47, 0x88];

        let exp_pixel_fmt = PixelFormat::Rgb888;
        let mut pixel_fmt = PixelFormat::Ag88;

        pixel_fmt.normalize(dst.as_mut(), src.as_ref(), 1, 1)?;

        assert_eq!(pixel_fmt, exp_pixel_fmt);
        assert_eq!(dst[0], src[1]);

        Ok(())
    }

    #[test]
    fn test_8() -> Result<()> {
        let mut dst = [0u8; 3];
        let src = [0x47, 0x88];

        let exp_pixel_fmt = PixelFormat::Rgb888;
        let mut pixel_fmt = PixelFormat::Ga88;

        pixel_fmt.normalize(dst.as_mut(), src.as_ref(), 1, 1)?;

        assert_eq!(pixel_fmt, exp_pixel_fmt);
        assert_eq!(dst[0], src[0]);

        Ok(())
    }

    #[test]
    fn test_9() -> Result<()> {
        let mut dst = [0u8; 3];
        let src = [0x46, 0xf3, 0xe5, 0xf0];

        let exp_pixel_fmt = PixelFormat::Rgb888;
        let mut pixel_fmt = PixelFormat::Rgba8888;

        pixel_fmt.normalize(dst.as_mut(), src.as_ref(), 1, 1)?;

        assert_eq!(pixel_fmt, exp_pixel_fmt);
        assert_eq!(dst.as_ref(), &src[..3]);

        Ok(())
    }

    #[test]
    fn test_10() -> Result<()> {
        let mut dst = [0u8; 3];
        let src = [0x46, 0xf3, 0xe5, 0xf0];

        let exp_pixel_fmt = PixelFormat::Rgb888;
        let mut pixel_fmt = PixelFormat::Argb8888;

        pixel_fmt.normalize(dst.as_mut(), src.as_ref(), 1, 1)?;

        assert_eq!(pixel_fmt, exp_pixel_fmt);
        assert_eq!(dst.as_ref(), &src[1..]);

        Ok(())
    }
}
