use svg::{
    Document,
    node::element::{Group, Rectangle},
};

use crate::{Color, geometry::Coordinate, point_set::PointSet, shape::Shape};
use canvas_builder::State;

/// Represents the image to be rendered
#[derive(bon::Builder)]
pub struct Canvas<P: PointSet> {
    /// The list of [`Shape`] to be rendered, ordered from lowest layer to highest
    #[builder(field)]
    pub shapes: Vec<Box<dyn Shape<Index = P::Index>>>,

    /// The size of the canvas, in pixels
    #[builder(with = |x: f64, y: f64| (x, y))]
    pub size: (f64, f64),

    /// The image background [`Color`]
    pub background_color: Color,

    // /// The [`PointSet`] used for the image
    pub points: P,
}

impl<P: PointSet> Canvas<P> {
    pub fn new(size: (f64, f64), background_color: Color, points: P) -> Self {
        Self {
            size,
            background_color,
            points,
            shapes: Vec::new(),
        }
    }

    /// Render the SVG document
    ///
    /// `index_filter` can be used to only render the shapes at a given `Index` if it returns
    /// `true`.
    pub fn render(&self, index_filter: impl Fn(&P::Index) -> bool) -> Document {
        let mut document = Document::new()
            .set("viewBox", (0, 0, self.size.0, self.size.1))
            .set("width", self.size.0)
            .set("height", self.size.1);

        let background = self.render_background();
        document = document.add(background);

        let (bb_min, bb_max) = self.points.bounding_box();
        let span = bb_max - bb_min;

        let grid_offset = (Coordinate::Cartesian {
            x: self.size.0,
            y: self.size.1,
        } - span)
            / 2.0;

        for index in self.points.index_iter().filter(index_filter) {
            let coordinate = self.points.index_to_coordinate(&index);
            let (offset_x, offset_y) = (grid_offset + coordinate - bb_min).to_cartesian();
            let group = self.render_shape_group(&index).set(
                "transform",
                format!("translate({offset_x:.3},{offset_y:.3})"),
            );
            document = document.add(group);
        }

        document
    }

    /// Add a shape on top of the `shapes` vec
    pub fn add_shape(&mut self, shape: impl Shape<Index = P::Index> + 'static) {
        self.shapes.push(Box::new(shape));
    }

    fn render_background(&self) -> Rectangle {
        Rectangle::new()
            .set("width", self.size.0)
            .set("height", self.size.1)
            .set("fill", self.background_color.to_svg_color())
            .set(
                "fill-opacity",
                format!("{:.3}", self.background_color.to_opacity_percent()),
            )
    }

    fn render_shape_group(&self, index: &P::Index) -> Group {
        let mut group = Group::new();

        for shape in &self.shapes {
            group = group.add(shape.generate_path(index));
        }

        group
    }
}

impl<P, S> CanvasBuilder<P, S>
where
    P: PointSet,
    S: State,
{
    pub fn add_shape(&mut self, shape: impl Shape<Index = P::Index> + 'static) -> &mut Self {
        self.shapes.push(Box::new(shape));
        self
    }
}
