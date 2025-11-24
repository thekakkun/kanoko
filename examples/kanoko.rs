use color::AlphaColor;
use kanoko::{Index, KanokoGrid, KanokoUnit, patterns::kanoko::Grid};

fn main() {
    let kanoko_grid = KanokoGrid {
        grid: Grid::Diamond,
        grid_size: Index { x: 24, y: 8 },
        cell_size: 120.0,
        background: AlphaColor::WHITE,
        unit: KanokoUnit {
            size: 100.0,
            color: AlphaColor::BLACK,
            spot_size: 30.0,
            spot_color: AlphaColor::WHITE,
            std_dev: 5.0,
        },
    };

    kanoko_grid.render(|_| true);
}
