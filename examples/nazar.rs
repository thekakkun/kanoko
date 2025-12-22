use kanoko::{Canvas, point_set::poisson_disk::PoissonDisk, shape::Polygon};

/// An example using PoissonDisk as the point set, colors based off Nazar amulets
fn main() {
    let size = 144.0;
    let mut canvas_builder = Canvas::builder()
        .size(2560.0, 1440.0)
        .background_color("#fff".try_into().unwrap())
        .points(
            PoissonDisk::builder()
                .size(2560.0, 1440.0)
                .r(size)
                .k(30)
                .build(),
        );

    let polygon_builder = || Polygon::builder().sides(7).cv(0.05);
    canvas_builder.add_shape(
        polygon_builder()
            .size(size)
            .color("#070d97".try_into().unwrap())
            .build(),
    );
    canvas_builder.add_shape(
        polygon_builder()
            .size(size * 3.0 / 4.0)
            .color("#fff".try_into().unwrap())
            .build(),
    );
    canvas_builder.add_shape(
        polygon_builder()
            .size(size / 2.0)
            .color("#73bff1".try_into().unwrap())
            .build(),
    );
    canvas_builder.add_shape(
        polygon_builder()
            .size(size / 4.0)
            .color("#000".try_into().unwrap())
            .build(),
    );

    let canvas = canvas_builder.build();
    let document = canvas.render(|_| true);
    svg::save("examples/nazar.svg", &document).unwrap();
}
