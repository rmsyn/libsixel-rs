use crate::std::fmt;

extern "C" {
    fn write(filedes: i32, buf: *const core::ffi::c_void, nbyte: usize);
}

/// Writes a string to stdout on *nix systems.
///
/// Only valid for platforms with:
/// - `write(i32, *const c_void, usize)` C function defined
/// - stdout file-descriptor set to `1`.
pub fn write_string(s: &str) {
    unsafe {
        write(1, s.as_ptr() as *const _, s.len());
    }
}

/// Zero-sized type for writing to stdout.
pub struct Writer;

impl Writer {
    /// Creates a new [Writer].
    pub const fn new() -> Self {
        Self {}
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_string(s);

        Ok(())
    }
}

/// Print a string to `stdout` using [Writer].
pub fn print(args: fmt::Arguments) {
    use fmt::Write;
    let _ = Writer::new().write_fmt(args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::writer::print(format_args!($($arg)*));
    }}
}
