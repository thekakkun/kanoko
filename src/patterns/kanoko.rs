use itertools::{Itertools, iproduct};
use rand_distr::Distribution;
use std::{f64::consts::PI, iter::zip, ops::Add};
use svg::{
    Document,
    node::element::{Group, Path, path::Data},
};

use color::{AlphaColor, Srgb};
use rand_distr::Normal;

#[derive(Debug, Clone, Copy)]
pub enum Grid {
    Triangle = 3,
    Diamond = 4,
    Hexagon = 6,
}

impl Grid {
    fn into_f64(&self) -> f64 {
        *self as u8 as f64
    }
}

impl Into<f64> for Grid {
    fn into(self) -> f64 {
        self as u8 as f64
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Index {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    pub x: f64,
    pub y: f64,
}

impl Coordinate {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        Self {
            x: self.x + t * (self.x - other.x),
            y: self.y + t * (self.y - other.y),
        }
    }
}

impl From<Coordinate> for (f64, f64) {
    fn from(val: Coordinate) -> Self {
        (val.x, val.y)
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

#[derive(Debug, Clone, Copy)]
pub struct KanokoGrid {
    pub grid: Grid,
    pub grid_size: Index,
    pub cell_size: f64,
    pub background: AlphaColor<Srgb>,
    pub unit: KanokoUnit,
}

impl KanokoGrid {
    pub fn render(&self, index_filter: impl Fn(Index) -> bool) {
        let mut document = Document::new();

        for (x, y) in iproduct!(0..self.grid_size.x, 0..self.grid_size.y) {
            let index = Index { x, y };
            let coord = self.index_to_coord(&index);
            let group = self
                .unit
                .generate_group(&self.grid)
                .set("transform", format!("translate({},{})", coord.x, coord.y));

            document = document.add(group)
        }

        svg::save("image.svg", &document).unwrap();
    }

    fn index_to_coord(&self, index: &Index) -> Coordinate {
        match self.grid {
            Grid::Triangle => todo!(),
            Grid::Diamond => {
                let x = index.x as f64 * self.cell_size / 1_f64.sqrt();
                let y = index.y as f64 * self.cell_size.sqrt()
                    + (index.x % 2) as f64 * self.cell_size / 1_f64.sqrt();

                Coordinate { x, y }
            }
            Grid::Hexagon => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KanokoUnit {
    pub size: f64,
    pub color: AlphaColor<Srgb>,
    pub spot_size: f64,
    pub spot_color: AlphaColor<Srgb>,
    pub std_dev: f64,
}

impl KanokoUnit {
    fn generate_group(&self, grid: &Grid) -> Group {
        let group = Group::new()
            .add(KanokoUnit::generate_path(grid, &self.size).set("fill", "black"))
            .add(KanokoUnit::generate_path(grid, &self.spot_size).set("fill", "white"));

        group
    }
    fn generate_path(grid: &Grid, size: &f64) -> Path {
        let corner_coords = KanokoUnit::generate_corner_coords(grid, size);
        println!("CORNER:{:#?}", corner_coords);
        let side_coords = KanokoUnit::generate_side_coords(&corner_coords);
        println!("{:#?}", side_coords);

        let mut data = Data::new();

        for ((start, end), c) in zip(
            side_coords.iter().circular_tuple_windows(),
            corner_coords.iter(),
        ) {
            data = data
                .move_to((start.x, start.y))
                .cubic_curve_to((c.x, c.y, c.x, c.y, end.x, end.y));
        }

        Path::new().set("stroke", "none").set("d", data)
    }
    fn generate_corner_coords(grid: &Grid, size: &f64) -> Vec<Coordinate> {
        (1..=*grid as u8)
            .map(|i| {
                let angle = 2_f64 * PI * i as f64 / grid.into_f64();

                Coordinate {
                    x: size * angle.cos(),
                    y: size * angle.sin(),
                }
            })
            .collect()
    }

    fn generate_side_coords(corner_coords: &Vec<Coordinate>) -> Vec<Coordinate> {
        // let normal = Normal::new(0.5, 0.1).unwrap();

        corner_coords
            .iter()
            .circular_tuple_windows()
            .map(|(coord_1, coord_2)| {
                coord_1.lerp(
                    coord_2, 0.5, // (normal.sample(&mut rand::rng()) as f64).clamp(0.0, 1.0),
                )
            })
            .collect()
    }

    fn add_jitter(&self, coord: Coordinate) -> Coordinate {
        let normal = Normal::new(0 as f64, self.std_dev).unwrap();
        let jitter = Coordinate {
            x: normal.sample(&mut rand::rng()),
            y: normal.sample(&mut rand::rng()),
        };

        coord + jitter
    }
}
