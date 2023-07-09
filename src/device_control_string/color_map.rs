use crate::std::{cmp, fmt};
use alloc::vec::Vec;

/// Naive grayscale values for a basic [ColorMap] palette.
pub const GRAYSCALE_MAP: [ColorMapItem; 5] = [
    ColorMapItem::create_rgb(0, 0x1f, 0x1f, 0x1f),
    ColorMapItem::create_rgb(1, 0x3f, 0x3f, 0x3f),
    ColorMapItem::create_rgb(2, 0x5f, 0x5f, 0x5f),
    ColorMapItem::create_rgb(3, 0x7f, 0x7f, 0x7f),
    ColorMapItem::create_rgb(4, 0xbe, 0xbe, 0xbe),
];

/// Naive RGB values for a basic [ColorMap] palette.
pub const RGB_MAP: [ColorMapItem; 3] = [
    ColorMapItem::create_rgb(0, 0x72, 0x1a, 0x1a),
    ColorMapItem::create_rgb(1, 0x2a, 0x82, 0x2a),
    ColorMapItem::create_rgb(2, 0x3a, 0x3a, 0x92),
];

/// Converts a `u8` value into a percentage.
pub const fn u8_percentage(val: u8) -> u8 {
    let ret = (val as u16) * 2 / 5;
    if ret > 100 {
        100
    } else {
        ret as u8
    }
}

/// Selects the color mode representation.
///
/// - HLS: hue light saturation
/// - RGB: red green blue
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ColorMode {
    Hls = 1,
    #[default]
    Rgb,
}

impl ColorMode {
    /// Creates a new [ColorMode].
    pub const fn new() -> Self {
        Self::Rgb
    }
}

impl From<u8> for ColorMode {
    fn from(val: u8) -> Self {
        match val {
            1 => Self::Hls,
            _ => Self::Rgb,
        }
    }
}

impl From<&ColorMode> for u8 {
    fn from(val: &ColorMode) -> Self {
        *val as u8
    }
}

impl From<ColorMode> for u8 {
    fn from(val: ColorMode) -> Self {
        (&val).into()
    }
}

/// Represents a [ColorMap] selection (by index);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BasicColor(u8);

impl From<u8> for BasicColor {
    fn from(val: u8) -> Self {
        Self(val)
    }
}

impl From<&BasicColor> for u8 {
    fn from(val: &BasicColor) -> Self {
        val.0
    }
}

impl From<BasicColor> for u8 {
    fn from(val: BasicColor) -> Self {
        (&val).into()
    }
}

impl fmt::Display for BasicColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

/// Represents a [ColorMap] entry item.
///
/// HLS mode:
///
/// - `x`: [0, 360] hue angle
/// - `y`: [0, 100] lightness percentage
/// - `z`: [0, 100] saturation percentage
///
/// RGB mode:
///
/// - `x`: [0, 100] red percentage
/// - `y`: [0, 100] green percentage
/// - `z`: [0, 100] blue percentage
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColorMapItem {
    number: u8,
    mode: ColorMode,
    x: u16,
    y: u16,
    z: u16,
}

impl ColorMapItem {
    /// Creates a new [ColorMapItem].
    pub const fn new() -> Self {
        Self {
            number: 0,
            mode: ColorMode::new(),
            x: 0,
            y: 0,
            z: 0,
        }
    }

    /// Creates a [ColorMapItem] in HLS mode.
    pub const fn create_hls(number: u8, degree: u16, lightness: u16, saturation: u16) -> Self {
        Self {
            number,
            mode: ColorMode::Hls,
            x: degree % 361,
            y: lightness % 101,
            z: saturation % 101,
        }
    }

    /// Creates a [ColorMapItem] in RGB mode.
    ///
    /// Parameters:
    ///
    /// `red`: red hue value in hex [0x00, 0xff]
    /// `green`: green hue value in hex [0x00, 0xff]
    /// `blue`: blue hue value in hex [0x00, 0xff]
    pub const fn create_rgb(number: u8, red: u8, green: u8, blue: u8) -> Self {
        Self {
            number,
            mode: ColorMode::Rgb,
            x: u8_percentage(red) as u16,
            y: u8_percentage(green) as u16,
            z: u8_percentage(blue) as u16,
        }
    }

    /// Gets the [ColorMapItem] index number.
    pub const fn number(&self) -> u8 {
        self.number
    }

    /// Sets the [ColorMapItem] index `number` parameter.
    pub fn set_number(&mut self, val: u8) {
        self.number = val;
    }

    /// Builder function that sets the [ColorMapItem] index `number` parameter.
    pub fn with_number(mut self, val: u8) -> Self {
        self.set_number(val);
        self
    }

    /// Gets the [ColorMode].
    pub const fn color_mode(&self) -> ColorMode {
        self.mode
    }

    /// Sets the [ColorMode].
    pub fn set_color_mode(&mut self, val: ColorMode) {
        self.mode = val;
    }

    /// Builder function that sets the [ColorMode].
    pub fn with_color_mode(mut self, val: ColorMode) -> Self {
        self.set_color_mode(val);
        self
    }

    /// Gets the [ColorMapItem] `x` parameter.
    ///
    /// Meaning:
    ///
    /// - `hls`: hue degree (`[0, 360]`)
    /// - `rgb`: red hue percentage (`[0, 100]`)
    pub const fn x(&self) -> u16 {
        self.x
    }

    /// Sets the [ColorMapItem] `x` parameter.
    ///
    /// Meaning:
    ///
    /// - `hls`: hue degree (`[0, 360]`)
    /// - `rgb`: red hue percentage (`[0, 100]`)
    pub fn set_x(&mut self, val: u16) {
        self.x = match self.mode {
            ColorMode::Hls => val % 361,
            ColorMode::Rgb => val % 101,
        };
    }

    /// Builder function that sets the [ColorMapItem] `x` parameter.
    ///
    /// Meaning:
    ///
    /// - `hls`: hue degree (`[0, 360]`)
    /// - `rgb`: red hue percentage (`[0, 100]`)
    pub fn with_x(mut self, val: u16) -> Self {
        self.set_x(val);
        self
    }

    /// Gets the [ColorMapItem] `y` parameter.
    ///
    /// Meaning:
    ///
    /// - `hls`: lightness percentage (`[0, 100]`)
    /// - `rgb`: green hue percentage (`[0, 100]`)
    pub const fn y(&self) -> u16 {
        self.y
    }

    /// Sets the [ColorMapItem] `y` parameter.
    ///
    /// Meaning:
    ///
    /// - `hls`: lightness percentage (`[0, 100]`)
    /// - `rgb`: green hue percentage (`[0, 100]`)
    pub fn set_y(&mut self, val: u16) {
        self.y = val % 101;
    }

    /// Builder function that sets the [ColorMapItem] `y` parameter.
    ///
    /// Meaning:
    ///
    /// - `hls`: lightness percentage (`[0, 100]`)
    /// - `rgb`: green hue percentage (`[0, 100]`)
    pub fn with_y(mut self, val: u16) -> Self {
        self.set_y(val);
        self
    }

    /// Gets the [ColorMapItem] `z` parameter.
    ///
    /// Meaning:
    ///
    /// - `hls`: saturation percentage (`[0, 100]`)
    /// - `rgb`: blue hue percentage (`[0, 100]`)
    pub const fn z(&self) -> u16 {
        self.z
    }

    /// Sets the [ColorMapItem] `z` parameter.
    ///
    /// Meaning:
    ///
    /// - `hls`: saturation percentage (`[0, 100]`)
    /// - `rgb`: blue hue percentage (`[0, 100]`)
    pub fn set_z(&mut self, val: u16) {
        self.z = val % 101;
    }

    /// Builder function that sets the [ColorMapItem] `z` parameter.
    ///
    /// Meaning:
    ///
    /// - `hls`: saturation percentage (`[0, 100]`)
    /// - `rgb`: blue hue percentage (`[0, 100]`)
    pub fn with_z(mut self, val: u16) -> Self {
        self.set_z(val);
        self
    }
}

/// Container representing a list of [ColorMapItem] entries.
///
/// Defines a palette to select colors using the
/// [`ColorIntroducer`](super::SixelControl::Color) control character.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ColorMap {
    items: Vec<ColorMapItem>,
}

impl ColorMap {
    /// Creates a new [ColorMap].
    pub const fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Creates a new [ColorMap] from the provided [ColorMapItem] list.
    pub const fn create(items: Vec<ColorMapItem>) -> Self {
        Self { items }
    }

    /// Gets a reference to the list of [ColorMapItem]s.
    pub fn items(&self) -> &[ColorMapItem] {
        self.items.as_ref()
    }

    /// Gets a mutable reference to the list of [ColorMapItem]s.
    pub fn items_mut(&mut self) -> &mut [ColorMapItem] {
        self.items.as_mut()
    }

    /// Gets a reference to the inner representation of the list of [ColorMapItem]s.
    pub fn inner(&self) -> &Vec<ColorMapItem> {
        &self.items
    }

    /// Gets a mutable reference to the inner representation of the list of [ColorMapItem]s.
    pub fn inner_mut(&mut self) -> &mut Vec<ColorMapItem> {
        &mut self.items
    }

    /// Encodes the [ColorMap] into a [Formatter](fmt::Formatter).
    pub fn encode(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, item) in self.items.iter().enumerate() {
            let (pu, px, py, pz) = (u8::from(item.mode), item.x, item.y, item.z);
            write!(f, "#{i};{pu};{px};{py};{pz}")?;
        }

        Ok(())
    }
}

impl From<Vec<ColorMapItem>> for ColorMap {
    fn from(val: Vec<ColorMapItem>) -> Self {
        Self::create(val)
    }
}

impl From<&[ColorMapItem]> for ColorMap {
    fn from(val: &[ColorMapItem]) -> Self {
        Self::create(val.into())
    }
}

impl<const N: usize> From<&[ColorMapItem; N]> for ColorMap {
    fn from(val: &[ColorMapItem; N]) -> Self {
        Self::create(val.as_ref().into())
    }
}

impl<const N: usize> From<[ColorMapItem; N]> for ColorMap {
    fn from(val: [ColorMapItem; N]) -> Self {
        Self::create(val.as_ref().into())
    }
}

impl From<&[u8]> for ColorMap {
    fn from(val: &[u8]) -> Self {
        let len = val.len() / 3;
        // only take up to 256 color map items
        let num = cmp::min((u8::MAX as usize) + 1, len);

        val.chunks_exact(3)
            .enumerate()
            .take(num)
            .map(|(i, rgb)| ColorMapItem::create_rgb(i as u8, rgb[0], rgb[1], rgb[2]))
            .collect::<Vec<ColorMapItem>>()
            .into()
    }
}

impl<const N: usize> From<&[u8; N]> for ColorMap {
    fn from(val: &[u8; N]) -> Self {
        val.as_ref().into()
    }
}

impl<const N: usize> From<[u8; N]> for ColorMap {
    fn from(val: [u8; N]) -> Self {
        val.as_ref().into()
    }
}

impl fmt::Display for ColorMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.encode(f)
    }
}
