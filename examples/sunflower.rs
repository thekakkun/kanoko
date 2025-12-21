use kanoko::{Canvas, geometry::Angle, point_set::vogel::Vogel, shape::Polygon};

/// An example using Vogel as the point set
fn main() {
    let mut canvas = Canvas::builder()
        .canvas_size(2560.0, 1440.0)
        .background_color("#fcba03".try_into().unwrap())
        .points(Vogel::new_golden(256, 50.0))
        .build();

    let rotation_fn = move |n: &usize| Angle::Radian(*n as f64 * canvas.points.angle.to_radian());
    canvas.add_shape(
        Polygon::builder()
            .sides(4)
            .size(100.0)
            .rotation_fn(rotation_fn)
            .color("#3d2f06".try_into().unwrap())
            .build(),
    );
    canvas.add_shape(
        Polygon::builder()
            .sides(4)
            .size(20.0)
            .rotation_fn(rotation_fn)
            .color("#dedbd5".try_into().unwrap())
            .build(),
    );

    let document = canvas.render(|_| true);
    svg::save("examples/sunflower.svg", &document).unwrap();
}
