use kanoko::{Canvas, Color, point_set::lattice::Lattice, shape::Polygon};

/// An example with minimal randomization, based on the traditional Japanese tie-dye pattern
fn main() {
    let background_color = "#002e4e".try_into().unwrap();
    let mut canvas_builder = Canvas::builder()
        .canvas_size(2560.0, 1440.0)
        .background_color(background_color)
        .points(Lattice::new_centered_square((19, 16), 90.0));

    canvas_builder.add_shape(
        Polygon::builder()
            .sides(4)
            .size(70.0)
            .color("#f5f5fa".try_into().unwrap())
            .build(),
    );
    canvas_builder.add_shape(
        Polygon::builder()
            .sides(4)
            .size(35.0)
            .color(background_color)
            .build(),
    );

    let canvas = canvas_builder.build();
    let document = canvas.render(|_| true);
    svg::save("examples/kanoko.svg", &document).unwrap();
}
