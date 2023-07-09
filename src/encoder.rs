//! Types and functions for Sixel image encoding.

use alloc::{string::String, vec::Vec};

use crate::{color::*, config::*, dither::*, quant::*, std::fmt, Result};

/// Encoder for Sixel images.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Encoder {
    pub req_colors: i32,
    pub mapfile: String,
    pub methods: EncoderMethods,
    pub config: EncoderConfig,
    pub dimensions: EncoderDimensions,
    pub clip_dimensions: ClipDimensions,
    pub macro_number: i32,
    pub bg_color: Color,
    pub cancel_flag: Option<i32>,
    pub dither_cache: Vec<SixelDither>,
    pub out_file: String,
}

impl Encoder {
    /// Creates a new [Encoder].
    pub const fn new() -> Self {
        Self {
            req_colors: 0,
            mapfile: String::new(),
            methods: EncoderMethods::new(),
            config: EncoderConfig::new(),
            dimensions: EncoderDimensions::new(),
            clip_dimensions: ClipDimensions::new(),
            macro_number: 0,
            bg_color: Color::new(),
            cancel_flag: None,
            dither_cache: Vec::new(),
            out_file: String::new(),
        }
    }

    pub fn out_file(&self) -> &str {
        self.out_file.as_str()
    }

    pub fn set_out_file<F: Into<String>>(&mut self, file: F) {
        self.out_file = file.into();
    }

    /*
    #[cfg(feature = "std")]
    pub fn encode_from_file(&mut self, filename: &str) -> Result<()> {
        if self.req_colors == 0 {
            // if the color is not set, set the max value
            self.req_colors = PALETTE_MAX;
        } else if self.req_colors < 2 {
            // if required color is less than 2, set the min value
            self.req_colors = PALETTE_MIN;
        }

        // if color space option is not set, choose RGB color space
        if self.config.palette_type == PaletteType::Auto {
            self.config.palette_type = PaletteType::Rgb;
        }

        // if color option is not default value, prohibit to read the file as a paletted image
        let mut use_palette = if self.config.color_option == ColorOption::Default {
            false
        } else {
            true
        };

        // if color option is non-default, or scaling options are set, prohibit reading the file as
        // a paletted image
        let use_palette = if self.config.color_option == ColorOption::Default
            || self.dimensions.percent_width > 0
            || self.dimensions.percent_height > 0
            || self.dimensions.pixel_width > 0
            || self.dimensions.pixel_height > 0
        {
            false
        } else {
            true
        };

        let image = image::io::Reader::open(filename)?.decode()?;

        // FIXME: implement encode_image_frame
        let frame = self.encode_image_frame(&image)?;

        // FIXME: implement output_without_macro
        self.output_without_macro(&frame)?;

        Ok(())
    }
    */
}

/// Policies of Sixel encoding
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum EncodePolicy {
    /// Choose encoding policy automatically.
    #[default]
    Auto = 0,
    /// Encode as fast as possible.
    Fast,
    /// Encode to as small sixel sequence as possible.
    Size,
}

impl EncodePolicy {
    /// Creates a new [EncodePolicy].
    pub const fn new() -> Self {
        Self::Auto
    }
}

/// Configuration settings for the [Encoder].
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EncoderConfig {
    pub builtin_palette: BuiltinPalette,
    pub color_option: ColorOption,
    pub loop_mode: LoopMode,
    pub palette_type: PaletteType,
    pub eight_bit: bool,
    pub invert: bool,
    pub has_gri_arg_limit: bool,
    pub use_macro: bool,
    pub ignore_delay: bool,
    pub complexion: i32,
    pub is_static: bool,
    pub verbose: bool,
    pub insecure: bool,
    pub pipe_mode: bool,
    pub quality_mode: QualityMode,
    pub ormode: bool,
    pub penetrate_multiplexer: bool,
    pub encode_policy: EncodePolicy,
}

impl EncoderConfig {
    /// Creates a new [EncoderConfig].
    pub const fn new() -> Self {
        Self {
            builtin_palette: BuiltinPalette::new(),
            color_option: ColorOption::new(),
            loop_mode: LoopMode::new(),
            palette_type: PaletteType::new(),
            eight_bit: false,
            invert: false,
            has_gri_arg_limit: false,
            use_macro: false,
            ignore_delay: false,
            complexion: 1,
            is_static: false,
            verbose: false,
            insecure: false,
            pipe_mode: false,
            quality_mode: QualityMode::new(),
            ormode: false,
            penetrate_multiplexer: false,
            encode_policy: EncodePolicy::new(),
        }
    }
}

/// Method settings for the [Encoder].
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EncoderMethods {
    pub method_for_diffuse: MethodForDiffuse,
    pub method_for_largest: MethodForLargest,
    pub method_for_rep: MethodForRep,
    pub method_for_resampling: MethodForResampling,
}

impl EncoderMethods {
    /// Creates a new [EncoderMethods].
    pub const fn new() -> Self {
        Self {
            method_for_diffuse: MethodForDiffuse::new(),
            method_for_largest: MethodForLargest::new(),
            method_for_rep: MethodForRep::new(),
            method_for_resampling: MethodForResampling::new(),
        }
    }
}

/// Encoder palette dimensions.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EncoderDimensions {
    pub pixel_width: usize,
    pub pixel_height: usize,
    pub percent_width: usize,
    pub percent_height: usize,
}

impl EncoderDimensions {
    /// Creates a new [EncoderDimensions].
    pub const fn new() -> Self {
        Self {
            pixel_width: 0,
            pixel_height: 0,
            percent_width: 0,
            percent_height: 0,
        }
    }
}

/// Dimensions for clipping.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ClipDimensions {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    first: usize,
}

impl ClipDimensions {
    /// Creates a new [ClipDimensions].
    pub const fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            first: 0,
        }
    }
}

/// Represents callback parameters for loading context from/to a map file.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SixelCallbackContextForMapfile {
    pub req_colors: i32,
    pub dither: SixelDither,
}

impl SixelCallbackContextForMapfile {
    /// Creates a new [SixelCallbackContextForMapfile].
    pub const fn new() -> Self {
        Self {
            req_colors: 0,
            dither: SixelDither::new(),
        }
    }
}

/// Writes data to a Rust formatter.
pub fn write_callback(data: &[u8], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for c in data.iter().map(|&c| c as char) {
        write!(f, "{}", c)?;
    }

    Ok(())
}

/// Writes hex-formatted data to a Rust formatter.
pub fn hex_write_callback(data: &[u8], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for c in data.iter() {
        write!(f, "{c:x}")?;
    }

    Ok(())
}

/// Prepares a builtin monochrome color palette.
///
/// Params:
///
/// `finvert`: whether to use dark (`false`) or light (`true`) monochrome
pub fn prepare_monochrome_palette(finvert: bool) -> Result<SixelDither> {
    SixelDither::get(finvert.into())
}

/// Prepares a builtin color palette using the provided [BuiltinPalette] selector.
pub fn prepare_builtin_palette(builtin_palette: BuiltinPalette) -> Result<SixelDither> {
    SixelDither::get(builtin_palette)
}
