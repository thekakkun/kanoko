use kanoko::{Coordinate, Index, KanokoGrid, hex_to_alpha_color, patterns::kanoko::Grid};
use rand::{Rng, seq::IndexedRandom};

fn main() {
    let background_color = hex_to_alpha_color("#282828").unwrap();
    let color_fn = Box::new(|_| {
        let colors = vec!["#98971a", "#458588", "#a89984", "#d79921", "#ebdbb2"];
        hex_to_alpha_color(colors.choose(&mut rand::rng()).unwrap()).unwrap()
    });

    let kanoko_grid = KanokoGrid {
        canvas_size: Coordinate {
            x: 2560.0,
            y: 1440.0,
        },
        background_color,
        grid: Grid::Diamond,
        grid_size: Index { x: 50, y: 50 },
        cell_size: 50.0,
        size: 50.0,
        color_fn,
        spot_size: 20.0,
        spot_color_fn: Box::new(move |_| background_color),
        standard_deviation: 2.5,
    };

    kanoko_grid.render(|index| index.x as f64 * (&mut rand::rng()).random::<f64>() < 15.0);
}
