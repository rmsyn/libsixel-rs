#[cfg(not(feature = "std"))]
mod printer;

#[cfg(not(feature = "std"))]
pub use printer::*;
