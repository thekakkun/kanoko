use rand_distr::Distribution;
use std::{collections::VecDeque, f64::consts::PI};
use svg::node::element::{Path, path::Data};

use itertools::Itertools;
use rand_distr::Normal;

use crate::{
    Color,
    geometry::{Angle, Coordinate},
    shape::{IndexFn, Shape},
    static_fn,
};

pub struct KanokoShape<I> {
    pub sides_fn: IndexFn<I, u8>,
    pub size_fn: IndexFn<I, f64>,
    pub rotation_fn: IndexFn<I, Angle>,
    pub color_fn: IndexFn<I, Color>,
    pub std_dev: Option<f64>,
}

impl<I> KanokoShape<I> {
    pub fn new(
        sides_fn: impl Fn(&I) -> u8 + 'static,
        size_fn: impl Fn(&I) -> f64 + 'static,
        rotation_fn: impl Fn(&I) -> Angle + 'static,
        color_fn: impl Fn(&I) -> Color + 'static,
        std_dev: Option<f64>,
    ) -> Self {
        Self {
            sides_fn: Box::new(sides_fn),
            size_fn: Box::new(size_fn),
            rotation_fn: Box::new(rotation_fn),
            color_fn: Box::new(color_fn),
            std_dev,
        }
    }

    pub fn new_static(
        sides: u8,
        size: f64,
        rotation: Angle,
        color: Color,
        std_dev: Option<f64>,
    ) -> Self {
        Self::new(
            static_fn!(sides),
            static_fn!(size),
            static_fn!(rotation),
            static_fn!(color),
            std_dev,
        )
    }

    fn generate_corner_coordinates(&self, index: &I) -> Vec<Coordinate> {
        let sides = (self.sides_fn)(index);
        let size = (self.size_fn)(index);
        let rotation = (self.rotation_fn)(index).to_radian();

        let mut corners = Vec::with_capacity(sides as usize);
        for i in 0..sides {
            let angle = 2.0 * PI * i as f64 / sides as f64 + rotation;
            let coordinate = Coordinate {
                x: size * angle.sin() / 2.0,
                y: -size * angle.cos() / 2.0,
            };

            corners.push(if let Some(std_dev) = self.std_dev {
                coordinate.add_jitter(std_dev)
            } else {
                coordinate
            });
        }
        corners
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
}

impl<I: Copy> Shape for KanokoShape<I> {
    type Index = I;

    fn generate_path(&self, index: &Self::Index) -> Path {
        let corner_coordinates = self.generate_corner_coordinates(index);
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

        let color = (self.color_fn)(index);
        Path::new()
            .set("stroke", "none")
            .set("d", data)
            .set("fill", color.to_svg_color())
    }
}
