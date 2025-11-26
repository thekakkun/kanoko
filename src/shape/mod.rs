mod kanoko;

pub use kanoko::KanokoShape;

use crate::grid::Index;
use svg::node::element::Path;

pub trait Shape {
    fn generate_path(&self, index: &Index) -> Path;
}
