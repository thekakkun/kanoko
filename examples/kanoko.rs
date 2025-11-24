use kanoko::{
    Coordinate, Index, Kanoko, hex_to_alpha_color,
    patterns::kanoko::{Grid, SpotConfig},
};

/// An example with minimal randomization, using traditional Japanese tie-dye colors.
fn main() {
    let background_color = hex_to_alpha_color("#393c7d").unwrap();

    let mut kanoko_grid = Kanoko {
        canvas_size: Coordinate {
            x: 2560.0,
            y: 1440.0,
        },
        background_color,
        grid: Grid::Diamond,
        grid_size: Index { x: 25, y: 15 },
        cell_size: 30.0,
        ..Default::default()
    };
    kanoko_grid.spots.push(SpotConfig {
        size: 30.0,
        color_fn: Box::new(|_| hex_to_alpha_color("#f5f5fa").unwrap()),
        standard_deviation: None,
    });
    kanoko_grid.spots.push(SpotConfig {
        size: 15.0,
        color_fn: Box::new(move |_| background_color),
        standard_deviation: None,
    });

    let document = kanoko_grid.render(|_| true);
    svg::save("examples/kanoko.svg", &document).unwrap();
}
