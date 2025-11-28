use rand_distr::Distribution;
use std::{collections::VecDeque, f64::consts::PI};
use svg::node::element::{Path, path::Data};

use color::{AlphaColor, Srgb};
use itertools::Itertools;
use rand_distr::Normal;

use crate::{
    geometry::{Angle, Coordinate},
    shape::Shape,
};

pub struct KanokoShape<I> {
    pub sides_fn: Box<dyn Fn(&I) -> u8>,
    pub size_fn: Box<dyn Fn(&I) -> f64>,
    pub rotation_fn: Box<dyn Fn(&I) -> Angle>,
    pub color_fn: Box<dyn Fn(&I) -> AlphaColor<Srgb>>,
    pub std_dev: Option<f64>,
}

impl<I> KanokoShape<I> {
    pub fn new(
        sides_fn: impl Fn(&I) -> u8 + 'static,
        size_fn: impl Fn(&I) -> f64 + 'static,
        rotation_fn: impl Fn(&I) -> Angle + 'static,
        color_fn: impl Fn(&I) -> AlphaColor<Srgb> + 'static,
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
        color: AlphaColor<Srgb>,
        std_dev: Option<f64>,
    ) -> Self {
        Self::new(
            move |_| sides,
            move |_| size,
            move |_| rotation,
            move |_| color,
            std_dev,
        )
    }

    fn generate_corner_coordinates(&self, index: &I) -> Vec<Coordinate> {
        let sides = (self.sides_fn)(index);
        (0..sides)
            .map(|i| {
                let angle: f64 =
                    2_f64 * PI * i as f64 / sides as f64 + (self.rotation_fn)(index).to_radian();
                let coordinate = Coordinate {
                    x: (self.size_fn)(index) * angle.sin() / 2.0,
                    y: -(self.size_fn)(index) * angle.cos() / 2.0,
                };

                if let Some(std_dev) = self.std_dev {
                    coordinate.add_jitter(std_dev)
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
        let [r, g, b, a] = color.to_rgba8().to_u8_array();
        let fill = format!("rgb({},{},{})", r, g, b);
        let opacity = a as f64 / 255.0;

        Path::new()
            .set("stroke", "none")
            .set("d", data)
            .set("fill", fill)
            .set("fill-opacity", opacity)
    }
}
