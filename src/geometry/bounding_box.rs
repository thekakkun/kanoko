use std::ops::{Add, Sub};

use crate::geometry::Coordinate;

/// Bounding box
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox(Coordinate, Coordinate);

impl BoundingBox {
    /// Create a new bounding box
    pub fn new(coordinate1: Coordinate, coordinate2: Coordinate) -> Self {
        let mut bb = Self(coordinate1, coordinate2);
        bb.normalize();

        bb
    }

    /// Create a new bounding box where one of the points is the origin
    pub fn from_point(point: Coordinate) -> Self {
        BoundingBox::new(Coordinate::origin(), point)
    }

    /// Create a new bounding box that encloses all points
    pub fn from_points<'a, I>(points: I) -> Self
    where
        I: IntoIterator<Item = &'a Coordinate>,
    {
        let (min_x, min_y, max_x, max_y) = points.into_iter().fold(
            (
                f64::INFINITY,
                f64::INFINITY,
                f64::NEG_INFINITY,
                f64::NEG_INFINITY,
            ),
            |(min_x, min_y, max_x, max_y), point| {
                let (x, y) = point.to_cartesian();
                (min_x.min(x), min_y.min(y), max_x.max(x), max_y.max(y))
            },
        );

        Self(
            Coordinate::Cartesian { x: min_x, y: min_y },
            Coordinate::Cartesian { x: max_x, y: max_y },
        )
    }

    /// Return the coordinate for minimum bound
    pub fn min(&self) -> Coordinate {
        self.0
    }

    /// Return the coordinate for maximum bound
    pub fn max(&self) -> Coordinate {
        self.1
    }

    /// Return the span of the bounding box
    pub fn span(&self) -> Coordinate {
        self.1 - self.0
    }

    /// Test whether two bounding boxes intersect
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

    fn normalize(&mut self) {
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

impl Add for BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: BoundingBox) -> Self::Output {
        let BoundingBox(self_min, self_max) = self;
        let BoundingBox(other_min, other_max) = rhs;

        let (self_min_x, self_min_y) = self_min.to_cartesian();
        let (self_max_x, self_max_y) = self_max.to_cartesian();
        let (other_min_x, other_min_y) = other_min.to_cartesian();
        let (other_max_x, other_max_y) = other_max.to_cartesian();

        let coordinate1 = Coordinate::Cartesian {
            x: self_min_x.min(other_min_x),
            y: self_min_y.min(other_min_y),
        };
        let coordinate2 = Coordinate::Cartesian {
            x: self_max_x.max(other_max_x),
            y: self_max_y.max(other_max_y),
        };

        BoundingBox::new(coordinate1, coordinate2)
    }
}

impl Add<Coordinate> for BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: Coordinate) -> Self::Output {
        BoundingBox(self.0 + rhs, self.1 + rhs)
    }
}

impl Sub<Coordinate> for BoundingBox {
    type Output = BoundingBox;

    fn sub(self, rhs: Coordinate) -> Self::Output {
        BoundingBox(self.0 - rhs, self.1 - rhs)
    }
}
