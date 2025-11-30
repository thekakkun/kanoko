pub mod lattice;

use crate::geometry::Coordinate;

/// A trait for putting points on a 2D plane.
///
/// Each point is given an [`Index`](PointSet::Index) which can be mapped onto a [`Coordinate`]. The bounding box of
/// all points in the PointSet are used to center it within the [`Canvas`](crate::Canvas).
pub trait PointSet {
    type Index;

    /// Generate an `Index` for each point and iterate through them
    fn index_iter(&self) -> Box<dyn Iterator<Item = Self::Index>>;

    /// Map an `Index` to a `Coordinate`
    ///
    /// This can be independent of the coordinates in the `Canvas`.
    fn index_to_coordinate(&self, index: &Self::Index) -> Coordinate;

    /// The size of the box that would contain all coordinates for the set
    ///
    /// This is used to center the `PointSet` within the `Canvas`.
    fn bounding_box(&self) -> Coordinate;
}
