pub mod lattice;

use crate::geometry::Coordinate;

pub trait PointSet {
    type Index;

    fn index_iter(&self) -> Box<dyn Iterator<Item = Self::Index>>;
    fn index_to_coordinate(&self, index: &Self::Index) -> Coordinate;
    fn bounding_box(&self) -> Coordinate;
}
