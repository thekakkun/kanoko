use kanoko::{
    Canvas, Color,
    geometry::{Angle, Coordinate},
    point_set::lattice::{Index, Lattice},
    shape::Polygon,
    static_fn,
};

/// An example with minimal randomization, using traditional Japanese tie-dye colors.
fn main() {
    let grid = Lattice::new_centered_square(Index { x: 19, y: 16 }, 90.0);

    let background_color = Color::from_hex("#002E4E").unwrap();
    let mut canvas = Canvas::new(Coordinate::new(2560.0, 1440.0), background_color, grid);

    canvas.add_shape(Polygon::new_static(
        4,
        70.0,
        Angle::default(),
        Color::from_hex("#f5f5fa").unwrap(),
        None,
    ));
    canvas.add_shape(Polygon::new_static(
        4,
        35.0,
        Angle::default(),
        background_color,
        None,
    ));

    let document = canvas.render(static_fn!(true));
    svg::save("examples/kanoko.svg", &document).unwrap();
}
