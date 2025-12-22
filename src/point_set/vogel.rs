//! Vogel's model of the spiral
use std::f64::consts::PI;

use bon::bon;

use crate::{
    geometry::{Angle, Coordinate},
    point_set::PointSet,
};

/// Vogel's model of the pattern drawn by the seeds of a sunflower
#[derive(Debug, Clone, Copy)]
pub struct Vogel {
    /// Number of points in the spiral
    pub seeds: usize,

    /// The rate at which the spiral grows
    pub scaling_factor: f64,

    /// The angle turned between seeds
    pub angle: Angle,

    radius: f64,
}

#[bon]
impl Vogel {
    /// Create a new Vogel spiral
    #[inline]
    #[builder(start_fn = builder)]
    pub fn new(seeds: usize, scaling_factor: f64, angle: Angle) -> Self {
        let radius = 2.0 * scaling_factor * ((seeds - 1) as f64).sqrt();

        Self {
            seeds,
            scaling_factor,
            angle,
            radius,
        }
    }

    /// Create a new Vogel spiral using the golden angle
    ///
    /// This angle provides the tighest packing of points per area and is commonly seen in
    /// nature. Namely, the spirals drawn by sunflower seeds.
    #[inline]
    #[builder(start_fn = golden_builder, finish_fn = build)]
    pub fn new_golden(seeds: usize, scaling_factor: f64) -> Self {
        Self::new(
            seeds,
            scaling_factor,
            Angle::Radian(PI * (3.0 - 5_f64.sqrt())),
        )
    }
}

impl PointSet for Vogel {
    type Index = usize;

    fn index_iter(&self) -> Box<dyn Iterator<Item = Self::Index>> {
        Box::new(0..self.seeds)
    }

    fn index_to_coordinate(&self, index: &Self::Index) -> Coordinate {
        let r = self.scaling_factor * (*index as f64).sqrt();
        let phi = self.angle * *index as f64;
        Coordinate::Polar { r, phi }
    }

    fn bounding_box(&self) -> (Coordinate, Coordinate) {
        (
            Coordinate::Cartesian {
                x: -self.radius,
                y: -self.radius,
            },
            Coordinate::Cartesian {
                x: self.radius,
                y: self.radius,
            },
        )
    }
}
