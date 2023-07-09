//! Types and functions for Sixel dither.

use alloc::vec::Vec;

use crate::{config::*, palette::*, pixel_format::PixelFormat, quant::*, Error, Result};

pub mod method;

/// Represents dither in a Sixel image.
#[derive(Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct SixelDither {
    pub palette: Vec<u8>,
    pub cache_table: Vec<u16>,
    pub req_colors: usize,
    pub ncolors: usize,
    pub orig_colors: usize,
    pub optimized: bool,
    pub optimize_palette: bool,
    pub complexion: i32,
    pub body_only: bool,
    pub method_for_largest: MethodForLargest,
    pub method_for_rep: MethodForRep,
    pub method_for_diffuse: MethodForDiffuse,
    pub quality_mode: QualityMode,
    pub key_color: i32,
    pub pixel_format: PixelFormat,
    pub height: usize,
    pub width: usize,
    pub depth: usize,
}

impl SixelDither {
    /// Creates a new [SixelDither].
    pub const fn new() -> Self {
        Self {
            palette: vec![],
            cache_table: vec![],
            req_colors: 256,
            ncolors: 256,
            orig_colors: 0,
            key_color: -1,
            optimized: false,
            optimize_palette: false,
            complexion: 1,
            body_only: false,
            method_for_largest: MethodForLargest::Norm,
            method_for_rep: MethodForRep::CenterBox,
            method_for_diffuse: MethodForDiffuse::Fs,
            quality_mode: QualityMode::High,
            pixel_format: PixelFormat::Rgb888,
            height: 0,
            width: 0,
            depth: 0,
        }
    }

    /// Creates a new [SixelDither] with the provided number of colors.
    pub fn create(mut ncolors: i32) -> Result<Self> {
        if ncolors == 0 || ncolors > 256 {
            Err(Error::Dither(format!(
                "sixel_dither_new: palette colors must be in range [1,256], have: {ncolors}"
            )))
        } else {
            let quality_mode = if ncolors < 0 {
                ncolors = 256;
                QualityMode::HighColor
            } else {
                QualityMode::Low
            };

            Ok(Self {
                palette: vec![],
                cache_table: vec![],
                req_colors: ncolors as usize,
                ncolors: ncolors as usize,
                orig_colors: 0,
                key_color: -1,
                optimized: false,
                optimize_palette: false,
                complexion: 1,
                body_only: false,
                method_for_largest: MethodForLargest::Norm,
                method_for_rep: MethodForRep::CenterBox,
                method_for_diffuse: MethodForDiffuse::Fs,
                quality_mode,
                pixel_format: PixelFormat::Rgb888,
                height: 0,
                width: 0,
                depth: 0,
            })
        }
    }

    /// Creates a new [SixelDither] from a predefined palette.
    pub fn get(builtin_dither: BuiltinPalette) -> Result<Self> {
        let (ncolors, palette, key_color) = match builtin_dither {
            BuiltinPalette::MonoDark => (2, PAL_MONO_DARK.as_ref(), 0),
            BuiltinPalette::MonoLight => (2, PAL_MONO_LIGHT.as_ref(), 0),
            BuiltinPalette::Xterm16 => (16, PAL_XTERM256.as_ref(), -1),
            BuiltinPalette::Xterm256 => (256, PAL_XTERM256.as_ref(), -1),
            BuiltinPalette::Vt340Mono => (16, PAL_VT340_MONO.as_ref(), -1),
            BuiltinPalette::Vt340Color => (16, PAL_VT340_COLOR.as_ref(), -1),
            BuiltinPalette::G1 => (2, PAL_GRAY_1BIT.as_ref(), -1),
            BuiltinPalette::G2 => (4, PAL_GRAY_2BIT.as_ref(), -1),
            BuiltinPalette::G4 => (16, PAL_GRAY_4BIT.as_ref(), -1),
            BuiltinPalette::G8 => (256, PAL_GRAY_8BIT.as_ref(), -1),
        };

        let mut dither = Self::create(ncolors)?;

        dither.palette = palette.into();
        dither.key_color = key_color;
        dither.optimized = true;
        dither.optimize_palette = false;

        Ok(dither)
    }

    /// Initializes the [SixelDither].
    pub fn initialize(
        &mut self,
        data: &[u8],
        width: usize,
        height: usize,
        palette_config: PaletteConfig,
    ) -> Result<()> {
        let mut normalized_pixels = vec![0u8; data.len()];

        self.pixel_format = palette_config.pixel_format;

        let input_pixels = match self.pixel_format {
            PixelFormat::Rgb888 => data,
            _ => {
                self.pixel_format
                    .normalize(normalized_pixels.as_mut(), data, width, height)?;

                normalized_pixels.as_ref()
            }
        };

        self.method_for_largest = palette_config.method_for_largest;
        self.method_for_rep = palette_config.method_for_rep;
        self.quality_mode = palette_config.quality_mode;

        let buf = self.make_palette(input_pixels)?;

        let palette_len = self.ncolors.saturating_mul(3);

        let dither_palette_len = palette_len;
        let buf_len = buf.len();

        if palette_len > dither_palette_len || palette_len > buf_len {
            Err(Error::Dither(format!("invalid palette length, palette: {dither_palette_len}, buffer: {buf_len}, have: {palette_len}")))
        } else {
            #[cfg(feature = "std")]
            println!(
                "palette len: {}, buf len: {}",
                self.palette.len(),
                buf.len()
            );
            self.palette[..palette_len].copy_from_slice(&buf[..palette_len]);
            self.optimized = true;

            if self.orig_colors <= self.ncolors {
                self.method_for_diffuse = MethodForDiffuse::None;
            }

            Ok(())
        }
    }

    /// Apply the palette to the [SixelDither].
    pub fn apply_palette(&mut self, pixels: &[u8], width: usize, height: usize) -> Result<Vec<u8>> {
        // if quality_mode is full, do not use palette caching
        if self.quality_mode == QualityMode::Full {
            self.optimized = false;
        }

        if self.cache_table.is_empty()
            && self.optimized
            && self.palette != PAL_MONO_DARK
            && self.palette != PAL_MONO_LIGHT
        {
            self.cache_table = vec![0u16; 1 << (3 * 5)];
        }

        let mut input_pixels: Vec<u8> = if self.pixel_format != PixelFormat::Rgb888 {
            let mut normalized_pixels = vec![0u8; width * height * 3];
            self.pixel_format
                .normalize(normalized_pixels.as_mut(), pixels, width, height)?;
            normalized_pixels
        } else {
            pixels.into()
        };

        self.width = width;
        self.height = height;
        self.depth = 3;

        #[cfg(feature = "std")]
        println!("pixel len: {}", input_pixels.len());

        self.quant_apply_palette(input_pixels.as_mut())
    }
}
