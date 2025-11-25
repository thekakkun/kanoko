use itertools::{Itertools, iproduct};
use rand_distr::Distribution;
use std::{
    collections::VecDeque,
    f64::consts::{PI, SQRT_2},
    ops::Add,
};
use svg::{
    Document,
    node::element::{Group, Path, Rectangle, path::Data},
};

use color::{AlphaColor, Srgb};
use rand_distr::Normal;

#[derive(Debug, Clone, Copy)]
pub enum Grid {
    Triangle = 3,
    Diamond = 4,
    Hexagon = 6,
}

impl From<Grid> for f64 {
    fn from(value: Grid) -> Self {
        value as u8 as f64
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Index {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

impl Coordinate {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        Self {
            x: self.x + t * (other.x - self.x),
            y: self.y + t * (other.y - self.y),
        }
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub struct SpotConfig {
    pub size: f64,
    pub color_fn: Box<dyn Fn(Index) -> AlphaColor<Srgb>>,
    pub standard_deviation: Option<f64>,
}

pub struct Kanoko {
    // Canvas dimensions
    pub canvas_size: Coordinate,
    pub background_color: AlphaColor<Srgb>,

    // Grid configuration
    pub grid: Grid,
    pub grid_size: Index,
    pub cell_size: f64,

    // Spots
    pub spots: Vec<SpotConfig>,
}

impl Default for Kanoko {
    fn default() -> Self {
        Self {
            canvas_size: Coordinate { x: 800.0, y: 600.0 },
            background_color: AlphaColor::BLACK,
            grid: Grid::Diamond,
            grid_size: Index { x: 10, y: 10 },
            cell_size: 20.0,
            spots: vec![],
        }
    }
}

impl Kanoko {
    pub fn render(&self, index_filter: impl Fn(Index) -> bool) -> Document {
        let [r, g, b, a] = self.background_color.to_rgba8().to_u8_array();
        let opacity = a as f64 / 255.0;

        let mut document = Document::new()
            .set("viewBox", (0, 0, self.canvas_size.x, self.canvas_size.y))
            .set("width", self.canvas_size.x)
            .set("height", self.canvas_size.y);

        let background_rect = Rectangle::new()
            .set("width", self.canvas_size.x)
            .set("height", self.canvas_size.y)
            .set("fill", format!("rgb({},{},{})", r, g, b))
            .set("fill-opacity", opacity);

        document = document.add(background_rect);

        let grid_size = self.calculate_grid_dimensions();
        let offset_x = (self.canvas_size.x - grid_size.x) / 2.0;
        let offset_y = (self.canvas_size.y - grid_size.y) / 2.0;

        for (x, y) in iproduct!(0..self.grid_size.x, 0..self.grid_size.y) {
            let index = Index { x, y };
            if index_filter(index) {
                let coordinate = self.index_to_coordinate(&index);
                let group = self.generate_group(&index).set(
                    "transform",
                    format!(
                        "translate({},{})",
                        coordinate.x + offset_x,
                        coordinate.y + offset_y
                    ),
                );

                document = document.add(group)
            }
        }

        document
    }

    fn calculate_grid_dimensions(&self) -> Coordinate {
        match self.grid {
            Grid::Triangle => todo!(),
            Grid::Diamond => {
                let max_x = 2.0 * (self.grid_size.x - 1) as f64 * self.cell_size * SQRT_2
                    + (self.grid_size.y.min(2) - 1) as f64 * 2.0 * self.cell_size / SQRT_2;
                let max_y = 2.0 * (self.grid_size.y - 1) as f64 * self.cell_size / SQRT_2;

                Coordinate { x: max_x, y: max_y }
            }
            Grid::Hexagon => todo!(),
        }
    }

    fn index_to_coordinate(&self, index: &Index) -> Coordinate {
        match self.grid {
            Grid::Triangle => todo!(),
            Grid::Diamond => {
                let x = 2.0 * index.x as f64 * self.cell_size * SQRT_2
                    + (index.y % 2) as f64 * 2.0 * self.cell_size / SQRT_2;
                let y = 2.0 * index.y as f64 * self.cell_size / SQRT_2;

                Coordinate { x, y }
            }
            Grid::Hexagon => todo!(),
        }
    }

    fn generate_group(&self, index: &Index) -> Group {
        let mut group = Group::new();

        for spot in &self.spots {
            let color = (spot.color_fn)(*index);
            let [r, g, b, a] = color.to_rgba8().to_u8_array();
            let fill = format!("rgb({},{},{})", r, g, b);
            let opacity = a as f64 / 255.0;

            group = group.add(
                self.generate_path(spot)
                    .set("fill", fill)
                    .set("fill-opacity", opacity),
            );
        }

        group
    }

    fn generate_path(&self, spot: &SpotConfig) -> Path {
        let corner_coordinates = self.generate_corner_coordinates(spot);
        let side_coordinates = self.generate_side_coordinates(&corner_coordinates);

        let mut data = Data::new();

        if let Some(first) = side_coordinates.first() {
            data = data.move_to((first.x, first.y));
        }

        for (end, corner) in side_coordinates
            .iter()
            .skip(1)
            .chain(side_coordinates.first())
            .zip(corner_coordinates.iter())
        {
            data = data.cubic_curve_to((corner.x, corner.y, corner.x, corner.y, end.x, end.y));
        }

        data = data.close();

        Path::new().set("stroke", "none").set("d", data)
    }

    fn generate_corner_coordinates(&self, spot: &SpotConfig) -> Vec<Coordinate> {
        (1..=self.grid as u8)
            .map(|i| {
                let angle: f64 = 2_f64 * PI * i as f64 / f64::from(self.grid);
                let coordinate = Coordinate {
                    x: spot.size * angle.cos(),
                    y: spot.size * angle.sin(),
                };

                if let Some(standard_deviation) = spot.standard_deviation {
                    self.add_jitter(coordinate, standard_deviation)
                } else {
                    coordinate
                }
            })
            .collect()
    }

    fn generate_side_coordinates(&self, corner_coordinates: &[Coordinate]) -> Vec<Coordinate> {
        let normal = Normal::new(0.5, 0.1).unwrap();

        let mut side_coordinates: VecDeque<Coordinate> = corner_coordinates
            .iter()
            .circular_tuple_windows()
            .map(|(coordinate_1, coordinate_2)| {
                coordinate_1.lerp(
                    coordinate_2,
                    (normal.sample(&mut rand::rng()) as f64).clamp(0.1, 0.9),
                )
            })
            .collect();

        if let Some(last) = side_coordinates.pop_back() {
            side_coordinates.push_front(last);
        }

        side_coordinates.into()
    }

    fn add_jitter(&self, coordinate: Coordinate, standard_deviation: f64) -> Coordinate {
        let normal = Normal::new(0.0, standard_deviation).unwrap();
        let jitter = Coordinate {
            x: normal.sample(&mut rand::rng()),
            y: normal.sample(&mut rand::rng()),
        };

        coordinate + jitter
    }
}
