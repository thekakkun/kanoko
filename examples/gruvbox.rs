use std::path::PathBuf;

use clap::Parser;
use kanoko::{
    Canvas,
    geometry::{Angle, Coordinate},
    hex_to_alpha_color,
    point_set::lattice::{Index, Lattice},
    shape::Polygon,
    static_fn,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng, seq::IndexedRandom};

#[derive(Parser)]
struct Cli {
    output: Option<PathBuf>,
    width: Option<f64>,
    height: Option<f64>,
}

fn random_sides(index: &Index) -> u8 {
    let seed = (index.x as u64)
        .wrapping_mul(31)
        .wrapping_add(index.y as u64);
    let mut rng = StdRng::seed_from_u64(seed);
    rng.random_range(3..8)
}

/// An example with lots of randomization using colors sourced from Gruvbox.
fn main() {
    let cli = Cli::parse();
    let mut rng = rand::rng();

    let side = rng.random_range(10.0..200.0);
    let grid = Lattice::new_hexagonal(
        Index {
            x: rng.random_range(5..50),
            y: rng.random_range(1..50),
        },
        side,
    );

    let background_color = hex_to_alpha_color("#282828").unwrap();
    let mut canvas = Canvas::new(
        Coordinate::new(cli.width.unwrap_or(2560.0), cli.width.unwrap_or(1440.0)),
        background_color,
        grid,
    );

    let size = rng.random_range(10.0..grid.a);
    let std_dev = size * rng.random_range(0.02..0.08);

    canvas.add_shape(Polygon::new(
        random_sides,
        move |_| size,
        static_fn!(Angle::Radian(0.0)),
        |_| {
            let colors = ["#98971a", "#458588", "#a89984", "#d79921", "#ebdbb2"];
            hex_to_alpha_color(colors.choose(&mut rand::rng()).unwrap()).unwrap()
        },
        Some(std_dev),
    ));
    let inner_ratio = rng.random_range(0.0..0.9);
    canvas.add_shape(Polygon::new(
        random_sides,
        move |_| size * inner_ratio,
        static_fn!(Angle::Radian(0.0)),
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
