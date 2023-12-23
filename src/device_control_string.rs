//! Types and functions for encoding Sixel image control strings.

use alloc::vec::Vec;

use crate::std::sync::atomic::{AtomicU8, AtomicUsize, Ordering};
use crate::{dimension::*, dither::*, quant::MethodForDiffuse, std::fmt, Error, Result};

mod color_map;
mod constants;
mod raster;
mod repeat;
mod scrolling_mode;
mod selector;
mod sixel_char;
mod sixel_control;

pub use color_map::*;
pub use constants::*;
pub use raster::*;
pub use repeat::*;
pub use scrolling_mode::*;
pub use selector::*;
pub use sixel_char::*;
pub use sixel_control::*;

static CUR_CHAR: AtomicU8 = AtomicU8::new(0);
static CUR_COLOR: AtomicUsize = AtomicUsize::new(0);
static SAVE_CHAR_COUNT: AtomicUsize = AtomicUsize::new(0);

const COLOR_THRESHOLD: usize = 24;

fn current_char() -> SixelChar {
    CUR_CHAR.load(Ordering::Relaxed).into()
}

fn set_current_char(sixel_char: SixelChar) {
    CUR_CHAR.store(sixel_char.into(), Ordering::SeqCst);
}

fn current_color() -> BasicColor {
    CUR_COLOR.load(Ordering::Relaxed).into()
}

fn set_current_color(color: usize) {
    CUR_COLOR.store(color, Ordering::SeqCst);
}

fn save_char_count() -> usize {
    SAVE_CHAR_COUNT.load(Ordering::Relaxed)
}

fn set_save_char_count(count: usize) {
    SAVE_CHAR_COUNT.store(count, Ordering::SeqCst);
}

fn increment_save_char_count() {
    SAVE_CHAR_COUNT.store(save_char_count() + 1, Ordering::SeqCst);
}

/// Variants of valid Sixel data structures.
pub enum SixelItem {
    Color(BasicColor),
    Char(SixelChar),
    Raster(Raster),
    Repeat(Repeat),
    Control(SixelControl),
}

impl fmt::Display for SixelItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Color(data) => write!(f, "{data}"),
            Self::Char(data) => write!(f, "{data}"),
            Self::Raster(data) => write!(f, "{data}"),
            Self::Repeat(data) => write!(f, "{data}"),
            Self::Control(data) => write!(f, "{data}"),
        }
    }
}

/// Common counts for Sixel encoding.
pub struct SixelCounts {
    pub rows: usize,
    pub sixel_len: usize,
    pub sixel_rem: usize,
    pub rgb_len: usize,
    pub hits: Vec<u8>,
    pub threshold: u8,
}

impl SixelCounts {
    /// Creates a new [SixelCounts].
    pub const fn new() -> Self {
        Self {
            rows: 0,
            sixel_len: 0,
            sixel_rem: 0,
            rgb_len: 0,
            hits: Vec::new(),
            threshold: 1,
        }
    }

    /// Creates a new [SixelCounts] with the provided counts.
    pub fn create(
        rows: usize,
        sixel_len: usize,
        sixel_rem: usize,
        rgb_len: usize,
        pixels: usize,
    ) -> Self {
        Self {
            rows,
            sixel_len,
            sixel_rem,
            rgb_len,
            hits: vec![0; pixels],
            threshold: 1,
        }
    }
}

// Collection of common sixel state for passing to inner functions.
struct SixelState<'s> {
    pub pixels: &'s mut [u8],
    pub sixel_data: &'s mut Vec<SixelItem>,
    pub color_map: &'s mut ColorMap,
    pub cur_color: &'s mut [u8; 3],
    pub pal_idx: &'s mut usize,
}

/// Device Control String
///
/// The format for the device control string is as follows.
///
/// | DCS | P1   | ;    | P2;  | P3;  |  q   | s..s  | ST   |
/// |-----|------|------|------|------|------|-------|------|
/// | 9/0 | `**` | 3/11 | `**` | `**` | 7/1  | `***` | 9/12 |
///
/// where:
///
/// - **DCS** is a C1 control character that introduces the sixel data sequence. You can also express DCS as the 7-bit escape sequence ESC P for a 7-bit environment.
/// - **P1** is the macro parameter. This parameter indicates the pixel aspect ratio used by the application or terminal. The pixel aspect ratio defines the shape of the pixel dots the terminal uses to draw images. For example, a pixel that is twice as high as it is wide has as aspect ratio of 2:1. The following list shows the values you can use for P1.
/// - **;** is a semicolon (3/11). This character separates numeric parameters in a DCS string.
/// - **P2** selects how the terminal draws the background color. You can use one of three values.
/// - **P3** is the horizontal grid size parameter. The horizontal grid size is the horizontal distance between two pixel dots. The VT300 ignores this parameter because the horizontal grid size is fixed at 0.0195 cm (0.0075 in).
/// - **q** indicates that this device control string is a sixel command.
/// - **s...s** is the sixel-encoded data string. The sixel data characters are characters in the range of ? (hex 3F) to ~ (hex 7E). Each sixel data character represents six vertical pixels of data. Each sixel data character represents a binary value equal to the character code value minus hex 3F.
/// - **ST** is the string terminator. ST is a C1 control character. You can also express ST as the 7-bit escape sequence ESC \ for a 7-bit environment ([`DcsMode::SevenBit`](DcsMode::SevenBit)).
///
/// **s..s** sixel encoding is typically preceded by color map definitions. This is useful for
/// using the [`BasicColor`](SixelControl::Color) selector sequence in the sixel data.
///
/// See [`ColorIntroducer`](SixelControl::Color) for more details.
pub struct DeviceControlString {
    mode: DcsMode,
    ratio: PixelAspectRatio,
    background_color: DcsBackground,
    raster: Raster,
    color_map: ColorMap,
    sixel_data: Vec<SixelItem>,
}

impl DeviceControlString {
    /// Creates a new [DeviceControlString].
    pub const fn new() -> Self {
        Self {
            mode: DcsMode::new(),
            ratio: PixelAspectRatio::new(),
            background_color: DcsBackground::new(),
            raster: Raster::new(),
            color_map: ColorMap::new(),
            sixel_data: Vec::new(),
        }
    }

    /// Gets the [DcsMode] selector.
    pub const fn mode(&self) -> DcsMode {
        self.mode
    }

    /// Sets the [DcsMode] selector.
    pub fn set_mode(&mut self, mode: DcsMode) {
        self.mode = mode;
    }

    /// Builder function that sets the [DcsMode] selector.
    pub fn with_mode(mut self, mode: DcsMode) -> Self {
        self.set_mode(mode);
        self
    }

    /// Gets the [PixelAspectRatio] selector.
    pub const fn ratio(&self) -> PixelAspectRatio {
        self.ratio
    }

    /// Sets the [PixelAspectRatio] selector.
    pub fn set_ratio(&mut self, ratio: PixelAspectRatio) {
        self.ratio = ratio;
    }

    /// Builder function that sets the [PixelAspectRatio] selector.
    pub fn with_ratio(mut self, ratio: PixelAspectRatio) -> Self {
        self.set_ratio(ratio);
        self
    }

    /// Gets the [DcsBackground] selector.
    pub const fn background_color(&self) -> DcsBackground {
        self.background_color
    }

    /// Sets the [DcsBackground] selector.
    pub fn set_background_color(&mut self, background_color: DcsBackground) {
        self.background_color = background_color;
    }

    /// Builder function that sets the [DcsBackground] selector.
    pub fn with_background_color(mut self, background_color: DcsBackground) -> Self {
        self.set_background_color(background_color);
        self
    }

    /// Gets the [Raster] attributes.
    pub const fn raster(&self) -> Raster {
        self.raster
    }

    /// Sets the [Raster] attributes.
    pub fn set_raster(&mut self, raster: Raster) {
        self.raster = raster;
    }

    /// Builder function that sets the [Raster] attributes.
    pub fn with_raster(mut self, raster: Raster) -> Self {
        self.set_raster(raster);
        self
    }

    /// Gets the [ColorMap].
    pub const fn color_map(&self) -> &ColorMap {
        &self.color_map
    }

    /// Sets the [ColorMap].
    pub fn set_color_map(&mut self, color_map: ColorMap) {
        self.color_map = color_map;
    }

    /// Builder function that sets the [ColorMap].
    pub fn with_color_map(mut self, color_map: ColorMap) -> Self {
        self.set_color_map(color_map);
        self
    }

    /// Gets the [SixelItem] list.
    pub fn sixel_data(&self) -> &[SixelItem] {
        self.sixel_data.as_ref()
    }

    /// Sets the [SixelItem] list.
    pub fn set_sixel_data<S: Into<Vec<SixelItem>>>(&mut self, sixel_data: S) {
        self.sixel_data = sixel_data.into();
    }

    /// Builder function that sets the [SixelItem] list.
    pub fn with_sixel_data<S: Into<Vec<SixelItem>>>(mut self, sixel_data: S) -> Self {
        self.set_sixel_data(sixel_data);
        self
    }

    /// Converts RGB pixel data into [DeviceControlString].
    pub fn from_rgb(
        pixels: &mut [u8],
        width: usize,
        height: usize,
        diffuse_method: MethodForDiffuse,
    ) -> Result<Self> {
        let depth = 3;
        let area = width.saturating_mul(height).saturating_mul(depth);
        let pixel_len = pixels.len();

        if pixel_len < area {
            Err(Error::Output(format!("invalid area [width*height*depth(3)] ({area}) is out-of-bounds, pixel buffer length ({pixel_len})")))
        } else if width == 0 || height == 0 {
            Err(Error::Output(format!(
                "invalid width/height, must be non-zero | width: {width}, height: {height}"
            )))
        } else {
            let mut color_map = ColorMap::new();
            // Add black as the first palette color
            color_map.inner_mut().push(ColorMapItem::new());

            let mut pal_idx = 0;
            let mut cur_color = [0, 0, 0];

            // RGB pixels are three bytes wide
            let rgb_len = width.saturating_mul(3);

            // length of six rows of pixels, one sixel == six pixels
            // width * 6 * depth(3)
            let sixel_len = rgb_len.saturating_mul(6);
            let sixel_rows = area / sixel_len;
            let sixel_rem = area % sixel_len;

            let max_sixels = width.saturating_mul(sixel_rows).saturating_add(sixel_rem);
            let mut sixel_data = Vec::with_capacity(max_sixels);

            let mut sixel_counts = SixelCounts::create(
                sixel_rows,
                sixel_len,
                sixel_rem,
                rgb_len,
                width.saturating_mul(height),
            );

            let mut sd = SpaceDimension {
                width,
                height,
                ..Default::default()
            };

            let mut state = SixelState {
                pixels,
                sixel_data: &mut sixel_data,
                color_map: &mut color_map,
                cur_color: &mut cur_color,
                pal_idx: &mut pal_idx,
            };

            state.build_color_map();

            state.encode_body(&mut sd, &mut sixel_counts, diffuse_method)?;

            Ok(Self::new()
                .with_background_color(DcsBackground::Background)
                .with_color_map(color_map)
                .with_raster(Raster::create(1, 1, width, height))
                .with_sixel_data(sixel_data))
        }
    }
}

impl<'s> SixelState<'s> {
    fn build_color_map(&mut self) {
        let mut counts: hashbrown::HashMap<u16, usize> = hashbrown::HashMap::with_capacity(1 << 15);

        self.pixels.chunks_exact_mut(3).for_each(|c| {
            // take upper five bits from each RGB value, and combine them into a single u16
            c[0] &= 0xf8;
            c[1] &= 0xf8;
            c[2] &= 0xf8;

            let idx = ((c[0] as u16) << 7) | ((c[1] as u16) << 2) | (c[2] >> 3) as u16;

            let count = counts.entry(idx).or_insert(0);
            *count += 1;
        });

        for (key, val) in counts.iter() {
            let item = ColorMapItem::create_rgb(
                0,
                ((key & 0b111_1100_0000_0000) >> 7) as u8,
                ((key & 0b11_1110_0000) >> 2) as u8,
                ((key & 0b11_111) << 3) as u8,
            );

            let has_item = self
                .color_map
                .items()
                .iter()
                .any(|c| c.x() == item.x() && c.y() == item.y() && c.z() == item.z());

            if val >= &COLOR_THRESHOLD && !has_item {
                let idx = self.color_map.items().len();
                self.color_map.inner_mut().push(item.with_number(idx));
            }
        }
    }

    fn encode_body(
        &mut self,
        sd: &mut SpaceDimension,
        sixel_counts: &mut SixelCounts,
        diffuse_method: MethodForDiffuse,
    ) -> Result<()> {
        let (sixel_rows, sixel_len, sixel_rem) = (
            sixel_counts.rows,
            sixel_counts.sixel_len,
            sixel_counts.sixel_rem,
        );

        for row in 0..sixel_rows {
            let row_idx = row * sixel_len;

            // iterate over each row in a sixel plane to output the color of the row pixel
            for six_idx in 0..6 {
                self.encode_row(sd, sixel_counts, six_idx, row_idx, diffuse_method);

                put_flash(self.sixel_data);

                if six_idx < 5 {
                    // overwrite the row for each sixel index
                    self.sixel_data
                        .push(SixelItem::Control(SixelControl::CarriageReturn));
                }
            }

            self.sixel_data
                .push(SixelItem::Control(SixelControl::NewLine));
        }

        // encode any remaining pixel rows
        if sixel_rem != 0 {
            for six_idx in 0..sixel_rem {
                self.encode_remainder_row(sd, sixel_counts, six_idx, diffuse_method);

                put_flash(self.sixel_data);

                if six_idx < sixel_rem - 1 {
                    // overwrite the row for each sixel index
                    self.sixel_data
                        .push(SixelItem::Control(SixelControl::CarriageReturn));
                }
            }

            self.sixel_data
                .push(SixelItem::Control(SixelControl::NewLine));
        }

        Ok(())
    }

    fn encode_row(
        &mut self,
        sd: &mut SpaceDimension,
        sixel_counts: &mut SixelCounts,
        six_idx: usize,
        row_idx: usize,
        diffuse_method: MethodForDiffuse,
    ) -> bool {
        let mut overwrite = false;

        for col_idx in (0..sixel_counts.rgb_len).step_by(3) {
            sd.x = col_idx;
            sd.y = row_idx;

            let pos =
                (row_idx + col_idx).saturating_add(six_idx.saturating_mul(sixel_counts.sixel_len));
            if pos.saturating_add(sixel_counts.sixel_len) < self.pixels.len() {
                method::apply_15bpp_dither(&mut self.pixels[pos..], *sd, diffuse_method).ok();
            }

            // sixels are six pixels high, one pixel wide
            // calculate offsets for each row in the sixel column
            let off = row_idx + col_idx;

            overwrite |= self.inner_encode(sixel_counts, six_idx, off);
        }

        overwrite
    }

    fn inner_encode(&mut self, sixel_counts: &mut SixelCounts, six_idx: usize, off: usize) -> bool {
        let rgb_len = sixel_counts.rgb_len;
        let offset = off + (rgb_len * six_idx);

        let (r, g, b) = (
            self.pixels[offset],
            self.pixels[offset + 1],
            self.pixels[offset + 2],
        );

        let mut color_item = ColorMapItem::create_rgb(0, r, g, b);

        if color_item.is_black() {
            *self.pal_idx = 0;
        } else {
            // check for a color change
            let mut delta = i16::MAX;

            // search for the closest color map entry to the current RGB value
            self.color_map.items().iter().for_each(|c| {
                let id = (color_item.x() as i16 - c.x() as i16).abs()
                    + (color_item.y() as i16 - c.y() as i16).abs()
                    + (color_item.z() as i16 - c.z() as i16).abs();

                if id <= delta {
                    delta = id;
                    color_item.set_number(c.number());
                }
            });

            let idx = color_item.number();
            if idx != 0 {
                *self.pal_idx = idx;
            }
        }

        // use hit counts to not overwrite a pixel that has already been written
        // **NOTE** divide the pixel offset by three to account for RGB width
        let sixel_char = if color_item.is_black() {
            SixelChar::full()
        } else {
            SixelChar::from_index(six_idx)
        };
        put_sixel_char(self.sixel_data, sixel_char, *self.pal_idx);

        true
    }

    fn encode_remainder_row(
        &mut self,
        sd: &mut SpaceDimension,
        sixel_counts: &SixelCounts,
        six_idx: usize,
        diffuse_method: MethodForDiffuse,
    ) -> bool {
        let (sixel_rows, sixel_len, sixel_rem, rgb_len) = (
            sixel_counts.rows,
            sixel_counts.sixel_len,
            sixel_counts.sixel_rem,
            sixel_counts.rgb_len,
        );
        let row_idx = sixel_rows * sixel_len;

        let mut overwrite = false;

        for col_idx in (0..rgb_len).step_by(3) {
            sd.x = col_idx;
            sd.y = row_idx;

            let pos =
                (row_idx + col_idx).saturating_add(six_idx.saturating_mul(sixel_counts.sixel_len));
            if pos < self.pixels.len() {
                method::apply_15bpp_dither(&mut self.pixels[pos..], *sd, diffuse_method).ok();
            }

            // sixels are six pixels high, one pixel wide
            // calculate offsets for each row in the sixel column
            let p1_off = row_idx * col_idx;
            let p2_off = row_idx * col_idx + rgb_len;
            let p3_off = row_idx * col_idx + (rgb_len * 2);
            let p4_off = row_idx * col_idx + (rgb_len * 3);
            let p5_off = row_idx * col_idx + (rgb_len * 4);

            let sixel_plane = [
                [
                    self.pixels[p1_off],
                    self.pixels[p1_off + 1],
                    self.pixels[p1_off + 2],
                ],
                if sixel_rem >= 2 {
                    [
                        self.pixels[p2_off],
                        self.pixels[p2_off + 1],
                        self.pixels[p2_off + 2],
                    ]
                } else {
                    [0, 0, 0]
                },
                if sixel_rem >= 3 {
                    [
                        self.pixels[p3_off],
                        self.pixels[p3_off + 1],
                        self.pixels[p3_off + 2],
                    ]
                } else {
                    [0, 0, 0]
                },
                if sixel_rem >= 4 {
                    [
                        self.pixels[p4_off],
                        self.pixels[p4_off + 1],
                        self.pixels[p4_off + 2],
                    ]
                } else {
                    [0, 0, 0]
                },
                if sixel_rem >= 5 {
                    [
                        self.pixels[p5_off],
                        self.pixels[p5_off + 1],
                        self.pixels[p5_off + 2],
                    ]
                } else {
                    [0, 0, 0]
                },
                [0, 0, 0],
            ];

            // use hit counts to not overwrite a pixel that has already been written
            // **NOTE** divide the pixel offset by three to account for RGB width
            let sixel_char = SixelChar::from_plane(
                &sixel_plane,
                six_idx,
                &sixel_counts.hits[(p1_off / 3)..],
                sixel_counts.threshold,
            );
            put_sixel_char(self.sixel_data, sixel_char, *self.pal_idx);

            overwrite |= sixel_plane
                .iter()
                .map(|c| (c != self.cur_color) as u8)
                .sum::<u8>()
                != 0;
        }

        overwrite
    }
}

fn put_sixel_char(sixel_data: &mut Vec<SixelItem>, sixel_char: SixelChar, color_idx: usize) {
    let cur_char = current_char();
    let color = BasicColor::from(color_idx);

    // check for a color change
    let cur_color = current_color();
    let color_change = cur_color != color;
    if color_change {
        log::trace!("Changing color: current {cur_color}, new {color}");
        sixel_data.push(SixelItem::Color(color));
        set_current_color(color.into());
    }

    if cur_char == sixel_char && !color_change {
        increment_save_char_count();

        let count = save_char_count();
        log::trace!("Incrementing current character: char {cur_char}, count {count}");
    } else {
        let count = save_char_count();
        log::trace!("Flashing character from put_sixel_char: char {sixel_char}, count {count}, saved char {cur_char}");

        put_flash(sixel_data);

        set_current_char(sixel_char);
        set_save_char_count(1);

        let cur_char = current_char();
        log::trace!("Flashed character from put_sixel_char, new saved char: {cur_char}");
    }
}

fn put_flash(sixel_data: &mut Vec<SixelItem>) {
    let cur_char = current_char();
    let save_count = save_char_count();

    if save_count >= 3 {
        log::trace!("Outputting repeat: count {save_count}, char {cur_char}");
        sixel_data.push(SixelItem::Repeat(Repeat::create(save_count, cur_char)));
    } else {
        log::trace!("Outputting manual repeat: count {save_count}, char {cur_char}");
        for _ in 0..save_count {
            sixel_data.push(SixelItem::Char(cur_char));
        }
    }

    set_current_char(SixelChar::new());
    set_save_char_count(0);
}

impl fmt::Display for DeviceControlString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dcs = DcsFunction::from(self.mode);
        let st = StFunction::from(self.mode);

        write!(f, "{dcs}q{}", self.raster)?;
        write!(f, "{}", self.color_map)?;
        for s in self.sixel_data.iter() {
            write!(f, "{s}")?;
        }
        write!(f, "{st}")
    }
}
