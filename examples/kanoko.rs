use kanoko::{Canvas, Color, geometry::Angle, point_set::lattice::Lattice, shape::Polygon};

/// An example with minimal randomization, based on the traditional Japanese tie-dye pattern
fn main() {
    let grid = Lattice::new_centered_square((19, 16), 90.0);
    let background_color = Color::from_hex("#002e4e").unwrap();
    let mut canvas = Canvas::new((2560.0, 1440.0), background_color, grid);

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

    let document = canvas.render(|_| true);
    svg::save("examples/kanoko.svg", &document).unwrap();
}
