//! A polygonal shape with rounded corners
use rand_distr::{Distribution, multi::Dirichlet};
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

/// A polygonal shape with rounded corners
///
/// Most of its fields are defined as functions of the `Index`. This allows the polygon to be
/// rendered dynamically depending on where it is in the image.
pub struct Polygon<I> {
    /// The number of sides in the polygon
    pub sides_fn: IndexFn<I, u8>,

    /// The size of the polygon
    ///
    /// This is the diameter of the circle that the vertices of the polygon would lie on.
    pub size_fn: IndexFn<I, f64>,

    /// The rotation of the polygon
    ///
    /// With no rotation, the shape is rendered "pointy side up".
    pub rotation_fn: IndexFn<I, Angle>,

    /// The color of the polygon
    pub color_fn: IndexFn<I, Color>,

    /// The coefficient of variance used when randomizing the shape of the polygon
    pub cv: Option<f64>,
}

impl<I> Polygon<I> {
    /// Define a new polygon
    pub fn new(
        sides_fn: impl Fn(&I) -> u8 + 'static,
        size_fn: impl Fn(&I) -> f64 + 'static,
        rotation_fn: impl Fn(&I) -> Angle + 'static,
        color_fn: impl Fn(&I) -> Color + 'static,
        cv: Option<f64>,
    ) -> Self {
        Self {
            sides_fn: Box::new(sides_fn),
            size_fn: Box::new(size_fn),
            rotation_fn: Box::new(rotation_fn),
            color_fn: Box::new(color_fn),
            cv,
        }
    }

    /// Define a new polygon with static values (no dependency on `Index`)
    pub fn new_static(
        sides: u8,
        size: f64,
        rotation: Angle,
        color: Color,
        cv: Option<f64>,
    ) -> Self {
        Self::new(
            static_fn!(sides),
            static_fn!(size),
            static_fn!(rotation),
            static_fn!(color),
            cv,
        )
    }

    fn generate_corner_coordinates(&self, index: &I) -> Vec<Coordinate> {
        let sides = (self.sides_fn)(index);
        let size = (self.size_fn)(index) / 2.0;
        let rotation = Angle::Radian(-PI / 2.0) + (self.rotation_fn)(index);

        let divisions = if let Some(cv) = self.cv {
            let alpha = (sides as f64 - 1_f64 - cv.powi(2)) / (sides as f64 * cv.powi(2));
            let params = vec![alpha; sides as usize];
            let dirichlet = Dirichlet::new(&params).unwrap();
            dirichlet.sample(&mut rand::rng())
        } else {
            vec![1.0 / sides as f64; sides as usize]
        };

        divisions
            .iter()
            .scan(Angle::Radian(0.0), |state, &x| {
                *state += Angle::Radian(x * 2.0 * PI);
                Some(*state)
            })
            .map(|theta| {
                let r = if let Some(cv) = self.cv {
                    let normal = Normal::new(size, cv * size).unwrap();
                    normal.sample(&mut rand::rng())
                } else {
                    size
                };

                Coordinate::Polar {
                    r,
                    phi: theta + rotation,
                }
            })
            .collect()
    }

    fn generate_side_coordinates(corner_coordinates: &[Coordinate]) -> Vec<Coordinate> {
        let normal = Normal::new(0.5, 0.1).unwrap();
        let mut rng = rand::rng();

        let mut side_coordinates: Vec<_> = corner_coordinates
            .iter()
            .circular_tuple_windows()
            .map(|(c1, c2)| c1.lerp(c2, (normal.sample(&mut rng) as f64).clamp(0.1, 0.9)))
            .collect();
        side_coordinates.rotate_right(1);

        side_coordinates
    }
}

impl<I: Copy> Shape for Polygon<I> {
    type Index = I;

    fn generate_path(&self, index: &Self::Index) -> Path {
        let mut data = Data::new();
        let color = (self.color_fn)(index);

        let corner_coordinates = self.generate_corner_coordinates(index);
        let side_coordinates = Self::generate_side_coordinates(&corner_coordinates);

        if let Some(first) = side_coordinates.first() {
            let (x, y) = first.to_cartesian();
            data = data.move_to((x, y));

            for (end, corner) in side_coordinates
                .iter()
                .skip(1)
                .chain(side_coordinates.first())
                .zip(corner_coordinates.iter())
            {
                let (end_x, end_y) = end.to_cartesian();
                let (corner_x, corner_y) = corner.to_cartesian();
                data = data.cubic_curve_to((corner_x, corner_y, corner_x, corner_y, end_x, end_y));
            }
        }

        Path::new()
            .set("stroke", "none")
            .set("d", data.close())
            .set("fill", color.to_svg_color())
            .set("fill-opacity", color.to_opacity_percent())
    }
}
