use std::f64::consts::PI;

use crate::{
    geometry::{Angle, Coordinate},
    point_set::PointSet,
};

#[derive(Debug, Clone, Copy)]
pub struct Vogel {
    pub seeds: usize,
    pub scaling_factor: f64,
    pub angle: Angle,
    radius: f64,
}

impl Vogel {
    pub fn new(seeds: usize, scaling_factor: f64, angle: Angle) -> Self {
        let radius = 2.0 * scaling_factor * ((seeds - 1) as f64).sqrt();

        Self {
            seeds,
            scaling_factor,
            angle,
            radius,
        }
    }

    pub fn new_sunflower(seeds: usize, scaling_factor: f64) -> Self {
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
        let phi = *index as f64 * self.angle.to_radian();
        Coordinate {
            x: r * phi.cos() + self.radius,
            y: r * phi.sin() + self.radius,
        }
    }

    fn bounding_box(&self) -> Coordinate {
        Coordinate {
            x: self.radius * 2.0,
            y: self.radius * 2.0,
        }
    }
}
