use alloc::vec::Vec;

use crate::{
    dither::SixelDither,
    pixel_format::PixelFormat,
    std::{cmp, fmt},
    Error, Result,
};

use super::{ColorMap, Sample};

/// Maximum number of palette items.
pub const PALETTE_MAX: usize = 256;

/// Matching enum for comparing a selected lookup function.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum LookupFunction {
    #[default]
    Normal,
    Fast,
    MonoDarkBg,
    MonoLightBg,
}

impl LookupFunction {
    /// Creates a new [LookupFunction].
    pub const fn new() -> Self {
        Self::Normal
    }
}

/// Method for finding the largest dimension for splitting, and sorting by that component.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum MethodForLargest {
    /// Choose automatically the method for finding the largest dimension
    #[default]
    Auto = 0,
    /// Simply comparing the range in RGB space
    Norm,
    /// Transforming into luminosities before the comparison
    Lum,
}

impl MethodForLargest {
    /// Creates a new [MethodForLargest].
    pub const fn new() -> Self {
        Self::Auto
    }
}

impl From<MethodForLargest> for &'static str {
    fn from(val: MethodForLargest) -> Self {
        match val {
            MethodForLargest::Auto => "auto",
            MethodForLargest::Norm => "normalized",
            MethodForLargest::Lum => "luminosity",
        }
    }
}

impl From<&MethodForLargest> for &'static str {
    fn from(val: &MethodForLargest) -> Self {
        (*val).into()
    }
}

impl fmt::Display for MethodForLargest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&str>::from(self))
    }
}

/// Method for choosing a color from the box
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum MethodForRep {
    /// Choose automatically the method for selecting representative color from each box
    #[default]
    Auto = 0,
    /// Choose the center of the box
    CenterBox,
    /// Choose the average all the color in the box (specified in Heckbert's paper)
    AverageColors,
    /// Choose the average all the pixels in the box
    AveragePixels,
}

impl MethodForRep {
    /// Creates a new [MethodForRep].
    pub const fn new() -> Self {
        Self::Auto
    }
}

impl From<MethodForRep> for &'static str {
    fn from(val: MethodForRep) -> Self {
        match val {
            MethodForRep::Auto => "auto",
            MethodForRep::CenterBox => "center box",
            MethodForRep::AverageColors => "average colors",
            MethodForRep::AveragePixels => "average pixels",
        }
    }
}

impl From<&MethodForRep> for &'static str {
    fn from(val: &MethodForRep) -> Self {
        (*val).into()
    }
}

impl fmt::Display for MethodForRep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&str>::from(self))
    }
}

/// Method for diffusing
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum MethodForDiffuse {
    /// Choose diffusion type automatically
    #[default]
    Auto = 0,
    /// Don't diffuse
    None,
    /// Diffuse with Bill Atkinson's method
    Atkinson,
    /// Diffuse with Floyd-Steinberg method
    Fs,
    /// Diffuse with Jarvis, Judice & Ninke method
    Jajuni,
    /// Diffuse with Stucki's method
    Stucki,
    /// Diffuse with Burkes' method
    Burkes,
    /// Positionally stable arithmetic dither
    ADither,
    /// Positionally stable arithmetic XOR-based dither
    XDither,
}

impl MethodForDiffuse {
    /// Creates a new [MethodForDiffuse].
    pub const fn new() -> Self {
        Self::Auto
    }

    /// Gets whether the [MethodForDiffuse] applies a mask.
    pub const fn is_mask(&self) -> bool {
        matches!(self, Self::ADither | Self::XDither)
    }
}

/// Method used for resampling
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum MethodForResampling {
    /// Use nearest neighbor method
    Nearest = 0,
    /// Use Guaussian filter
    Gaussian,
    /// Use Hanning filter
    Hanning,
    /// Use Hamming filter
    Hamming,
    /// Use bilinear filter
    #[default]
    Bilinear,
    /// Use Welsh filter
    Welsh,
    /// Use bicubic filter
    Bicubic,
    /// Use Lanczos-2 filter
    Lanczos2,
    /// Use Lanczos-3 filter
    Lanczos3,
    /// Use Lanczos-4 filter
    Lanczos4,
}

impl MethodForResampling {
    /// Creates a new [MethodForResampling].
    pub const fn new() -> Self {
        Self::Bilinear
    }
}

/// Quality modes
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum QualityMode {
    /// Choose quality mode automatically
    #[default]
    Auto = 0,
    /// High quality palette construction
    High,
    /// Low quality palette construction
    Low,
    /// Full quality palette construction
    Full,
    /// High color palette construction
    HighColor,
}

impl QualityMode {
    /// Creates a new [QualityMode].
    pub const fn new() -> Self {
        Self::Auto
    }
}

/// Groups palette configuration settings together.
pub struct PaletteConfig {
    pub pixel_format: PixelFormat,
    pub method_for_largest: MethodForLargest,
    pub method_for_rep: MethodForRep,
    pub quality_mode: QualityMode,
}

impl PaletteConfig {
    /// Creates a new [PaletteConfig].
    pub const fn new() -> Self {
        Self {
            pixel_format: PixelFormat::new(),
            method_for_largest: MethodForLargest::new(),
            method_for_rep: MethodForRep::new(),
            quality_mode: QualityMode::new(),
        }
    }
}

/// Compute a hash for a [Histogram](super::Histogram).
pub fn compute_hash(data: &[u8], depth: usize) -> usize {
    let depth = cmp::min(depth, data.len());
    let mut hash = 0;

    for (n, b) in data[..depth].iter().enumerate().rev() {
        hash |= (b.wrapping_shr(3) as usize).wrapping_shl((depth - n).saturating_mul(5) as u32);
    }

    hash
}

/// Gets the largest value by its normalized difference.
pub fn largest_by_norm(min_val: &[Sample], max_val: &[Sample]) -> usize {
    let depth = min_val.len();

    let mut largest_spread = 0;
    let mut largest_dimension = 0;

    for (plane, (&minv, &maxv)) in min_val[..depth]
        .iter()
        .zip(max_val[..depth].iter())
        .enumerate()
    {
        let delta = maxv.saturating_sub(minv);
        if delta > largest_spread {
            largest_spread = delta;
            largest_dimension = plane;
        }
    }

    largest_dimension
}

/// Gets the largest value by its luminosity.
///
/// From the `libsixel` docs:
///
/// ```no_build, no_run
/// This subroutine presumes that the tuple type is either
/// BLACKANDWHITE, GRAYSCALE, or RGB (which implies pamP->depth is 1 or 3).
/// To save time, we don't actually check it.
/// ```
///
/// `To save time, we don't actually check it.` <- except we do, because bounds are a thing.
pub fn largest_by_luminosity(min_val: &[Sample], max_val: &[Sample]) -> usize {
    let depth = min_val.len();
    let min_val_len = min_val.len();
    let max_val_len = max_val.len();

    if min_val_len != 3 || max_val_len != 3 || depth == 1 || min_val_len != max_val_len {
        0
    } else {
        let lumin_factor = [0.2989f32, 0.5866f32, 0.1145f32];
        let (mut max_lumin, mut max_dimension): (f32, usize) = (0.0, 0);

        for (plane, (&minv, &maxv)) in min_val[..3].iter().zip(max_val[..3].iter()).enumerate() {
            let lumin = lumin_factor[plane] * maxv.saturating_sub(minv) as f32;
            if lumin > max_lumin {
                max_lumin = lumin;
                max_dimension = plane;
            }
        }

        max_dimension
    }
}

/// Diffuses error energy to surrounding pixels.
pub fn error_diffuse(
    data: &mut [u8],
    pos: i32,
    depth: i32,
    error: i32,
    numerator: i32,
    denominator: i32,
    area: i32,
) -> Result<()> {
    if pos < 0 || pos >= area {
        Err(Error::Quant(format!(
            "invalid position: {pos}, area: {area}"
        )))
    } else if denominator == 0 {
        Err(Error::Quant("invalid denominator, divide-by-zero".into()))
    } else if let Some(ret) = data.get_mut((pos * depth) as usize) {
        let mut c = (*ret as i32)
            .saturating_add(error.saturating_mul(numerator).saturating_div(denominator));

        if c < 0 {
            c = 0;
        }

        if c >= 1 << 8 {
            c = (1 << 8) - 1;
        }

        *ret = c as u8;

        Ok(())
    } else {
        let data_len = data.len();
        Err(Error::Quant(format!(
            "pos({pos}) * depth({depth}) is out-of-bounds, max value: {data_len})"
        )))
    }
}

type DiffuseFn = fn(&mut [u8], i32, i32, i32, i32, i32, i32) -> Result<()>;

/// No-op diffuse function.
pub fn diffuse_none(
    _data: &mut [u8],
    _width: i32,
    _height: i32,
    _x: i32,
    _y: i32,
    _depth: i32,
    _error: i32,
) -> Result<()> {
    Ok(())
}

/// Floyd-Steinberg Method
///
/// | Col0 | Col1 | Col2 |
/// |------|------|------|
/// |      | curr | 7/16 |
/// | 3/16 | 5/48 | 1/16 |
pub fn diffuse_fs(
    data: &mut [u8],
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    depth: i32,
    error: i32,
) -> Result<()> {
    let pos = y.saturating_mul(width).saturating_add(x);

    if x < width.saturating_sub(1) && y < height.saturating_sub(1) {
        // add error to the right cell
        error_diffuse(
            data,
            pos.saturating_add(1),
            depth,
            error,
            7,
            16,
            width.saturating_mul(height),
        )?;
        // add error to the left-bottom cell
        error_diffuse(
            data,
            pos.saturating_add(width.saturating_sub(1)),
            depth,
            error,
            3,
            16,
            width.saturating_mul(height),
        )?;
        // add error to the bottom cell
        error_diffuse(
            data,
            pos.saturating_add(width),
            depth,
            error,
            5,
            16,
            width.saturating_mul(height),
        )?;
        // add error to the right-bottom cell
        error_diffuse(
            data,
            pos.saturating_add(width.saturating_add(1)),
            depth,
            error,
            1,
            16,
            width.saturating_mul(height),
        )?;
    }

    Ok(())
}

/// Atkinson's Method
///
/// | Col0 | Col1 | Col2 | Col3 |
/// |------|------|------|------|
/// |      | curr | 1/8  | 1/8  |
/// | 1/8  | 1/8  | 1/8  |      |
/// |      | 1/8  |      |      |
pub fn diffuse_atkinson(
    data: &mut [u8],
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    depth: i32,
    error: i32,
) -> Result<()> {
    let pos = y.saturating_mul(width).saturating_add(x);

    if y < height.saturating_sub(2) {
        let area = width.saturating_mul(height);

        // add error to the right cell
        error_diffuse(data, pos.saturating_add(1), depth, error, 1, 8, area)?;
        // add error to the 2nd right cell
        error_diffuse(data, pos.saturating_add(2), depth, error, 1, 8, area)?;
        // add error to the left-bottom cell
        error_diffuse(
            data,
            pos.saturating_add(width.saturating_sub(1)),
            depth,
            error,
            1,
            8,
            area,
        )?;
        // add error to the bottom cell
        error_diffuse(data, pos.saturating_add(width), depth, error, 1, 8, area)?;
        // add error to the right-bottom cell
        error_diffuse(
            data,
            pos.saturating_add(width.saturating_add(1)),
            depth,
            error,
            1,
            8,
            area,
        )?;
        // add error to the 2nd bottom cell
        error_diffuse(
            data,
            pos.saturating_add(width.saturating_mul(2)),
            depth,
            error,
            1,
            8,
            area,
        )?;
    }

    Ok(())
}

/// Jarvis, Judice & Ninks Method
///
/// | Col0 | Col1 | Col2 | Col3 | Col4 |
/// |------|------|------|------|------|
/// |      |      | curr | 7/48 | 5/48 |
/// | 3/48 | 5/48 | 7/48 | 5/48 | 3/48 |
/// | 1/48 | 3/48 | 5/48 | 3/48 | 1/48 |
pub fn diffuse_jajuni(
    data: &mut [u8],
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    depth: i32,
    error: i32,
) -> Result<()> {
    let pos = y.saturating_mul(width).saturating_add(x);

    if y < height.saturating_sub(2) {
        let area = width.saturating_mul(height);
        let width2 = width.saturating_mul(2);

        error_diffuse(data, pos.saturating_add(1), depth, error, 7, 48, area)?;
        error_diffuse(data, pos.saturating_add(2), depth, error, 5, 48, area)?;
        error_diffuse(
            data,
            pos.saturating_add(width.saturating_sub(2)),
            depth,
            error,
            3,
            48,
            area,
        )?;
        error_diffuse(
            data,
            pos.saturating_add(width.saturating_sub(1)),
            depth,
            error,
            5,
            48,
            area,
        )?;
        error_diffuse(data, pos.saturating_add(width), depth, error, 7, 48, area)?;
        error_diffuse(
            data,
            pos.saturating_add(width.saturating_add(1)),
            depth,
            error,
            5,
            48,
            area,
        )?;
        error_diffuse(
            data,
            pos.saturating_add(width.saturating_add(2)),
            depth,
            error,
            3,
            48,
            area,
        )?;
        error_diffuse(
            data,
            pos.saturating_add(width2.saturating_sub(2)),
            depth,
            error,
            1,
            48,
            area,
        )?;
        error_diffuse(
            data,
            pos.saturating_add(width2.saturating_sub(1)),
            depth,
            error,
            3,
            48,
            area,
        )?;
        error_diffuse(data, pos.saturating_add(width2), depth, error, 5, 48, area)?;
        error_diffuse(
            data,
            pos.saturating_add(width2.saturating_add(1)),
            depth,
            error,
            3,
            48,
            area,
        )?;
        error_diffuse(
            data,
            pos.saturating_add(width2.saturating_add(2)),
            depth,
            error,
            1,
            48,
            area,
        )?;
    }

    Ok(())
}

/// Stucki's Method
///
/// | Col0 | Col1 | Col2 | Col3 | Col4 |
/// |------|------|------|------|------|
/// |      |      | curr | 8/48 | 4/48 |
/// | 2/48 | 4/48 | 8/48 | 4/48 | 2/48 |
/// | 1/48 | 2/48 | 4/48 | 2/48 | 1/48 |
pub fn diffuse_stucki(
    data: &mut [u8],
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    depth: i32,
    error: i32,
) -> Result<()> {
    let pos = y.saturating_mul(width).saturating_add(x);

    if y < height.saturating_sub(2) {
        let area = width.saturating_mul(height);
        let width2 = width.saturating_mul(2);

        error_diffuse(data, pos.saturating_add(1), depth, error, 1, 6, area)?;
        error_diffuse(data, pos.saturating_add(2), depth, error, 1, 12, area)?;
        error_diffuse(
            data,
            pos.saturating_add(width.saturating_sub(2)),
            depth,
            error,
            1,
            24,
            area,
        )?;
        error_diffuse(
            data,
            pos.saturating_add(width.saturating_sub(1)),
            depth,
            error,
            1,
            12,
            area,
        )?;
        error_diffuse(data, pos.saturating_add(width), depth, error, 1, 6, area)?;
        error_diffuse(
            data,
            pos.saturating_add(width.saturating_add(1)),
            depth,
            error,
            1,
            12,
            area,
        )?;
        error_diffuse(
            data,
            pos.saturating_add(width.saturating_add(2)),
            depth,
            error,
            1,
            24,
            area,
        )?;
        error_diffuse(
            data,
            pos.saturating_add(width2.saturating_sub(2)),
            depth,
            error,
            1,
            48,
            area,
        )?;
        error_diffuse(
            data,
            pos.saturating_add(width2.saturating_sub(1)),
            depth,
            error,
            1,
            24,
            area,
        )?;
        error_diffuse(data, pos.saturating_add(width2), depth, error, 1, 12, area)?;
        error_diffuse(
            data,
            pos.saturating_add(width2.saturating_add(1)),
            depth,
            error,
            1,
            24,
            area,
        )?;
        error_diffuse(
            data,
            pos.saturating_add(width2.saturating_add(2)),
            depth,
            error,
            1,
            48,
            area,
        )?;
    }

    Ok(())
}

/// Burkes' Method
///
/// | Col0 | Col1 | Col2 | Col3 | Col4 |
/// |------|------|------|------|------|
/// |      |      | curr | 4/16 | 2/16 |
/// | 1/16 | 2/16 | 4/16 | 2/16 | 1/16 |
pub fn diffuse_burkes(
    data: &mut [u8],
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    depth: i32,
    error: i32,
) -> Result<()> {
    let pos = y.saturating_mul(width).saturating_add(x);

    if y < height.saturating_sub(2) {
        let area = width.saturating_mul(height);

        error_diffuse(data, pos.saturating_add(1), depth, error, 1, 4, area)?;
        error_diffuse(data, pos.saturating_add(2), depth, error, 1, 8, area)?;
        error_diffuse(
            data,
            pos.saturating_add(width.saturating_sub(2)),
            depth,
            error,
            1,
            16,
            area,
        )?;
        error_diffuse(
            data,
            pos.saturating_add(width.saturating_sub(1)),
            depth,
            error,
            1,
            8,
            area,
        )?;
        error_diffuse(data, pos.saturating_add(width), depth, error, 1, 4, area)?;
        error_diffuse(
            data,
            pos.saturating_add(width.saturating_add(1)),
            depth,
            error,
            1,
            8,
            area,
        )?;
        error_diffuse(
            data,
            pos.saturating_add(width.saturating_add(2)),
            depth,
            error,
            1,
            16,
            area,
        )?;
    }

    Ok(())
}

type MaskFn = fn(i32, i32, i32) -> f32;

fn mask_a(x: i32, y: i32, c: i32) -> f32 {
    let x_mul = x.saturating_add(c.saturating_mul(67));
    let y_mul = y.saturating_mul(236);

    ((x_mul.saturating_add(y_mul).saturating_mul(119) & 255) as f32) / 128f32 - 1f32
}

fn mask_x(x: i32, y: i32, c: i32) -> f32 {
    let x_mul = x.saturating_add(c.saturating_mul(29));
    let y_mul = y.saturating_mul(149);

    ((x_mul.saturating_add(y_mul).saturating_mul(1234) & 511) as f32) / 256f32 - 1f32
}

fn mask_none(_x: i32, _y: i32, _c: i32) -> f32 {
    0.0
}

type LookupFn = fn(&[u8], usize, &[u8], usize, &mut [u16], i32) -> Option<usize>;

/// No-op lookup function
pub fn lookup_none(
    _pixel: &[u8],
    _depth: usize,
    _palette: &[u8],
    _req_color: usize,
    _cache_table: &mut [u16],
    _complexion: i32,
) -> Option<usize> {
    None
}

/// Performs lookup for the closest color from palette using `"normal"` strategy.
pub fn lookup_normal(
    pixel: &[u8],
    depth: usize,
    palette: &[u8],
    req_color: usize,
    _cache_table: &mut [u16],
    complexion: i32,
) -> Option<usize> {
    let max_palette = req_color.saturating_mul(depth).saturating_add(depth);

    if palette.len() < max_palette || pixel.len() < depth || max_palette == usize::MAX {
        None
    } else {
        let mut result = None;
        let mut diff = i32::MAX;

        for i in 0..req_color {
            let r = pixel[0].saturating_sub(palette[i * depth]) as i32;

            let mut distant = r.saturating_mul(r).saturating_mul(complexion);

            for n in 1..depth {
                let r = pixel[n].saturating_sub(palette[i * depth + n]) as i32;

                distant = distant.saturating_add(r.saturating_mul(r));
            }

            if distant < diff {
                diff = distant;
                result = Some(i);
            }
        }

        result
    }
}

/// Performs lookup for the closest color from palette using `"fast"` strategy.
pub fn lookup_fast(
    pixel: &[u8],
    _depth: usize,
    palette: &[u8],
    req_color: usize,
    cache_table: &mut [u16],
    complexion: i32,
) -> Option<usize> {
    let depth = 3;
    let max_palette = req_color.saturating_mul(depth).saturating_add(depth);

    if palette.len() < max_palette || pixel.len() < depth {
        None
    } else {
        let hash = compute_hash(pixel, depth);
        let cache = cache_table.get(hash).unwrap_or(&0);

        if *cache > 0 {
            // fast lookup
            Some((cache - 1) as usize)
        } else {
            let mut result = None;
            let mut diff = i32::MAX;

            for i in 0..req_color {
                // complexion correction
                let p0 = (pixel[0] as i32)
                    .saturating_sub(palette[i * depth] as i32)
                    .saturating_mul(pixel[0].saturating_sub(palette[i * depth]) as i32)
                    .saturating_mul(complexion);

                let p1 = (pixel[1] as i32)
                    .saturating_sub(palette[i * depth + 1] as i32)
                    .saturating_mul(pixel[1].saturating_sub(palette[i * depth + 1]) as i32);

                let p2 = (pixel[2] as i32)
                    .saturating_sub(palette[i * depth + 1] as i32)
                    .saturating_mul(pixel[2].saturating_sub(palette[i * depth + 2]) as i32);

                let distant = p0.saturating_add(p1).saturating_add(p2);

                if distant < diff {
                    diff = distant;
                    result = Some(i);
                }
            }

            if let Some(r) = result {
                cache_table[hash] = (r + 1) as u16;
            }

            result
        }
    }
}

/// Performs lookup for the closest color from mono dark background palette.
pub fn lookup_mono_darkbg(
    pixel: &[u8],
    depth: usize,
    _palette: &[u8],
    req_color: usize,
    _cache_table: &mut [u16],
    _complexion: i32,
) -> Option<usize> {
    let distant: usize = pixel[..depth].iter().map(|&p| p as usize).sum();

    if distant >= req_color.saturating_mul(128) {
        Some(1)
    } else {
        Some(0)
    }
}

/// Performs lookup for the closest color from mono light background palette.
pub fn lookup_mono_lightbg(
    pixel: &[u8],
    depth: usize,
    _palette: &[u8],
    req_color: usize,
    _cache_table: &mut [u16],
    _complexion: i32,
) -> Option<usize> {
    let distant: usize = pixel[..depth].iter().map(|&p| p as usize).sum();

    if distant < req_color.saturating_mul(128) {
        Some(1)
    } else {
        Some(0)
    }
}

impl SixelDither {
    const MAX_DEPTH: usize = 4;

    /// Creates a color palette.
    pub fn make_palette(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        let depth = self.pixel_format.compute_depth();

        let color_map = ColorMap::compute_from_input(
            data,
            depth,
            self.req_colors,
            self.method_for_largest,
            self.method_for_rep,
            self.quality_mode,
            &mut self.orig_colors,
        )?;

        self.ncolors = color_map.table.len();

        let mut result: Vec<u8> = Vec::with_capacity(self.ncolors.wrapping_mul(depth));

        let colors = self.ncolors;
        for t in color_map.table[..colors].iter() {
            for &tuple in t.tuple[..depth].iter() {
                result.push(tuple as u8);
            }
        }

        Ok(result)
    }

    /// Applies color palette into the specified pixel buffers.
    pub fn quant_apply_palette(&mut self, data: &mut [u8]) -> Result<Vec<u8>> {
        let (height, width, depth) = (self.height, self.width, self.depth);

        if self.req_colors < 1 {
            Err(Error::Quant("apply_palette: req_color == 0".into()))
        } else if depth > Self::MAX_DEPTH {
            let max = Self::MAX_DEPTH;

            Err(Error::Quant(format!(
                "apply_palette: depth is out-of-bounds: {depth}, max: {max}"
            )))
        } else {
            let mut result = vec![0u8; width * height * depth];

            let (f_mask, f_diffuse): (MaskFn, DiffuseFn) = match self.method_for_diffuse {
                MethodForDiffuse::None => (mask_none, diffuse_none),
                MethodForDiffuse::Atkinson => (mask_none, diffuse_atkinson),
                MethodForDiffuse::Fs => (mask_none, diffuse_fs),
                MethodForDiffuse::Jajuni => (mask_none, diffuse_jajuni),
                MethodForDiffuse::Stucki => (mask_none, diffuse_stucki),
                MethodForDiffuse::Burkes => (mask_none, diffuse_burkes),
                MethodForDiffuse::ADither => (mask_a, diffuse_none),
                MethodForDiffuse::XDither => (mask_x, diffuse_none),
                MethodForDiffuse::Auto => {
                    return Err(Error::Quant(
                        "apply_palette: invalid method_for_diffuse(Auto)".into(),
                    ))
                }
            };

            let (f_lookup, lookup_fn): (LookupFn, LookupFunction) = if self.req_colors == 2 {
                let sum1: u32 = self.palette[..depth].iter().map(|&p| u32::from(p)).sum();
                let sum2: u32 = self.palette[..depth + depth]
                    .iter()
                    .map(|&p| u32::from(p))
                    .sum();

                if sum1 == 0 && sum2 == 255 * 3 {
                    (lookup_mono_darkbg, LookupFunction::MonoDarkBg)
                } else if sum1 == 255 * 3 && sum2 == 0 {
                    (lookup_mono_lightbg, LookupFunction::MonoLightBg)
                } else if self.optimized && depth == 3 {
                    (lookup_fast, LookupFunction::Fast)
                } else {
                    (lookup_normal, LookupFunction::Normal)
                }
            } else if self.optimized && depth == 3 {
                (lookup_fast, LookupFunction::Fast)
            } else {
                (lookup_normal, LookupFunction::Normal)
            };

            let mut index_table = self.cache_table.clone();
            if index_table.is_empty() && lookup_fn == LookupFunction::Fast {
                index_table.resize(1usize << (depth * 5), 0u16);
            }

            if self.optimize_palette {
                self.ncolors = 0;

                let mut new_palette = vec![0u8; PALETTE_MAX * depth];
                let mut migration_map = vec![0u16; PALETTE_MAX];

                let max_area = height.saturating_mul(width).saturating_add(width);
                let max_depth = max_area.saturating_mul(depth).saturating_add(depth);
                let data_len = data.len();

                if data_len < max_area || data_len < max_depth {
                    return Err(Error::Quant(format!("apply_palette: max area and/or depth are out-of-bounds: max area: {max_area}, max depth: {max_depth}, data length: {data_len}")));
                }

                if self.method_for_diffuse.is_mask() {
                    self.apply_mask_optimize(
                        data,
                        new_palette.as_mut(),
                        result.as_mut(),
                        migration_map.as_mut(),
                        index_table.as_mut(),
                        (f_mask, f_lookup),
                    )?;
                } else {
                    self.apply_no_mask_optimize(
                        data,
                        new_palette.as_mut(),
                        result.as_mut(),
                        migration_map.as_mut(),
                        index_table.as_mut(),
                        (f_lookup, f_diffuse),
                    )?;
                }
            } else {
                if self.method_for_diffuse.is_mask() {
                    self.apply_mask(
                        data,
                        result.as_mut(),
                        index_table.as_mut(),
                        (f_mask, f_lookup),
                    )?;
                } else {
                    self.apply_no_mask(
                        data,
                        result.as_mut(),
                        index_table.as_mut(),
                        (f_lookup, f_diffuse),
                    )?;
                }

                self.ncolors = self.req_colors;
            }

            self.cache_table = index_table;

            Ok(result)
        }
    }

    fn apply_mask_optimize(
        &mut self,
        data: &mut [u8],
        new_palette: &mut [u8],
        result: &mut [u8],
        migration_map: &mut [u16],
        index_table: &mut [u16],
        apply_fns: (MaskFn, LookupFn),
    ) -> Result<()> {
        let (height, width, depth) = (self.height, self.width, self.depth);
        let (f_mask, f_lookup) = apply_fns;

        for y in 0..height {
            for x in 0..width {
                let mut copy = vec![0u8; Self::MAX_DEPTH];

                let pos = y * width + x;
                for d in 0..depth {
                    let val = data[pos * depth + d] as f32
                        + (f_mask(x as i32, y as i32, d as i32) * 32.0);
                    copy[d] = if val.is_nan() || val.is_infinite() || val < 0.0 {
                        0
                    } else if val > 255.0 {
                        255
                    } else {
                        val as u8
                    }
                }

                let color_index = f_lookup(
                    copy.as_ref(),
                    depth,
                    self.palette.as_mut(),
                    self.req_colors,
                    index_table,
                    self.complexion,
                )
                .unwrap_or(0);

                if migration_map[color_index] == 0 {
                    result[pos] = self.ncolors as u8;
                    for n in 0..depth {
                        new_palette[self.ncolors * depth + n] =
                            self.palette[color_index * depth + n];
                    }
                    self.ncolors += 1;
                    migration_map[color_index] = self.ncolors as u16;
                } else {
                    result[pos] = (migration_map[color_index] - 1) as u8;
                }
            }
        }

        let copy_len = self.ncolors * depth;
        self.palette[..copy_len].copy_from_slice(new_palette[..copy_len].as_ref());

        Ok(())
    }

    fn apply_no_mask_optimize(
        &mut self,
        data: &mut [u8],
        new_palette: &mut [u8],
        result: &mut [u8],
        migration_map: &mut [u16],
        index_table: &mut [u16],
        apply_fns: (LookupFn, DiffuseFn),
    ) -> Result<()> {
        let (height, width, depth) = (self.height, self.width, self.depth);
        let (f_lookup, f_diffuse) = apply_fns;

        for y in 0..height {
            for x in 0..width {
                let pos = y * width + x;
                let color_index = f_lookup(
                    data[pos * depth..].as_mut(),
                    depth,
                    self.palette.as_mut(),
                    self.req_colors,
                    index_table,
                    self.complexion,
                )
                .unwrap_or(0);

                if migration_map[color_index] == 0 {
                    result[pos] = self.ncolors as u8;

                    for n in 0..depth {
                        new_palette[self.ncolors * depth + n] =
                            self.palette[color_index * depth + n];
                    }
                    self.ncolors += 1;
                    migration_map[color_index] = self.ncolors as u16;
                } else {
                    result[pos] = (migration_map[color_index] - 1) as u8;
                }

                for n in 0..depth {
                    let offset =
                        data[pos * depth + n].saturating_sub(self.palette[color_index * depth + n]);

                    f_diffuse(
                        data[n..].as_mut(),
                        width as i32,
                        height as i32,
                        x as i32,
                        y as i32,
                        depth as i32,
                        offset as i32,
                    )?;
                }
            }
        }

        let copy_len = self.ncolors * depth;
        self.palette[..copy_len].copy_from_slice(new_palette[..copy_len].as_ref());

        Ok(())
    }

    fn apply_mask(
        &mut self,
        data: &mut [u8],
        result: &mut [u8],
        index_table: &mut [u16],
        apply_fns: (MaskFn, LookupFn),
    ) -> Result<()> {
        let (height, width, depth) = (self.height, self.width, self.depth);
        let (f_mask, f_lookup) = apply_fns;

        for y in 0..height {
            for x in 0..width {
                let mut copy = vec![0u8; Self::MAX_DEPTH];
                let pos = y * width + x;
                for d in 0..depth {
                    let val =
                        data[pos * depth + d] as f32 + f_mask(x as i32, y as i32, d as i32) * 32.0;
                    copy[d] = if val.is_nan() || val.is_infinite() || val < 0.0 {
                        0
                    } else if val > 255.0 {
                        255
                    } else {
                        val as u8
                    };

                    result[pos] = f_lookup(
                        copy.as_ref(),
                        depth,
                        self.palette.as_mut(),
                        self.req_colors,
                        index_table,
                        self.complexion,
                    )
                    .unwrap_or(0) as u8;
                }
            }
        }

        Ok(())
    }

    fn apply_no_mask(
        &mut self,
        data: &mut [u8],
        result: &mut [u8],
        index_table: &mut [u16],
        apply_fns: (LookupFn, DiffuseFn),
    ) -> Result<()> {
        let (height, width, depth) = (self.height, self.width, self.depth);
        let (f_lookup, f_diffuse) = apply_fns;

        for y in 0..height {
            for x in 0..width {
                let pos = y * width + x;
                let color_index = f_lookup(
                    data[pos * depth..].as_ref(),
                    depth,
                    self.palette.as_mut(),
                    self.req_colors,
                    index_table,
                    self.complexion,
                )
                .unwrap_or(0);

                result[pos] = color_index as u8;

                for n in 0..depth {
                    let offset =
                        data[pos * depth + n].saturating_sub(self.palette[color_index * depth + n]);

                    f_diffuse(
                        data[n..].as_mut(),
                        width as i32,
                        height as i32,
                        x as i32,
                        y as i32,
                        depth as i32,
                        offset as i32,
                    )?;
                }
            }
        }

        Ok(())
    }
}
