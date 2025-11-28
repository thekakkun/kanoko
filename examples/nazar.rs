use color::AlphaColor;
use kanoko::{
    Canvas,
    geometry::Coordinate,
    hex_to_alpha_color,
    point_set::lattice::{Index, Lattice},
    shape::Polygon,
};
use rand::Rng;

/// An example of using multiple layers to create patterns based on a nazar amulet.
fn main() {
    let grid = Lattice::new_rectangle(Index { x: 15, y: 8 }, 350.0, 175.0);

    let mut canvas = Canvas::new(
        Coordinate {
            x: 2560.0,
            y: 1440.0,
        },
        AlphaColor::WHITE,
        grid,
    );

    canvas.add_shape(Polygon::new(
        7,
        200.0,
        |_| hex_to_alpha_color("#070d97").unwrap(),
        Some(4.0),
    ));
    canvas.add_shape(Polygon::new(7, 150.0, |_| AlphaColor::WHITE, Some(4.0)));
    canvas.add_shape(Polygon::new(
        7,
        100.0,
        |_| hex_to_alpha_color("#73bff1").unwrap(),
        Some(4.0),
    ));
    canvas.add_shape(Polygon::new(7, 50.0, |_| AlphaColor::BLACK, Some(4.0)));

    let document = canvas.render(|_| rand::rng().random_bool(0.5));
    svg::save("examples/nazar.svg", &document).unwrap();
}
