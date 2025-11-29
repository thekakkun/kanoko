use std::f64::consts::PI;

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
    pub fn to_degree(self) -> f64 {
        match self {
            Angle::Degree(theta) => theta,
            Angle::Radian(theta) => 2.0 * PI * theta / 360.0,
        }
    }
    pub fn to_radian(self) -> f64 {
        match self {
            Angle::Degree(theta) => theta * 360.0 / 2.0 / PI,
            Angle::Radian(theta) => theta,
        }
    }
}
