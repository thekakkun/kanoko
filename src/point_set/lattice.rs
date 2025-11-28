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
}

impl Lattice {
    pub fn new(grid_size: Index, a: f64, b: f64, theta: Angle) -> Self {
        Self {
            grid_size,
            a,
            b,
            theta,
        }
    }

    pub fn new_square(grid_size: Index, a: f64) -> Self {
        Self {
            grid_size,
            a,
            b: a,
            theta: Angle::Radian(PI / 2.0),
        }
    }

    pub fn new_rectangle(grid_size: Index, a: f64, b: f64) -> Self {
        Self {
            grid_size,
            a,
            b,
            theta: Angle::Radian(PI / 2.0),
        }
    }

    pub fn new_centered_rectangle(grid_size: Index, a: f64, b: f64) -> Self {
        Self {
            grid_size,
            a,
            b,
            theta: Angle::Radian((a / b).atan()),
        }
    }

    pub fn new_centered_square(grid_size: Index, a: f64) -> Self {
        Self {
            grid_size,
            a,
            b: a / SQRT_2,
            theta: Angle::Radian(PI / 4.0),
        }
    }

    pub fn new_hexagonal(grid_size: Index, a: f64) -> Self {
        Self {
            grid_size,
            a,
            b: a,
            theta: Angle::Radian(PI / 3.0),
        }
    }
}

impl PointSet for Lattice {
    type Index = Index;

    fn index_iter(&self) -> Box<dyn Iterator<Item = Self::Index>> {
        let iter = iproduct!(0..self.grid_size.x, 0..self.grid_size.y).map(|(x, y)| Index { x, y });

        Box::new(iter)
    }

    fn index_to_coordinate(&self, index: &Self::Index) -> Coordinate {
        let x =
            index.x as f64 * self.a + (index.y % 2) as f64 * self.b * self.theta.to_radian().cos();
        let y = index.y as f64 * self.b * self.theta.to_radian().sin();

        Coordinate { x, y }
    }

    fn bounding_box(&self) -> Coordinate {
        let max_x = (self.grid_size.x - 1) as f64 * self.a
            + ((self.grid_size.y - 1) % 2) as f64 * self.b * self.theta.to_radian().cos();
        let max_y = (self.grid_size.y - 1) as f64 * self.b * self.theta.to_radian().sin();

        Coordinate { x: max_x, y: max_y }
    }
}
