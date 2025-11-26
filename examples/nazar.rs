use color::AlphaColor;
use kanoko::{
    Canvas, Coordinate,
    grid::{DiamondGrid, Index},
    hex_to_alpha_color,
    shape::KanokoShape,
};
use rand::Rng;

/// An example of using multiple layers to create patterns based on a nazar amulet.
fn main() {
    let grid = DiamondGrid {
        grid_size: Index { x: 5, y: 5 },
        cell_size: 200.0,
    };

    let mut canvas = Canvas::new(
        Coordinate {
            x: 2560.0,
            y: 1440.0,
        },
        AlphaColor::WHITE,
        grid,
    );

    canvas.add_shape(KanokoShape::new(
        200.0,
        |_| hex_to_alpha_color("#070d97").unwrap(),
        Some(8.0),
    ));
    canvas.add_shape(KanokoShape::new(
        150.0,
        |_| AlphaColor::WHITE,
        Some(8.0),
    ));
    canvas.add_shape(KanokoShape::new(
        100.0,
        |_| hex_to_alpha_color("#73bff1").unwrap(),
        Some(8.0),
    ));
    canvas.add_shape(KanokoShape::new(
        50.0,
        |_| AlphaColor::BLACK,
        Some(8.0),
    ));

    let document = canvas.render(|_| rand::rng().random_bool(0.8));
    svg::save("examples/nazar.svg", &document).unwrap();
}
