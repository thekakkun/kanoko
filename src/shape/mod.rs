mod polygon;

pub use polygon::KanokoShape as Polygon;

use svg::node::element::Path;

pub trait Shape {
    type Index;

    fn generate_path(&self, index: &Self::Index) -> Path;
}
