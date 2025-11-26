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
        Box::new(grid),
    );

    canvas.add_shape(Box::new(KanokoShape::new(
        200.0,
        Box::new(|_| hex_to_alpha_color("#070d97").unwrap()),
        Some(8.0),
    )));
    canvas.add_shape(Box::new(KanokoShape::new(
        150.0,
        Box::new(|_| AlphaColor::WHITE),
        Some(8.0),
    )));
    canvas.add_shape(Box::new(KanokoShape::new(
        100.0,
        Box::new(|_| hex_to_alpha_color("#73bff1").unwrap()),
        Some(8.0),
    )));
    canvas.add_shape(Box::new(KanokoShape::new(
        50.0,
        Box::new(|_| AlphaColor::BLACK),
        Some(8.0),
    )));

    let document = canvas.render(|_| rand::rng().random_bool(0.8));
    svg::save("examples/nazar.svg", &document).unwrap();
}
