use std::path::PathBuf;

use clap::Parser;
use kanoko::{
    Canvas,
    geometry::Coordinate,
    hex_to_alpha_color,
    point_set::lattice::{Index, Lattice},
    shape::Polygon,
};
use rand::{Rng, seq::IndexedRandom};

#[derive(Parser)]
struct Cli {
    output: Option<PathBuf>,
    width: Option<f64>,
    height: Option<f64>,
}

/// An example with lots of randomization using colors sourced from Gruvbox.
fn main() {
    let cli = Cli::parse();
    let mut rng = rand::rng();

    let grid = Lattice {
        grid_size: Index {
            x: rng.random_range(5..50),
            y: rng.random_range(1..50),
        },
        cell_size: rng.random_range(10.0..200.0),
    };

    let background_color = hex_to_alpha_color("#282828").unwrap();
    let mut canvas = Canvas::new(
        Coordinate {
            x: cli.width.unwrap_or(2560.0),
            y: cli.width.unwrap_or(1440.0),
        },
        background_color,
        grid,
    );

    let size = rng.random_range(10.0..grid.cell_size);
    let std_dev = grid.cell_size * rng.random_range(0.01..0.05);
    canvas.add_shape(Polygon::new(
        4,
        size,
        |_| {
            let colors = ["#98971a", "#458588", "#a89984", "#d79921", "#ebdbb2"];
            hex_to_alpha_color(colors.choose(&mut rand::rng()).unwrap()).unwrap()
        },
        Some(std_dev),
    ));
    let inner_ratio = rng.random_range(0.1..0.9);
    canvas.add_shape(Polygon::new(
        4,
        size * inner_ratio,
        move |_| background_color,
        Some(std_dev * inner_ratio),
    ));

    let document = canvas.render(|_| rand::rng().random_bool(0.9));

    svg::save(
        cli.output.unwrap_or(PathBuf::from("examples/gruvbox.svg")),
        &document,
    )
    .unwrap();
}
