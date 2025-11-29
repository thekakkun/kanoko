use kanoko::{
    Canvas, Color,
    geometry::{Angle, Coordinate},
    point_set::lattice::{Index, Lattice},
    shape::Polygon,
};
use rand::Rng;

/// An example of using multiple layers to create patterns based on a nazar amulet.
fn main() {
    let grid = Lattice::new_hexagonal(Index { x: 15, y: 9 }, 450.0);

    let mut canvas = Canvas::new(
        Coordinate::new(2560.0, 1440.0),
        Color::from_hex("#fff").unwrap(),
        grid,
    );

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
