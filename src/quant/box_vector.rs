use alloc::vec::Vec;

use crate::std::{
    cmp,
    ops::{Index, IndexMut},
};

/// Sort [BoxVector] by sum.
pub fn sum_compare(lhs: &Box0, rhs: &Box0) -> cmp::Ordering {
    rhs.sum.cmp(&lhs.sum)
}

/// Represents a box of [ColorMap](super::ColorMap) pixels.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Box0 {
    /// Index in the [ColorMap](super::ColorMap).
    pub ind: usize,
    /// Colors of this box.
    pub colors: usize,
    /// Sum of pixel values.
    pub sum: usize,
}

impl Box0 {
    /// Creates a new [Box0].
    pub const fn new() -> Self {
        Self {
            ind: 0,
            colors: 0,
            sum: 0,
        }
    }

    /// Creates a new [Box0] with the given parameters.
    pub const fn create(ind: usize, colors: usize, sum: usize) -> Self {
        Self { ind, colors, sum }
    }
}

/// Represents a list of [Box0].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BoxVector(pub Vec<Box0>);

impl BoxVector {
    /// Creates a new [BoxVector].
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates a new [BoxVector] with a provided capacity.
    pub fn with_capacity(cap: usize) -> Self {
        Self(Vec::with_capacity(cap))
    }

    /// Creates a new [BoxVector] with the give parameters
    pub fn create(ind: usize, colors: usize, sum: usize) -> Self {
        Self([Box0::create(ind, colors, sum)].into())
    }

    /// Gets a reference to the inner [BoxVector] type.
    pub fn as_inner(&self) -> &Vec<Box0> {
        &self.0
    }

    /// Gets a mutable reference to the inner [BoxVector] type.
    pub fn as_inner_mut(&mut self) -> &mut Vec<Box0> {
        &mut self.0
    }
}

impl AsRef<[Box0]> for BoxVector {
    fn as_ref(&self) -> &[Box0] {
        self.0.as_ref()
    }
}

impl AsMut<[Box0]> for BoxVector {
    fn as_mut(&mut self) -> &mut [Box0] {
        self.0.as_mut()
    }
}

impl Index<usize> for BoxVector {
    type Output = Box0;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl IndexMut<usize> for BoxVector {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}
