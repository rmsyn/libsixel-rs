//! Container types for basic dimensions

/// Container type for spacial dimension parameters.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SpaceDimension {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub x: usize,
    pub y: usize,
}

/// Container type for color dimension parameters.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColorDimension {
    pub n_colors: usize,
    pub key_color: i32,
}
