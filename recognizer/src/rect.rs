//! # Rectangle

use std::fmt;
use std::fmt::{Debug, Display};

/// Rectangle.
#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct Rect {
    /// Left edge coordinate (inclusive).
    pub left: usize,
    /// Top edge coordinate (inclusive).
    pub top: usize,
    /// Right edge coordinate (exclusive).
    pub right: usize,
    /// Bottom edge coordinate (exclusive).
    pub bottom: usize,
}

impl Display for Rect {
    /// Implements [Display] trait for [Rect].
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({},{};{},{})",
            self.left, self.top, self.right, self.bottom
        )
    }
}

impl Debug for Rect {
    /// Implements [Debug] trait for [Rect].
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Rect {
    /// Creates a new rectangle with specified coordinates.
    pub fn new(left: usize, top: usize, right: usize, bottom: usize) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    /// Returns a copy of the rectangle with top value incremented by specified offset.
    pub fn offset_top(&self, offset: usize) -> Self {
        Self {
            top: self.top + offset,
            ..*self
        }
    }

    /// Checks if the specified rectangle is contained in this rectangle.
    pub fn contains(&self, other: &Rect) -> bool {
        other.left >= self.left
            && other.top >= self.top
            && other.right <= self.right
            && other.bottom <= self.bottom
    }

    /// Returns the width of the rectangle.
    pub fn width(&self) -> usize {
        self.right - self.left
    }

    /// Returns the height of the rectangle.
    pub fn height(&self) -> usize {
        self.bottom - self.top
    }
}

impl From<Rect> for (usize, usize, usize, usize) {
    /// Converts the rectangle's coordinates into tuple of unsigned integers.
    fn from(value: Rect) -> Self {
        (value.left, value.top, value.right, value.bottom)
    }
}
