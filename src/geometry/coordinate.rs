use std::ops::{Add, Div, Sub};

use crate::geometry::Angle;

/// Coordinate system
#[derive(Debug, Clone, Copy)]
pub enum Coordinate {
    Cartesian { x: f64, y: f64 },
    Polar { r: f64, phi: Angle },
}

impl Coordinate {
    /// Get the cartesian coordinates
    pub fn to_cartesian(self) -> (f64, f64) {
        match self {
            Coordinate::Cartesian { x, y } => (x, y),
            Coordinate::Polar { r, phi } => (r * phi.to_radian().cos(), r * phi.to_radian().sin()),
        }
    }

    /// Get the polar coordinates
    pub fn to_polar(self) -> (f64, Angle) {
        match self {
            Coordinate::Cartesian { x, y } => ((x * x + y * y).sqrt(), Angle::Radian(y.atan2(x))),
            Coordinate::Polar { r, phi } => (r, phi),
        }
    }

    /// Calculate the linearly interpolated point between two coordinates
    #[inline]
    pub(crate) fn lerp(&self, other: &Self, t: f64) -> Self {
        let (self_x, self_y) = self.to_cartesian();
        let (other_x, other_y) = other.to_cartesian();

        Self::Cartesian {
            x: self_x + t * (other_x - self_x),
            y: self_y + t * (other_y - self_y),
        }
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (self_x, self_y) = self.to_cartesian();
        let (rhs_x, rhs_y) = rhs.to_cartesian();

        Self::Cartesian {
            x: self_x + rhs_x,
            y: self_y + rhs_y,
        }
    }
}

impl Div<f64> for Coordinate {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let (self_x, self_y) = self.to_cartesian();

        Self::Cartesian {
            x: self_x / rhs,
            y: self_y / rhs,
        }
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let (self_x, self_y) = self.to_cartesian();
        let (rhs_x, rhs_y) = rhs.to_cartesian();

        Self::Cartesian {
            x: self_x - rhs_x,
            y: self_y - rhs_y,
        }
    }
}
