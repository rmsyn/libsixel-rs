use crate::std::fmt;

/// Raster attributes
///
/// The `" (2/2)` character is the set raster attributes command.
///
/// This command selects the raster attributes for the sixel data string that follows it.
///
/// You must use the command before any sixel data string.
///
/// The `"` command overrides any raster attributes set by the macro parameter described above.
///
/// You use the following format for the `"` command:
///
/// | "   | Pan  |  ;   | Pad; | Ph;  | Pv   |
/// |-----|------|------|------|------|------|
/// | 2/2 | `**` | 3/11 | `**` | `**` | `**` |
///
/// where:
///
/// **Pan** and **Pad** define the pixel aspect ratio for the following sixel data string. Pan is the numerator, and Pad is the denominator.
///
/// ```no_build, no_run
/// Pan
/// --- = pixel aspect ratio
/// Pad
/// ```
///
/// The pixel aspect ratio defines the shape of the pixels the terminal uses to draw the sixel image.
///
/// Pan defines the vertical shape of the pixel. Pad defines the horizontal shape of the pixel. For example, to define a pixel that is twice as high as it is wide, you use a value of 2 for Pan and 1 for Pad.
///
/// If you use the set raster attributes command (") in a sixel data string, you must specify a pixel aspect ratio. You can only use integer values for Pan and Pad. The VT300 rounds the pixel aspect ratio to the nearest integer.
///
/// **Ph** and **Pv** define the horizontal and vertical size of the image (in pixels), respectively.
///
/// `Ph` and `Pv` do not limit the size of the image defined by the sixel data. However, `Ph` and `Pv` let you omit background sixel data from the image definition and still have a color background. They also provide a concise way for the application or terminal to encode the size of an image.
///
/// ___NOTE: The VT300 uses Ph and Pv to erase the background when P2 is set to 0 or 2.___
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Raster {
    pan: usize,
    pad: usize,
    h: usize,
    v: usize,
}

impl Raster {
    /// Creates a new [Raster].
    pub const fn new() -> Self {
        Self {
            pan: 0,
            pad: 0,
            h: 0,
            v: 0,
        }
    }

    /// Creates a new [Raster] with the provided parameters.
    pub const fn create(pan: usize, pad: usize, h: usize, v: usize) -> Self {
        Self { pan, pad, h, v }
    }

    /// Gets the `pan` numerator for the pixel aspect ratio.
    pub const fn pan(&self) -> usize {
        self.pan
    }

    /// Sets the `pan` numerator for the pixel aspect ratio.
    pub fn set_pan(&mut self, val: usize) {
        self.pan = val;
    }

    /// Builder function that sets the `pan` numerator for the pixel aspect ratio.
    pub fn with_pan(mut self, val: usize) -> Self {
        self.set_pan(val);
        self
    }

    /// Gets the `pad` denominator for the pixel aspect ratio.
    pub const fn pad(&self) -> usize {
        self.pad
    }

    /// Sets the `pad` denominator for the pixel aspect ratio.
    pub fn set_pad(&mut self, val: usize) {
        self.pad = val;
    }

    /// Builder function that sets the `pad` denominator for the pixel aspect ratio.
    pub fn with_pad(mut self, val: usize) -> Self {
        self.set_pad(val);
        self
    }

    /// Gets the horizontal size of the image.
    pub const fn h(&self) -> usize {
        self.h
    }

    /// Sets the horizontal size of the image.
    pub fn set_h(&mut self, val: usize) {
        self.h = val;
    }

    /// Builder function that sets the horizontal size of the image.
    pub fn with_h(mut self, val: usize) -> Self {
        self.set_h(val);
        self
    }

    /// Gets the vertical size of the image.
    pub const fn v(&self) -> usize {
        self.v
    }

    /// Sets the vertical size of the image.
    pub fn set_v(&mut self, val: usize) {
        self.v = val;
    }

    /// Builder function that sets the vertical size of the image.
    pub fn with_v(mut self, val: usize) -> Self {
        self.set_v(val);
        self
    }
}

impl fmt::Display for Raster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{};{};{};{}"#, self.pan, self.pad, self.h, self.v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let raster = Raster::create(2, 1, 600, 480);
        let exp_raster = r#""2;1;600;480"#;

        assert_eq!(format!("{raster}").as_str(), exp_raster);
    }
}
