//! Types and functions for encoding Sixel image control strings.

use alloc::vec::Vec;

use crate::{
    dimension::*, dither::*, palette::*, quant::MethodForDiffuse, std::fmt, Error, Result,
};

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
    // FIXME: getting closer, but it still looks like shit.
    // Need better "hit" detection, and encoding per layer color.
    pub fn from_rgb(pixels: &mut [u8], width: usize, height: usize) -> Result<Self> {
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
            // FIXME: take palette as a parameter
            let color_map = ColorMap::from(PAL_XTERM256);

            // RGB pixels are three bytes wide
            let rgb_len = width.saturating_mul(3);
            // length of six rows of pixels, one sixel == six pixels
            // width * 6 * depth(3)
            let sixel_len = rgb_len.saturating_mul(6);
            let sixel_rows = area / sixel_len;
            let sixel_rem = area % sixel_len;
            let max_sixels = width.saturating_mul(sixel_rows).saturating_add(sixel_rem);

            let mut sixel_data = Vec::with_capacity(max_sixels);

            let mut sd = SpaceDimension {
                width,
                height,
                ..Default::default()
            };

            for row in 0..sixel_rows {
                let row_idx = row * sixel_len;

                method::apply_15bpp_dither(
                    pixels[row_idx..].as_mut(),
                    sd,
                    MethodForDiffuse::Atkinson,
                )?;

                for col_idx in (0..rgb_len).step_by(3) {
                    // sixels are six pixels high, one pixel wide
                    // calculate offsets for each row in the sixel column
                    let p1_off = row_idx + col_idx;
                    let p2_off = row_idx + col_idx + rgb_len;
                    let p3_off = row_idx + col_idx + (rgb_len * 2);
                    let p4_off = row_idx + col_idx + (rgb_len * 3);
                    let p5_off = row_idx + col_idx + (rgb_len * 4);
                    let p6_off = row_idx + col_idx + (rgb_len * 5);

                    sd.x = col_idx;
                    sd.y = row;

                    let sixel_plane = [
                        [pixels[p1_off], pixels[p1_off + 1], pixels[p1_off + 2]],
                        [pixels[p2_off], pixels[p2_off + 1], pixels[p2_off + 2]],
                        [pixels[p3_off], pixels[p3_off + 1], pixels[p3_off + 2]],
                        [pixels[p4_off], pixels[p4_off + 1], pixels[p4_off + 2]],
                        [pixels[p5_off], pixels[p5_off + 1], pixels[p5_off + 2]],
                        [pixels[p6_off], pixels[p6_off + 1], pixels[p6_off + 2]],
                    ];

                    let color_idx = find_closest_palette(&sixel_plane, &color_map);
                    sixel_data.push(SixelItem::Color(BasicColor::from(color_idx)));

                    let sixel_char = SixelChar::from(&sixel_plane);

                    // FIXME: add repeat detection
                    sixel_data.push(SixelItem::Char(sixel_char));
                }
                // add newline to move to the next row
                sixel_data.push(SixelItem::Control(SixelControl::NewLine));
            }

            // encode any remaining pixel rows
            if sixel_rem != 0 {
                let row_idx = sixel_rows * sixel_len;

                method::apply_15bpp_dither(
                    pixels[row_idx..].as_mut(),
                    sd,
                    MethodForDiffuse::Atkinson,
                )?;

                for col_idx in (0..rgb_len).step_by(3) {
                    // sixels are six pixels high, one pixel wide
                    // calculate offsets for each row in the sixel column
                    let p1_off = row_idx * col_idx;
                    let p2_off = row_idx * col_idx + rgb_len;
                    let p3_off = row_idx * col_idx + (rgb_len * 2);
                    let p4_off = row_idx * col_idx + (rgb_len * 3);
                    let p5_off = row_idx * col_idx + (rgb_len * 4);

                    let sixel_plane = [
                        [pixels[p1_off], pixels[p1_off + 1], pixels[p1_off + 2]],
                        if sixel_rem >= 2 {
                            [pixels[p2_off], pixels[p2_off + 1], pixels[p2_off + 2]]
                        } else {
                            [0, 0, 0]
                        },
                        if sixel_rem >= 3 {
                            [pixels[p3_off], pixels[p3_off + 1], pixels[p3_off + 2]]
                        } else {
                            [0, 0, 0]
                        },
                        if sixel_rem >= 4 {
                            [pixels[p4_off], pixels[p4_off + 1], pixels[p4_off + 2]]
                        } else {
                            [0, 0, 0]
                        },
                        if sixel_rem >= 5 {
                            [pixels[p5_off], pixels[p5_off + 1], pixels[p5_off + 2]]
                        } else {
                            [0, 0, 0]
                        },
                        [0, 0, 0],
                    ];

                    let color_idx = find_closest_palette(&sixel_plane, &color_map);
                    sixel_data.push(SixelItem::Color(BasicColor::from(color_idx)));

                    // FIXME: add repeat detection
                    let sixel_char = SixelChar::from(&sixel_plane);
                    sixel_data.push(SixelItem::Char(sixel_char));
                }
                // add newline to move to the next row
                sixel_data.push(SixelItem::Control(SixelControl::NewLine));
            }

            Ok(Self::new()
                .with_color_map(color_map)
                .with_raster(Raster::create(1, 1, width, height))
                .with_sixel_data(sixel_data))
        }
    }
}

pub fn find_closest_palette(sixel_plane: &RgbSixelBytes, color_map: &ColorMap) -> u8 {
    let mut dist = 784i32;
    let mut color_idx = 0;

    // get the average RGB values of the sixel
    let mut median_r = [
        sixel_plane[0][0],
        sixel_plane[1][0],
        sixel_plane[2][0],
        sixel_plane[3][0],
        sixel_plane[4][0],
        sixel_plane[5][0],
    ];
    median_r.sort();
    let avg_r = ((median_r[0..3].iter().cloned().map(u16::from).sum::<u16>() / 4) & 0xff) as i32;

    let mut median_g = [
        sixel_plane[0][1],
        sixel_plane[1][1],
        sixel_plane[2][1],
        sixel_plane[3][1],
        sixel_plane[4][1],
        sixel_plane[5][1],
    ];
    median_g.sort();
    let avg_g = ((median_g[0..3].iter().cloned().map(u16::from).sum::<u16>() / 4) & 0xff) as i32;

    let mut median_b = [
        sixel_plane[0][2],
        sixel_plane[1][2],
        sixel_plane[2][2],
        sixel_plane[3][2],
        sixel_plane[4][2],
        sixel_plane[5][2],
    ];
    median_b.sort();
    let avg_b = ((median_b[0..3].iter().cloned().map(u16::from).sum::<u16>() / 4) & 0xff) as i32;

    // find the closest palette color to the average sixel color
    for (i, color) in color_map.items().iter().enumerate() {
        let d = (avg_r - (color.x() as i32)).abs()
            + (avg_g - (color.y() as i32)).abs()
            + (avg_b - (color.z() as i32)).abs();

        if d < dist {
            color_idx = i;
            dist = d;
        }
    }

    color_idx as u8
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
