use color::AlphaColor;
use kanoko::{
    Canvas,
    geometry::{Angle, Coordinate},
    hex_to_alpha_color,
    point_set::lattice::{Index, Lattice},
    shape::Polygon,
};
use rand::Rng;

/// An example of using multiple layers to create patterns based on a nazar amulet.
fn main() {
    let grid = Lattice::new_hexagonal(Index { x: 15, y: 9 }, 450.0);

    let mut canvas = Canvas::new(Coordinate::new(2560.0, 1440.0), AlphaColor::WHITE, grid);

    canvas.add_shape(Polygon::new_static(
        7,
        400.0,
        Angle::default(),
        hex_to_alpha_color("#070d97").unwrap(),
        Some(4.0),
    ));
    canvas.add_shape(Polygon::new_static(
        7,
        300.0,
        Angle::default(),
        AlphaColor::WHITE,
        Some(4.0),
    ));
    canvas.add_shape(Polygon::new_static(
        7,
        200.0,
        Angle::default(),
        hex_to_alpha_color("#73bff1").unwrap(),
        Some(4.0),
    ));
    canvas.add_shape(Polygon::new_static(
        7,
        100.0,
        Angle::default(),
        AlphaColor::BLACK,
        Some(4.0),
    ));

    let document = canvas.render(|_| rand::rng().random_bool(0.5));
    svg::save("examples/nazar.svg", &document).unwrap();
}
