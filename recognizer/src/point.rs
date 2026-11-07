//! # Point

use std::fmt;
use std::fmt::{Debug, Display};

/// Point.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Point {
    /// Left coordinate.
    pub x: usize,
    /// Top coordinate.
    pub y: usize,
}

impl Display for Point {
    /// Implements [Display] trait for [Point].
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Debug for Point {
    /// Implements [Debug] trait for [Point].
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Point {
    /// Creates a zero point.
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    /// Creates a point with specified coordinates.
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl From<(usize, usize)> for Point {
    /// Converts a tuple of coordinates into [Point].
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<Point> for (usize, usize) {
    /// Converts this [Point] into a tuple of coordinates.
    fn from(value: Point) -> Self {
        (value.x, value.y)
    }
}
