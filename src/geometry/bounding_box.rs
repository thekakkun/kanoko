use std::cmp::min;

use crate::geometry::Coordinate;

/// Bounding Box
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox(Coordinate, Coordinate);

impl BoundingBox {
    pub fn new(coordinate1: Coordinate, coordinate2: Coordinate) -> Self {
        let mut bb = Self(coordinate1, coordinate2);
        bb.normalize();

        bb
    }

    pub fn normalize(&mut self) {
        let (x0, y0) = self.0.to_cartesian();
        let (x1, y1) = self.1.to_cartesian();

        self.0 = Coordinate::Cartesian {
            x: x0.min(x1),
            y: y0.min(y1),
        };
        self.1 = Coordinate::Cartesian {
            x: x0.max(x1),
            y: y0.max(y1),
        };
    }
}
