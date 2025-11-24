use kanoko::{
    Coordinate, Index, Kanoko, hex_to_alpha_color,
    patterns::kanoko::{Grid, LayerConfig},
};
use rand::{Rng, seq::IndexedRandom};

/// An example with lots of randomization using colors sourced from Gruvbox.
fn main() {
    let mut rng = rand::rng();

    let background_color = hex_to_alpha_color("#282828").unwrap();
    let color_fn = Box::new(|_| {
        let colors = vec!["#98971a", "#458588", "#a89984", "#d79921", "#ebdbb2"];
        hex_to_alpha_color(colors.choose(&mut rand::rng()).unwrap()).unwrap()
    });
    let cell_size = rng.random_range(10.0..200.0);
    let size = rng.random_range(10.0..cell_size);
    let standard_deviation = cell_size * rng.random_range(0.025..0.035);

    let mut kanoko_grid = Kanoko::new(
        Coordinate {
            x: 2560.0,
            y: 1440.0,
        },
        background_color,
        Grid::Diamond,
        Index {
            x: rng.random_range(5..50),
            y: rng.random_range(1..50),
        },
        cell_size,
    );
    kanoko_grid.layers.push(LayerConfig {
        size,
        color_fn,
        standard_deviation: Some(standard_deviation),
    });
    kanoko_grid.layers.push(LayerConfig {
        size: size * rng.random_range(0.1..0.9),
        color_fn: Box::new(move |_| background_color),
        standard_deviation: Some(standard_deviation),
    });

    let document = kanoko_grid.render(|_| rand::rng().random_bool(0.9));
    svg::save("examples/gruvbox.svg", &document).unwrap();
}
