use std::ops::Add;

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

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        let BoundingBox(self_min, self_max) = self;
        let BoundingBox(other_min, other_max) = other;

        let (self_min_x, self_min_y) = self_min.to_cartesian();
        let (self_max_x, self_max_y) = self_max.to_cartesian();
        let (other_min_x, other_min_y) = other_min.to_cartesian();
        let (other_max_x, other_max_y) = other_max.to_cartesian();

        self_min_x <= other_max_x
            && self_max_x >= other_min_x
            && self_min_y <= other_max_y
            && self_max_y >= other_min_y
    }
}

impl Add<Coordinate> for BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: Coordinate) -> Self::Output {
        BoundingBox(self.0 + rhs, self.1 + rhs)
    }
}
