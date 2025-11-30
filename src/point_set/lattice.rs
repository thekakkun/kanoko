//! A 2D lattice.
use std::f64::consts::{PI, SQRT_2};

use itertools::iproduct;

use crate::{
    geometry::{Angle, Coordinate},
    point_set::PointSet,
};

/// The index for each point in the lattice
#[derive(Debug, Clone, Copy)]
pub struct Index {
    pub x: u16,
    pub y: u16,
}

/// A 2D lattice, composed of points defined by two vectors `a` and `b` with angle `theta`
///
/// See the [Wikipedia page for lattices](https://en.wikipedia.org/wiki/Lattice_(group)#Lattices_in_two_dimensions:_detailed_discussion) for examples of different 2D lattice configurations.
#[derive(Debug, Clone, Copy)]
pub struct Lattice {
    /// Number of points in the lattice in the X and Y direction
    pub grid_size: Index,

    /// Magnitude of the `a` vector
    pub a: f64,

    /// Magnitude of the `b` vector
    pub b: f64,

    /// Angle between the `a` and `b` vector
    pub theta: Angle,

    theta_cos: f64,
    theta_sin: f64,
}

impl Lattice {
    /// Create a new lattice
    pub fn new(grid_size: Index, a: f64, b: f64, theta: Angle) -> Self {
        let theta_rad = theta.to_radian();
        Self {
            grid_size,
            a,
            b,
            theta,
            theta_cos: theta_rad.cos(),
            theta_sin: theta_rad.sin(),
        }
    }

    /// Create a square lattice
    pub fn new_square(grid_size: Index, a: f64) -> Self {
        Self::new(grid_size, a, a, Angle::Radian(PI / 2.0))
    }

    /// Create a rectangular lattice
    pub fn new_rectangle(grid_size: Index, a: f64, b: f64) -> Self {
        Self::new(grid_size, a, b, Angle::Radian(PI / 2.0))
    }

    /// Create a centered rectangle lattice, aka a rhombic lattice
    pub fn new_centered_rectangle(grid_size: Index, a: f64, b: f64) -> Self {
        Self::new(grid_size, a, b, Angle::Radian((a / b).atan()))
    }

    /// Create a centered square lattice, aka a diagonal square lattice
    pub fn new_centered_square(grid_size: Index, a: f64) -> Self {
        Self::new(grid_size, a, a / SQRT_2, Angle::Radian(PI / 4.0))
    }

    /// Create a hexagonal lattice, aka an equilateral triangular lattice
    pub fn new_hexagonal(grid_size: Index, a: f64) -> Self {
        Self::new(grid_size, a, a, Angle::Radian(PI / 3.0))
    }
}

impl PointSet for Lattice {
    type Index = Index;

    fn index_iter(&self) -> Box<dyn Iterator<Item = Self::Index>> {
        Box::new(iproduct!(0..self.grid_size.x, 0..self.grid_size.y).map(|(x, y)| Index { x, y }))
    }

    fn index_to_coordinate(&self, index: &Self::Index) -> Coordinate {
        let x = index.x as f64 * self.a + (index.y % 2) as f64 * self.b * self.theta_cos;
        let y = index.y as f64 * self.b * self.theta_sin;

        Coordinate { x, y }
    }

    fn bounding_box(&self) -> Coordinate {
        let max_x = (self.grid_size.x - 1) as f64 * self.a
            + ((self.grid_size.y - 1) % 2) as f64 * self.b * self.theta_cos;
        let max_y = (self.grid_size.y - 1) as f64 * self.b * self.theta_sin;

        Coordinate { x: max_x, y: max_y }
    }
}
