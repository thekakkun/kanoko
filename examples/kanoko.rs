use kanoko::{
    Canvas,
    geometry::Coordinate,
    hex_to_alpha_color,
    point_set::lattice::{Index, Lattice},
    shape::Polygon,
};

/// An example with minimal randomization, using traditional Japanese tie-dye colors.
fn main() {
    let grid = Lattice::new_centered_square(Index { x: 19, y: 16 }, 90.0);

    let background_color = hex_to_alpha_color("#393c7d").unwrap();
    let mut canvas = Canvas::new(
        Coordinate {
            x: 2560.0,
            y: 1440.0,
        },
        background_color,
        grid,
    );

    canvas.add_shape(Polygon::new(
        4,
        70.0,
        |_| hex_to_alpha_color("#f5f5fa").unwrap(),
        None,
    ));
    canvas.add_shape(Polygon::new(4, 35.0, move |_| background_color, None));

    let document = canvas.render(|_| true);
    svg::save("examples/kanoko.svg", &document).unwrap();
}
