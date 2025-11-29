use std::f64::consts::{PI, SQRT_2};

use itertools::iproduct;

use crate::{
    geometry::{Angle, Coordinate},
    point_set::PointSet,
};

#[derive(Debug, Clone, Copy)]
pub struct Index {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct Lattice {
    pub grid_size: Index,
    pub a: f64,
    pub b: f64,
    pub theta: Angle,
    theta_cos: f64,
    theta_sin: f64,
}

impl Lattice {
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

    pub fn new_square(grid_size: Index, a: f64) -> Self {
        Self::new(grid_size, a, a, Angle::Radian(PI / 2.0))
    }

    pub fn new_rectangle(grid_size: Index, a: f64, b: f64) -> Self {
        Self::new(grid_size, a, b, Angle::Radian(PI / 2.0))
    }

    pub fn new_centered_rectangle(grid_size: Index, a: f64, b: f64) -> Self {
        Self::new(grid_size, a, b, Angle::Radian((a / b).atan()))
    }

    pub fn new_centered_square(grid_size: Index, a: f64) -> Self {
        Self::new(grid_size, a, a / SQRT_2, Angle::Radian(PI / 4.0))
    }

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
