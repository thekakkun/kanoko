use kanoko::{
    Canvas, Coordinate,
    grid::{DiamondGrid, Index},
    hex_to_alpha_color,
    shape::KanokoShape,
};

/// An example with minimal randomization, using traditional Japanese tie-dye colors.
fn main() {
    let grid = DiamondGrid {
        grid_size: Index { x: 19, y: 16 },
        cell_size: 30.0,
    };

    let background_color = hex_to_alpha_color("#393c7d").unwrap();
    let mut canvas = Canvas::new(
        Coordinate {
            x: 2560.0,
            y: 1440.0,
        },
        background_color,
        grid,
    );

    canvas.add_shape(KanokoShape::new(
        30.0,
        |_| hex_to_alpha_color("#f5f5fa").unwrap(),
        None,
    ));
    canvas.add_shape(KanokoShape::new(
        15.0,
        move |_| background_color,
        None,
    ));

    let document = canvas.render(|_| true);
    svg::save("examples/kanoko.svg", &document).unwrap();
}
