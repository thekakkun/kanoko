//! A 2D lattice.
use std::f64::consts::{PI, SQRT_2};

use bon::bon;
use itertools::iproduct;

use crate::{
    geometry::{Angle, Coordinate},
    point_set::PointSet,
};

/// The index for each point in the lattice
#[derive(Debug, Clone, Copy)]
pub struct Index {
    pub u: u16,
    pub v: u16,
}

/// A 2D lattice, composed of points defined by two vectors `a` and `b` with angle `theta`
///
/// See the [Wikipedia page for lattices](https://en.wikipedia.org/wiki/Lattice_(group)#Lattices_in_two_dimensions:_detailed_discussion) for examples of different 2D lattice configurations.
#[derive(Debug, Clone, Copy)]
pub struct Lattice {
    /// Number of points in the lattice in the X and Y direction
    pub grid_size: (u16, u16),

    /// Magnitude of the `a` vector
    pub len_a: f64,

    /// Magnitude of the `b` vector
    pub len_b: f64,

    /// Angle between the `a` and `b` vector
    pub theta: Angle,

    theta_cos: f64,
    theta_sin: f64,
}

#[bon]
impl Lattice {
    /// Create a new lattice
    #[inline]
    #[builder(start_fn = builder)]
    pub fn new(
        #[builder(with = |u: u16, v: u16| (u,v))] grid_size: (u16, u16),
        len_a: f64,
        len_b: f64,
        theta: Angle,
    ) -> Self {
        let theta_rad = theta.to_radian();
        Self {
            grid_size,
            len_a,
            len_b,
            theta,
            theta_cos: theta_rad.cos(),
            theta_sin: theta_rad.sin(),
        }
    }

    /// Create a square lattice
    #[inline]
    #[builder(start_fn = square_builder, finish_fn = build)]
    pub fn new_square(
        #[builder(with = |u: u16, v: u16| (u,v))] grid_size: (u16, u16),
        len_a: f64,
    ) -> Self {
        Self::new(grid_size, len_a, len_a, Angle::Radian(PI / 2.0))
    }

    /// Create a rectangular lattice
    #[inline]
    #[builder(start_fn = rectangular_builder, finish_fn = build)]
    pub fn new_rectangular(
        #[builder(with = |u: u16, v: u16| (u,v))] grid_size: (u16, u16),
        len_a: f64,
        len_b: f64,
    ) -> Self {
        Self::new(grid_size, len_a, len_b, Angle::Radian(PI / 2.0))
    }

    /// Create a centered rectangle lattice, aka a rhombic lattice
    #[inline]
    #[builder(start_fn = rhombic_builder, finish_fn = build)]
    pub fn new_rhombic(
        #[builder(with = |u: u16, v: u16| (u,v))] grid_size: (u16, u16),
        len_a: f64,
        len_b: f64,
    ) -> Self {
        Self::new(
            grid_size,
            len_a,
            len_b,
            Angle::Radian((len_a / len_b).atan()),
        )
    }

    /// Create a centered square lattice, aka a diagonal square lattice
    #[inline]
    #[builder(start_fn = diamond_builder, finish_fn = build)]
    pub fn new_diamond(
        #[builder(with = |u: u16, v: u16| (u,v))] grid_size: (u16, u16),
        len_a: f64,
    ) -> Self {
        Self::new(grid_size, len_a, len_a / SQRT_2, Angle::Radian(PI / 4.0))
    }

    /// Create a hexagonal lattice, aka an equilateral triangular lattice
    #[inline]
    #[builder(start_fn = hexagonal_builder, finish_fn = build)]
    pub fn new_hexagonal(
        #[builder(with = |u: u16, v: u16| (u,v))] grid_size: (u16, u16),
        len_a: f64,
    ) -> Self {
        Self::new(grid_size, len_a, len_a, Angle::Radian(PI / 3.0))
    }
}

impl PointSet for Lattice {
    type Index = Index;

    fn index_iter(&self) -> Box<dyn Iterator<Item = Self::Index>> {
        Box::new(iproduct!(0..self.grid_size.0, 0..self.grid_size.1).map(|(u, v)| Index { u, v }))
    }

    fn index_to_coordinate(&self, index: &Self::Index) -> Coordinate {
        let x =
            f64::from(index.u) * self.len_a + f64::from(index.v % 2) * self.len_b * self.theta_cos;
        let y = f64::from(index.v) * self.len_b * self.theta_sin;

        Coordinate::Cartesian { x, y }
    }

    fn bounding_box(&self) -> (Coordinate, Coordinate) {
        let max_x = f64::from(self.grid_size.0 - 1) * self.len_a
            + if 2 <= self.grid_size.1 {
                self.len_b * self.theta_cos
            } else {
                0.0
            };
        let max_y = f64::from(self.grid_size.1 - 1) * self.len_b * self.theta_sin;

        (
            Coordinate::Cartesian { x: 0.0, y: 0.0 },
            Coordinate::Cartesian { x: max_x, y: max_y },
        )
    }
}
