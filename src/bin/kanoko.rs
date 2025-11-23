use color::AlphaColor;
use kanoko::{Index, KanokoGrid, KanokoUnit, patterns::kanoko::Grid};

fn main() {
    let kanoko_grid = KanokoGrid {
        grid: Grid::Diamond,
        grid_size: Index { x: 1, y: 1 },
        cell_size: 100.0,
        background: AlphaColor::WHITE,
        unit: KanokoUnit {
            size: 80.0,
            color: AlphaColor::BLACK,
            spot_size: 40.0,
            spot_color: AlphaColor::WHITE,
            std_dev: 1.0,
        },
    };

    kanoko_grid.render(|_| true);
}
