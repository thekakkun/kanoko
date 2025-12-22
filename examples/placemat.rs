use kanoko::{Canvas, geometry::Angle, point_set::lattice::Lattice, shape::Polygon};
use rand::{Rng, seq::IndexedRandom};

/// An example with lots of randomimzation
fn main() {
    let mut rng = rand::rng();

    let lattice = Lattice::builder()
        .grid_size(rng.random_range(5..50), rng.random_range(1..50))
        .len_a(rng.random_range(100.0..400.0))
        .len_b(rng.random_range(100.0..400.0))
        .theta(Angle::Degree(rng.random_range(0.0..90.0)))
        .build();
    let mut canvas_builder = Canvas::builder()
        .size(2560.0, 1440.0)
        .background_color("#e0d8d1".try_into().unwrap())
        .points(lattice);

    let size = rng
        .random_range(100.0..lattice.len_a)
        .min(rng.random_range(100.0..lattice.len_b) * lattice.theta.to_radian().sin());

    canvas_builder.add_shape(
        Polygon::builder()
            .sides_fn(|_| rand::rng().random_range(3..8))
            .size(size)
            .color_fn(|_| {
                vec![
                    "#6f6e6a", "#b3b4af", "#b09e90", "#bea24e", "#d9bdb9", "#9a9ba0",
                ]
                .choose(&mut rand::rng())
                .copied()
                .unwrap()
                .try_into()
                .unwrap()
            })
            .cv_fn(|_| rand::rng().random_range(0.1..0.5) / 6.0)
            .build(),
    );

    let canvas = canvas_builder.build();

    let document = canvas.render(|_| rand::rng().random_bool(0.9));
    svg::save("examples/placemat.svg", &document).unwrap();
}
