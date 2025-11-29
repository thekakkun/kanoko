mod polygon;

pub use polygon::KanokoShape as Polygon;
use svg::node::element::Path;

#[macro_export]
macro_rules! static_fn {
    ($value:expr) => {
        move |_| $value
    };
}

pub type IndexFn<I, T> = Box<dyn Fn(&I) -> T>;

pub trait Shape {
    type Index;

    fn generate_path(&self, index: &Self::Index) -> Path;
}
