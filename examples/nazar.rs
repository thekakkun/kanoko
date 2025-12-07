use kanoko::{Canvas, Color, geometry::Angle, point_set::lattice::Lattice, shape::Polygon};
use rand::Rng;

/// An example with multiple layers, colors based off a Nazar amulet.
fn main() {
    let grid = Lattice::new_hexagonal((15, 9), 450.0);
    let mut canvas = Canvas::new((2560.0, 1440.0), Color::from_hex("#fff").unwrap(), grid);

    canvas.add_shape(Polygon::new_static(
        7,
        400.0,
        Angle::default(),
        Color::from_hex("#070d97").unwrap(),
        Some(4.0),
    ));
    canvas.add_shape(Polygon::new_static(
        7,
        300.0,
        Angle::default(),
        Color::from_hex("#fff").unwrap(),
        Some(4.0),
    ));
    canvas.add_shape(Polygon::new_static(
        7,
        200.0,
        Angle::default(),
        Color::from_hex("#73bff1").unwrap(),
        Some(4.0),
    ));
    canvas.add_shape(Polygon::new_static(
        7,
        100.0,
        Angle::default(),
        Color::from_hex("#000").unwrap(),
        Some(4.0),
    ));

    let document = canvas.render(|_| rand::rng().random_bool(0.5));
    svg::save("examples/nazar.svg", &document).unwrap();
}
