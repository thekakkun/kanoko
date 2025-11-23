use itertools::Itertools;
use rand_distr::Distribution;
use std::{f64::consts::PI, iter::zip, ops::Add};
use svg::Document;

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
struct Index {
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
    pub fn render(&self, index_filter: Option<impl Fn(Index) -> bool>) {
        todo!()
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
    fn generate_corner_coords(&self, grid: &Grid, size: f64) -> Vec<Coordinate> {
        (1..=*grid as u8)
            .map(|i| {
                let angle = 2_f64 * PI * i as f64 / grid.into_f64();

                Coordinate {
                    x: size * angle.cos(),
                    y: size * angle.sin(),
                }
            })
            .map(|spot| self.add_jitter(spot))
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

    fn generate_side_coords(&self, corner_coords: &Vec<Coordinate>) -> Vec<Coordinate> {
        let normal = Normal::new(0.5, 0.1).unwrap();

        corner_coords
            .iter()
            .circular_tuple_windows()
            .map(|(coord_1, coord_2)| {
                coord_1.lerp(
                    coord_2,
                    (normal.sample(&mut rand::rng()) as f64).clamp(0.0, 1.0),
                )
            })
            .collect()
    }
}
