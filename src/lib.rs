//! Sixel image encoding/decoding.

#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use(format, vec)]
extern crate alloc;

#[cfg(not(feature = "std"))]
use core as std;
#[cfg(feature = "std")]
use std;

pub mod color;
pub mod config;
pub(crate) mod constants;
pub mod device_control_string;
pub mod dimension;
pub mod dither;
pub mod encoder;
mod error;
pub mod output;
pub mod palette;
pub mod pixel_format;
pub mod quant;
pub mod tosixel;
pub mod writer;

pub use constants::*;
pub use error::*;
