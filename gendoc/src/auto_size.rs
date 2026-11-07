//! # Diagram size auto-discovery utilities

use dsntk_model::{DcDimension, DmnDiagram, DmnEdge, DmnShape};

/// Default diagram width.
const DEFAULT_WIDTH: f64 = 300.0;

/// Default diagram height.
const DEFAULT_HEIGHT: f64 = 300.0;

/// Auto-discovered diagram size.
#[derive(Debug, Default)]
pub struct AutoSize {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
}

impl AutoSize {
    /// Creates a new, zero [AutoSize].
    pub fn new() -> Self {
        Self {
            min_x: f64::INFINITY,
            max_x: f64::NEG_INFINITY,
            min_y: f64::INFINITY,
            max_y: f64::NEG_INFINITY,
        }
    }

    /// Updates diagram size based on shape.
    pub fn discover_from_shape(&mut self, shape: &DmnShape) {
        if self.min_x > shape.bounds.x {
            self.min_x = shape.bounds.x;
        }
        if self.max_x < shape.bounds.x + shape.bounds.width {
            self.max_x = shape.bounds.x + shape.bounds.width
        }
        if self.min_y > shape.bounds.y {
            self.min_y = shape.bounds.y;
        }
        if self.max_y < shape.bounds.y + shape.bounds.height {
            self.max_y = shape.bounds.y + shape.bounds.height
        }
    }

    /// Updates diagram size based on edge.
    pub fn discover_from_edge(&mut self, edge: &DmnEdge) {
        for point in &edge.way_points {
            if self.min_x > point.x {
                self.min_x = point.x;
            }
            if self.max_x < point.x {
                self.max_x = point.x
            }
            if self.min_y > point.y {
                self.min_y = point.y;
            }
            if self.max_y < point.y {
                self.max_y = point.y
            }
        }
    }

    /// Returns dimension of the diagram.
    pub fn dimension(mut self, diagram: &DmnDiagram) -> DcDimension {
        if let Some(size) = diagram.size {
            size
        } else {
            if self.min_x.is_infinite() {
                self.min_x = 0.0;
            }
            if self.max_x.is_infinite() {
                self.max_x = DEFAULT_WIDTH;
            }
            if self.min_y.is_infinite() {
                self.min_y = 0.0;
            }
            if self.max_y.is_infinite() {
                self.max_y = DEFAULT_HEIGHT;
            }
            DcDimension {
                width: self.min_x + self.max_x,
                height: self.min_y + self.max_y,
            }
        }
    }
}
