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
mod error;
pub mod palette;
pub mod pixel_format;
pub mod quant;

pub use constants::*;
pub use device_control_string::*;
pub use error::*;
