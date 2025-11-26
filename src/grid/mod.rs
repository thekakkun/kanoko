mod diamond;

pub use diamond::DiamondGrid;

use crate::Coordinate;

#[derive(Debug, Clone, Copy)]
pub struct Index {
    pub x: u16,
    pub y: u16,
}

pub trait Grid {
    fn index_iter(&self) -> Box<dyn Iterator<Item = Index>>;
    fn index_to_coordinate(&self, index: &Index) -> Coordinate;
    fn bounding_box(&self) -> Coordinate;
}
