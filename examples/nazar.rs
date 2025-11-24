use color::AlphaColor;
use kanoko::{
    Coordinate, Index, Kanoko, hex_to_alpha_color,
    patterns::kanoko::{Grid, LayerConfig},
};
use rand::Rng;

/// An example of using multiple layers to create patterns based on a nazar amulet.
fn main() {
    let mut kanoko_grid = Kanoko::new(
        Coordinate {
            x: 2560.0,
            y: 1440.0,
        },
        AlphaColor::WHITE,
        Grid::Diamond,
        Index { x: 5, y: 5 },
        200.0,
    );
    kanoko_grid.layers.push(LayerConfig {
        size: 200.0,
        color_fn: Box::new(|_| hex_to_alpha_color("#070d97").unwrap()),
        standard_deviation: Some(8.0),
    });
    kanoko_grid.layers.push(LayerConfig {
        size: 150.0,
        color_fn: Box::new(|_| AlphaColor::WHITE),
        standard_deviation: Some(8.0),
    });
    kanoko_grid.layers.push(LayerConfig {
        size: 100.0,
        color_fn: Box::new(|_| hex_to_alpha_color("#73bff1").unwrap()),
        standard_deviation: Some(8.0),
    });
    kanoko_grid.layers.push(LayerConfig {
        size: 50.0,
        color_fn: Box::new(|_| AlphaColor::BLACK),
        standard_deviation: Some(8.0),
    });

    let document = kanoko_grid.render(|_| rand::rng().random_bool(0.8));
    svg::save("examples/nazar.svg", &document).unwrap();
}
