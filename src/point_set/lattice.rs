use std::f64::consts::SQRT_2;

use itertools::iproduct;

use crate::{Coordinate, point_set::PointSet};

#[derive(Debug, Clone, Copy)]
pub struct Index {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct Lattice {
    pub grid_size: Index,
    pub cell_size: f64,
}

impl PointSet for Lattice {
    type Index = Index;

    fn index_iter(&self) -> Box<dyn Iterator<Item = Self::Index>> {
        let iter = iproduct!(0..self.grid_size.x, 0..self.grid_size.y).map(|(x, y)| Index { x, y });

        Box::new(iter)
    }

    fn index_to_coordinate(&self, index: &Self::Index) -> Coordinate {
        let x = 2.0 * index.x as f64 * self.cell_size * SQRT_2
            + (index.y % 2) as f64 * 2.0 * self.cell_size / SQRT_2;
        let y = 2.0 * index.y as f64 * self.cell_size / SQRT_2;

        Coordinate { x, y }
    }

    fn bounding_box(&self) -> crate::Coordinate {
        let max_x = 2.0 * (self.grid_size.x - 1) as f64 * self.cell_size * SQRT_2
            + (self.grid_size.y.min(2) - 1) as f64 * 2.0 * self.cell_size / SQRT_2;
        let max_y = 2.0 * (self.grid_size.y - 1) as f64 * self.cell_size / SQRT_2;

        Coordinate { x: max_x, y: max_y }
    }
}
