use std::{f64::consts::PI, ops::Mul};

#[derive(Debug, Clone, Copy)]
pub enum Angle {
    Degree(f64),
    Radian(f64),
}

impl Default for Angle {
    fn default() -> Self {
        Self::Radian(0.0)
    }
}

impl Angle {
    #[inline]
    pub fn to_degree(self) -> f64 {
        match self {
            Angle::Degree(theta) => theta,
            Angle::Radian(theta) => theta * 180.0 / PI,
        }
    }

    #[inline]
    pub fn to_radian(self) -> f64 {
        match self {
            Angle::Degree(theta) => theta * PI / 180.0,
            Angle::Radian(theta) => theta,
        }
    }
}

impl Mul<f64> for Angle {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        match self {
            Angle::Degree(theta) => Angle::Degree((theta * rhs) % 360.0),
            Angle::Radian(theta) => Angle::Radian((theta * rhs) % (2.0 * PI)),
        }
    }
}
