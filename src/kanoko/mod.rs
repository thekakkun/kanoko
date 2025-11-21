use crate::Color;

#[derive(Debug)]
pub struct Kanoko {
    pub grid: Grid,
    pub grid_size: (u16, u16),
    pub cell_size: f32,
    pub background: Color,
    pub shapes: Vec<KanokoShape>,
}

#[derive(Debug)]
pub struct KanokoShape {
    pub grid: Grid,
    pub coordinate: (u16, u16),
    pub size: u16,
    pub color: Color,
    pub spot_size: u16,
    pub spot_color: Color,
}

#[derive(Debug)]
pub enum Grid {
    Triangle,
    Square,
    Hexagonal,
}
