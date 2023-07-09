use alloc::vec::Vec;

/// Convenience alias for a palette sample.
pub type Sample = u32;

/// Convenience alias for a [Sample] list.
pub type Tuple = Vec<Sample>;

/// Variable-length container of [ColorMap](super::ColorMap) [Sample]s.
#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct TupleInt {
    pub value: u32,
    pub tuple: Tuple,
}

impl TupleInt {
    /// Creates a new [TupleInt].
    pub const fn new() -> Self {
        Self {
            value: 0,
            tuple: Tuple::new(),
        }
    }

    /// Creates a new [TupleInt] with `depth` entries.
    pub fn with_depth(depth: usize) -> Self {
        Self {
            value: 0,
            tuple: vec![0; depth],
        }
    }

    /// Gets the value.
    pub const fn value(&self) -> u32 {
        self.value
    }

    /// Sets the value.
    pub fn set_value(&mut self, value: u32) {
        self.value = value;
    }

    /// Gets a reference to the [Tuple].
    pub fn tuple(&self) -> &[Sample] {
        self.tuple.as_ref()
    }

    /// Sets the [Tuple].
    pub fn set_tuple<V: Into<Tuple>>(&mut self, tuple: V) {
        self.tuple = tuple.into();
    }
}
