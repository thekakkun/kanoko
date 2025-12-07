use kanoko::{
    Canvas, Color, geometry::Angle, point_set::lattice::Lattice, shape::Polygon, static_fn,
};
use rand::{Rng, seq::IndexedRandom};

/// An example with lots of randomimzation, using colors sourced from Gruvbox.
fn main() {
    let mut rng = rand::rng();

    let grid = Lattice::new(
        (rng.random_range(5..50), rng.random_range(1..50)),
        rng.random_range(100.0..400.0),
        rng.random_range(100.0..400.0),
        Angle::Degree(rng.random_range(0.0..90.0)),
    );
    let background_color = Color::from_hex("#282828").unwrap();
    let mut canvas = Canvas::new((2560.0, 1440.0), background_color, grid);

    let size = rng
        .random_range(100.0..grid.a)
        .min(rng.random_range(100.0..grid.b) * grid.theta.to_radian().sin());

    canvas.add_shape(Polygon::new(
        |_| rand::rng().random_range(3..8),
        move |_| size,
        static_fn!(Angle::default()),
        |_| {
            let colors = ["#98971a", "#458588", "#a89984", "#d79921", "#ebdbb2"];
            Color::from_hex(colors.choose(&mut rand::rng()).unwrap()).unwrap()
        },
        Some(rng.random_range(0.1..0.5) / 6.0),
    ));

    let document = canvas.render(|_| rand::rng().random_bool(0.9));
    svg::save("examples/gruvbox.svg", &document).unwrap();
}
