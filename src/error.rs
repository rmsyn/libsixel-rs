//! Types and functions for error handling.

use alloc::string::String;

use crate::std::{self, fmt};

/// Result type for the library.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for the library.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Generic(i32),
    Dither(String),
    Frame(String),
    PixelFormat(String),
    Quant(String),
    Output(String),
    Io(String),
    Rgb,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generic(err) => write!(f, "Generic error: {err}"),
            Self::Rgb => write!(f, "RGB parsing/encoding error"),
            Self::Dither(err) => write!(f, "Dither error: {err}"),
            Self::Frame(err) => write!(f, "Frame error: {err}"),
            Self::PixelFormat(err) => write!(f, "PixelFormat error: {err}"),
            Self::Quant(err) => write!(f, "Quant error: {err}"),
            Self::Output(err) => write!(f, "Output error: {err}"),
            Self::Io(err) => write!(f, "I/O error: {err}"),
        }
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(format!("{err}"))
    }
}
