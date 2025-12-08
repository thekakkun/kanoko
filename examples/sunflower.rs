use kanoko::{Canvas, Color, geometry::Angle, point_set::vogel::Vogel, shape::Polygon, static_fn};

/// An example using Vogel as the point set
fn main() {
    let grid = Vogel::new_golden(256, 50.0);
    let background_color = Color::from_hex("#fcba03").unwrap();
    let mut canvas = Canvas::new((2560.0, 1440.0), background_color, grid);

    canvas.add_shape(Polygon::new(
        static_fn!(4),
        static_fn!(100.0),
        move |n| Angle::Radian(*n as f64 * grid.angle.to_radian()),
        static_fn!(Color::from_hex("#3d2f06").unwrap()),
        None,
    ));
    canvas.add_shape(Polygon::new(
        static_fn!(4),
        static_fn!(20.0),
        move |n| Angle::Radian(*n as f64 * grid.angle.to_radian()),
        static_fn!(Color::from_hex("#dedbd5").unwrap()),
        None,
    ));

    let document = canvas.render(static_fn!(true));
    svg::save("examples/sunflower.svg", &document).unwrap();
}
