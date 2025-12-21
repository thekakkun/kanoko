use svg::{
    Document,
    node::element::{Group, Rectangle},
};

use crate::{Color, geometry::Coordinate, point_set::PointSet, shape::Shape};

/// Represents the image to be rendered
#[derive(bon::Builder)]
pub struct Canvas<I> {
    /// The list of [`Shape`] to be rendered, ordered from lowest layer to highest
    #[builder(field)]
    pub shapes: Vec<Box<dyn Shape<Index = I>>>,

    /// The size of the canvas, in pixels
    #[builder(with = |x: f64, y: f64| ( x, y ))]
    pub canvas_size: (f64, f64),

    /// The image background [`Color`]
    pub background_color: Color,

    /// The [`PointSet`] used for the image
    #[builder(with = |points:impl PointSet<Index=I> + 'static| Box::new(points))]
    pub points: Box<dyn PointSet<Index = I>>,
}

impl<I> Canvas<I> {
    pub fn new(
        canvas_size: (f64, f64),
        background_color: Color,
        grid: impl PointSet<Index = I> + 'static,
    ) -> Self {
        Self {
            canvas_size,
            background_color,
            points: Box::new(grid),
            shapes: Vec::new(),
        }
    }

    /// Render the SVG document
    ///
    /// `index_filter` can be used to only render the shapes at a given `Index` if it returns
    /// `true`.
    pub fn render(&self, index_filter: impl Fn(&I) -> bool) -> Document {
        let mut document = Document::new()
            .set("viewBox", (0, 0, self.canvas_size.0, self.canvas_size.1))
            .set("width", self.canvas_size.0)
            .set("height", self.canvas_size.1);

        let background = self.render_background();
        document = document.add(background);

        let (bb_min, bb_max) = self.points.bounding_box();
        let span = bb_max - bb_min;

        let grid_offset = (Coordinate::Cartesian {
            x: self.canvas_size.0,
            y: self.canvas_size.1,
        } - span)
            / 2.0;

        for index in self.points.index_iter().filter(index_filter) {
            let coordinate = self.points.index_to_coordinate(&index);
            let (offset_x, offset_y) = (grid_offset + coordinate - bb_min).to_cartesian();
            let group = self.render_shape_group(&index).set(
                "transform",
                format!("translate({:.3},{:.3})", offset_x, offset_y),
            );
            document = document.add(group);
        }

        document
    }

    /// Add a shape on top of the `shapes` vec
    pub fn add_shape(&mut self, shape: impl Shape<Index = I> + 'static) {
        self.shapes.push(Box::new(shape));
    }

    fn render_background(&self) -> Rectangle {
        Rectangle::new()
            .set("width", self.canvas_size.0)
            .set("height", self.canvas_size.1)
            .set("fill", self.background_color.to_svg_color())
            .set(
                "fill-opacity",
                format!("{:.3}", self.background_color.to_opacity_percent()),
            )
    }

    fn render_shape_group(&self, index: &I) -> Group {
        let mut group = Group::new();

        for shape in &self.shapes {
            group = group.add(shape.generate_path(index));
        }

        group
    }
}

impl<I, S: canvas_builder::State> CanvasBuilder<I, S> {
    pub fn add_shape(&mut self, shape: impl Shape<Index = I> + 'static) -> &mut Self {
        self.shapes.push(Box::new(shape));
        self
    }
}
