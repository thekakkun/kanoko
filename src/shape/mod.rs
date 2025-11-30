mod polygon;

pub use polygon::Polygon;
use svg::node::element::Path;

/// A utility macro to create a function that returns a static value
#[macro_export]
macro_rules! static_fn {
    ($value:expr) => {
        move |_| $value
    };
}

pub type IndexFn<I, T> = Box<dyn Fn(&I) -> T>;

/// A trait for defining a shape
pub trait Shape {
    type Index;

    /// Generate an SVG `path` for a given `index`
    ///
    /// This `index` can be used by implementers to control the generated shape based on where it
    /// is in the [`PointSet`](crate::point_set::PointSet).
    fn generate_path(&self, index: &Self::Index) -> Path;
}
