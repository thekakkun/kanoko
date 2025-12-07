use std::{
    f64::consts::PI,
    ops::{Add, AddAssign, Mul},
};

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

impl Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let self_rad = self.to_radian();
        let rhs_rad = rhs.to_radian();

        Angle::Radian((self_rad + rhs_rad) % (2.0 * PI))
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
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
