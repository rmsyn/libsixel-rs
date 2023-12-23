use super::SixelChar;
use crate::std::fmt;

/// Graphics repeat introducer sequence.
///
/// The `! (2/1)` character introduces a repeat sequence. A repeat sequence lets you repeat a graphic character a specified number of times. You use the following format for the repeat sequence.
///
/// | !    | Pn   | character |
/// |------|------|-----------|
/// | 2/1  | `**` | `****`    |
///
/// where:
///
/// - **Pn** is the repeat count. The repeat count can be any decimal value. For example, if you use a repeat count of 23, the next character repeats 23 times.
/// - **character** is the character to repeat. You can use any character in the range of `? (hex 3F)` to `~ (hex 7E)`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Repeat {
    number: usize,
    character: SixelChar,
}

impl Repeat {
    /// Creates a new [Repeat].
    pub const fn new() -> Self {
        Self {
            number: 0,
            character: SixelChar::new(),
        }
    }

    /// Creates a new [Repeat] with the provided parameters.
    pub const fn create(number: usize, character: SixelChar) -> Self {
        Self { number, character }
    }

    /// Gets the number of [SixelChar]s to repeat.
    pub const fn number(&self) -> usize {
        self.number
    }

    /// Sets the number of [SixelChar]s to repeat.
    pub fn set_number(&mut self, val: usize) {
        self.number = val;
    }

    /// Builder function that sets the number of [SixelChar]s to repeat.
    pub fn with_number(mut self, val: usize) -> Self {
        self.set_number(val);
        self
    }

    /// Gets the [SixelChar] to repeat.
    pub const fn character(&self) -> SixelChar {
        self.character
    }

    /// Sets the [SixelChar] to repeat.
    pub fn set_character(&mut self, val: SixelChar) {
        self.character = val;
    }

    /// Builder function that sets the [SixelChar] to repeat.
    pub fn with_character(mut self, val: SixelChar) -> Self {
        self.set_character(val);
        self
    }
}

impl fmt::Display for Repeat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "!{}{}", self.number, self.character)
    }
}
