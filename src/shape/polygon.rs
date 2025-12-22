//! A polygonal shape with rounded corners
use rand_distr::{Distribution, multi::Dirichlet};
use std::f64::consts::PI;
use svg::node::element::{Path, path::Data};

use itertools::Itertools;
use rand_distr::Normal;

use crate::{
    Color,
    geometry::{Angle, Coordinate},
    shape::{IndexFn, Shape},
};
use polygon_builder::{IsUnset, SetColorFn, SetCvFn, SetRotationFn, SetSidesFn, SetSizeFn, State};

/// A polygonal shape with rounded corners
///
/// Most of its fields are defined as functions of the `Index`. This allows the polygon to be
/// rendered dynamically depending on where it is in the image.
#[derive(bon::Builder)]
pub struct Polygon<I> {
    /// The number of sides in the polygon
    #[builder(with = |func: impl Fn(&I) -> u8 + 'static| Box::new(func))]
    pub sides_fn: IndexFn<I, u8>,

    /// The size of the polygon
    ///
    /// This is the diameter of the circle that the vertices of the polygon would lie on.
    #[builder(with = |func: impl Fn(&I) -> f64 + 'static| Box::new(func))]
    pub size_fn: IndexFn<I, f64>,

    /// The rotation of the polygon
    ///
    /// With no rotation, the shape is rendered "pointy side up".
    #[builder(
        default = (Box::new(|_|Angle::default())),
        with = |func: impl Fn(&I) -> Angle + 'static| Box::new(func)
    )]
    // #[builder(with = |func: impl  Fn(&I) -> Angle + 'static| Box::new(func) as IndexFn<I, Angle>)]
    pub rotation_fn: IndexFn<I, Angle>,

    /// The color of the polygon
    #[builder(with = |func: impl Fn(&I) -> Color + 'static| Box::new(func))]
    pub color_fn: IndexFn<I, Color>,

    /// The coefficient of variance used when randomizing the shape of the polygon
    #[builder(with = |func: impl Fn(&I) -> f64 + 'static| Box::new(func))]
    pub cv_fn: Option<IndexFn<I, f64>>,
}

impl<I> Polygon<I> {
    /// Define a new polygon
    pub fn new(
        sides_fn: impl Fn(&I) -> u8 + 'static,
        size_fn: impl Fn(&I) -> f64 + 'static,
        rotation_fn: impl Fn(&I) -> Angle + 'static,
        color_fn: impl Fn(&I) -> Color + 'static,
        cv_fn: Option<impl Fn(&I) -> f64 + 'static>,
    ) -> Self {
        Self {
            sides_fn: Box::new(sides_fn),
            size_fn: Box::new(size_fn),
            rotation_fn: Box::new(rotation_fn),
            color_fn: Box::new(color_fn),
            cv_fn: cv_fn.map(|f| Box::new(f) as Box<dyn Fn(&I) -> f64>),
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
            move |_| sides,
            move |_| size,
            move |_| rotation,
            move |_| color,
            cv.map(|cv| move |_: &I| cv),
        )
    }

    fn generate_corner_coordinates(&self, index: &I) -> Vec<Coordinate> {
        let sides = (self.sides_fn)(index);
        let size = (self.size_fn)(index) / 2.0;
        let rotation = Angle::Radian(-PI / 2.0) + (self.rotation_fn)(index);

        let divisions = if let Some(cv_fn) = &self.cv_fn {
            let cv = cv_fn(index);
            let alpha = (f64::from(sides) - 1_f64 - cv.powi(2)) / (f64::from(sides) * cv.powi(2));
            let params = vec![alpha; sides as usize];
            let dirichlet = Dirichlet::new(&params).unwrap();
            dirichlet.sample(&mut rand::rng())
        } else {
            vec![1.0 / f64::from(sides); sides as usize]
        };

        divisions
            .iter()
            .scan(Angle::Radian(0.0), |state, &x| {
                *state += Angle::Radian(x * 2.0 * PI);
                Some(*state)
            })
            .map(|theta| {
                let r = if let Some(cv_fn) = &self.cv_fn {
                    let cv = cv_fn(index);
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
            let (x, y) = first.to_rounded_cartesian(3);
            data = data.move_to((x, y));

            for (end, corner) in side_coordinates
                .iter()
                .skip(1)
                .chain(side_coordinates.first())
                .zip(corner_coordinates.iter())
            {
                let (end_x, end_y) = end.to_rounded_cartesian(3);
                let (corner_x, corner_y) = corner.to_rounded_cartesian(3);
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

impl<I, S: State> PolygonBuilder<I, S> {
    pub fn sides(self, sides: u8) -> PolygonBuilder<I, SetSidesFn<S>>
    where
        S::SidesFn: IsUnset,
    {
        self.sides_fn(move |_| sides)
    }

    pub fn size(self, size: f64) -> PolygonBuilder<I, SetSizeFn<S>>
    where
        S::SizeFn: IsUnset,
    {
        self.size_fn(move |_| size)
    }

    pub fn rotation(self, rotation: Angle) -> PolygonBuilder<I, SetRotationFn<S>>
    where
        S::RotationFn: IsUnset,
    {
        self.rotation_fn(move |_| rotation)
    }

    pub fn color(self, color: Color) -> PolygonBuilder<I, SetColorFn<S>>
    where
        S::ColorFn: IsUnset,
    {
        self.color_fn(move |_| color)
    }
    pub fn cv(self, cv: f64) -> PolygonBuilder<I, SetCvFn<S>>
    where
        S::CvFn: IsUnset,
    {
        self.cv_fn(move |_| cv)
    }
}
