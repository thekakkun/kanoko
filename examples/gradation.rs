use kanoko::{
    Canvas, Color,
    point_set::lattice::{Index, Lattice},
    shape::Polygon,
};

/// An example where the shapes are varied depending on the index.
fn main() {
    let background_color = "#ddd".try_into().unwrap();
    let mut canvas_builder = Canvas::builder()
        .size(2560.0, 1440.0)
        .background_color(background_color)
        .points(
            Lattice::rectangular_builder()
                .grid_size(7, 5)
                .len_a(300.0)
                .len_b(200.0)
                .build(),
        );

    canvas_builder.add_shape(
        Polygon::builder()
            .sides_fn(|Index { u, .. }| *u as u8 + 3)
            .size_fn(|Index { v, .. }| *v as f64 * 35.0 + 80.0)
            .color_fn(|Index { u, v }| Color::new((*u + 1) as u8 * 25, 0, (*v + 1) as u8 * 25, 255))
            .build(),
    );

    let canvas = canvas_builder.build();
    let document = canvas.render(|_| true);
    svg::save("examples/gradation.svg", &document).unwrap();
}
