//! Fast poisson disk sampling algorithm for generating blue noise
use rand::RngExt;
use std::{
    collections::HashMap,
    f64::consts::{PI, SQRT_2},
};

use bon::bon;
use itertools::iproduct;

use crate::{
    geometry::{Angle, Coordinate},
    point_set::PointSet,
};

/// Index for the grid cell used during the fast poisson disk sampling algorithm
///
/// Each grid cell is a square with sides `r / SQRT_2`, and starts at `(0, 0)` on the top left
/// (same as SVG). Each cell can contain _at most_ one point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Index {
    pub x: u16,
    pub y: u16,
}

/// Fast poisson disk sampling algorithm for generating blue noise
///
/// An implementation of
/// [the algorithm described here](https://www.cs.ubc.ca/~rbridson/docs/bridson-siggraph07-poissondisk.pdf).
#[derive(Debug, Clone)]
pub struct PoissonDisk {
    /// Size of the rectangle where points are placed
    pub size: (f64, f64),

    /// The minimum distance between points
    pub r: f64,

    /// Number of candidate points to try before rejection
    ///
    /// The paper suggests `k = 30` as a typical value.
    pub k: u16,

    cell_size: f64,
    grid: HashMap<Index, Coordinate>,
}

#[bon]
impl PoissonDisk {
    /// Create a new `PoissonDisk` sample
    #[inline]
    #[builder]
    pub fn new(
        #[builder(with = |x: f64, y: f64| ( x, y ))] size: (f64, f64),
        r: f64,
        k: u16,
    ) -> Self {
        let mut poisson_disk = Self {
            size,
            r,
            k,
            cell_size: r / SQRT_2,
            grid: HashMap::new(),
        };
        poisson_disk.generate();

        poisson_disk
    }

    fn generate(&mut self) {
        let mut rng = rand::rng();

        let init_point = Coordinate::Cartesian {
            x: rng.random_range(0.0..self.size.0),
            y: rng.random_range(0.0..self.size.1),
        };
        let mut actives = vec![init_point];
        self.grid
            .insert(init_point.to_cell_index(self.cell_size), init_point);

        while !actives.is_empty() {
            let i = rng.random_range(0..actives.len());
            let point = actives[i];

            if let Some(new_point) = (0..self.k).find_map(|_| {
                let candidate = point
                    + Coordinate::Polar {
                        r: rng.random_range(self.r..self.r * 2.0),
                        phi: Angle::Radian(rng.random_range(0.0..2.0 * PI)),
                    };

                (self.is_in_bounds(&candidate) && self.is_valid(&candidate)).then_some(candidate)
            }) {
                actives.push(new_point);
                self.grid
                    .insert(new_point.to_cell_index(self.cell_size), new_point);
            } else {
                actives.swap_remove(i);
            }
        }
    }

    fn is_in_bounds(&self, point: &Coordinate) -> bool {
        let (x, y) = point.to_cartesian();
        (0.0..self.size.0).contains(&x) && (0.0..self.size.1).contains(&y)
    }

    fn is_valid(&self, candidate: &Coordinate) -> bool {
        let cell_index = candidate.to_cell_index(self.cell_size);

        iproduct!(
            cell_index.x.saturating_sub(2)..=cell_index.x + 2,
            cell_index.y.saturating_sub(2)..=cell_index.y + 2
        )
        .filter_map(|(x, y)| {
            let index = Index { x, y };
            self.grid.get(&index)
        })
        .all(|point| self.r <= candidate.dist(point))
    }
}

impl PointSet for PoissonDisk {
    type Index = Index;

    fn index_iter(&self) -> Box<dyn Iterator<Item = Self::Index>> {
        Box::new(self.grid.keys().copied().collect::<Vec<_>>().into_iter())
    }

    fn index_to_coordinate(&self, index: &Self::Index) -> Coordinate {
        *self.grid.get(index).unwrap()
    }

    fn bounding_box(&self) -> (Coordinate, Coordinate) {
        (
            Coordinate::Cartesian { x: 0.0, y: 0.0 },
            Coordinate::Cartesian {
                x: self.size.0,
                y: self.size.1,
            },
        )
    }
}

impl Coordinate {
    fn to_cell_index(self, cell_size: f64) -> Index {
        let (x, y) = self.to_cartesian();

        Index {
            x: (x / cell_size).floor() as u16,
            y: (y / cell_size).floor() as u16,
        }
    }
}
