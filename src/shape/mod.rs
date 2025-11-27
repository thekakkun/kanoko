mod kanoko;

pub use kanoko::KanokoShape;

use svg::node::element::Path;

pub trait Shape {
    type Index;

    fn generate_path(&self, index: &Self::Index) -> Path;
}
