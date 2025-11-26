use kanoko::{
    Canvas, Coordinate,
    grid::{DiamondGrid, Index},
    hex_to_alpha_color,
    shape::KanokoShape,
};
use rand::{Rng, seq::IndexedRandom};

/// An example with lots of randomization using colors sourced from Gruvbox.
fn main() {
    let mut rng = rand::rng();

    let background_color = hex_to_alpha_color("#282828").unwrap();

    let grid = DiamondGrid {
        grid_size: Index {
            x: rng.random_range(5..50),
            y: rng.random_range(1..50),
        },
        cell_size: rng.random_range(10.0..200.0),
    };

    let mut canvas = Canvas::new(
        Coordinate {
            x: 2560.0,
            y: 1440.0,
        },
        background_color,
        Box::new(grid),
    );

    let size = rng.random_range(10.0..grid.cell_size);
    let std_dev = grid.cell_size * rng.random_range(0.025..0.035);
    canvas.add_shape(Box::new(KanokoShape::new(
        size,
        Box::new(|_| {
            let colors = ["#98971a", "#458588", "#a89984", "#d79921", "#ebdbb2"];
            hex_to_alpha_color(colors.choose(&mut rand::rng()).unwrap()).unwrap()
        }),
        Some(std_dev),
    )));
    canvas.add_shape(Box::new(KanokoShape::new(
        size * rng.random_range(0.1..0.9),
        Box::new(move |_| background_color),
        Some(std_dev),
    )));

    let document = canvas.render(|_| rand::rng().random_bool(0.9));
    svg::save("examples/gruvbox.svg", &document).unwrap();
}
