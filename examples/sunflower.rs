use kanoko::{Canvas, geometry::Angle, point_set::vogel::Vogel, shape::Polygon};

/// An example using Vogel as the point set
fn main() {
    let points = Vogel::golden_builder()
        .seeds(256)
        .scaling_factor(50.0)
        .build();
    let mut canvas = Canvas::builder()
        .size(2560.0, 1440.0)
        .background_color("#fcba03".try_into().unwrap())
        .points(points)
        .build();

    let polygon_builder = || {
        Polygon::builder()
            .sides(4)
            .rotation_fn(move |n: &usize| Angle::Radian(*n as f64 * points.angle.to_radian()))
    };
    canvas.add_shape(
        polygon_builder()
            .size(100.0)
            .color("#3d2f06".try_into().unwrap())
            .build(),
    );
    canvas.add_shape(
        polygon_builder()
            .size(20.0)
            .color("#dedbd5".try_into().unwrap())
            .build(),
    );

    let document = canvas.render(|_| true);
    svg::save("examples/sunflower.svg", &document).unwrap();
}
