use kanoko::{
    Canvas, Color, geometry::Angle, point_set::poisson_disk::PoissonDisk, shape::Polygon, static_fn,
};

/// An example with PoissonDisk, colors based off Nazar amulets
fn main() {
    let size = 144.0;
    let grid = PoissonDisk::new((2560.0, 1440.0), size, 30);
    let mut canvas = Canvas::new((2560.0, 1440.0), Color::from_hex("#fff").unwrap(), grid);

    canvas.add_shape(Polygon::new_static(
        7,
        size,
        Angle::default(),
        Color::from_hex("#070d97").unwrap(),
        Some(0.05),
    ));
    canvas.add_shape(Polygon::new_static(
        7,
        size * 3.0 / 4.0,
        Angle::default(),
        Color::from_hex("#fff").unwrap(),
        Some(0.05),
    ));
    canvas.add_shape(Polygon::new_static(
        7,
        size / 2.0,
        Angle::default(),
        Color::from_hex("#73bff1").unwrap(),
        Some(0.05),
    ));
    canvas.add_shape(Polygon::new_static(
        7,
        size / 4.0,
        Angle::default(),
        Color::from_hex("#000").unwrap(),
        Some(0.05),
    ));

    let document = canvas.render(static_fn!(true));
    svg::save("examples/nazar.svg", &document).unwrap();
}
