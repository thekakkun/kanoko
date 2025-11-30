use svg::{
    Document,
    node::element::{Group, Rectangle},
};

use crate::{Color, geometry::Coordinate, point_set::PointSet, shape::Shape};

/// Represents the image to be rendered
pub struct Canvas<I> {
    /// The size of the canvas, in pixels
    pub canvas_size: Coordinate,

    /// The image background [`Color`]
    pub background_color: Color,

    /// The [`PointSet`] used for the image
    pub points: Box<dyn PointSet<Index = I>>,

    /// The list of [`Shape`] to be rendered, ordered from lowest layer to highest
    pub shapes: Vec<Box<dyn Shape<Index = I>>>,
}

impl<I> Canvas<I> {
    pub fn new(
        canvas_size: Coordinate,
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
            .set("viewBox", (0, 0, self.canvas_size.x, self.canvas_size.y))
            .set("width", self.canvas_size.x)
            .set("height", self.canvas_size.y);

        let background = self.render_background();
        document = document.add(background);

        let grid_bb = self.points.bounding_box();
        let grid_offset = (self.canvas_size - grid_bb) / 2.0;

        for index in self.points.index_iter().filter(index_filter) {
            let coordinate = self.points.index_to_coordinate(&index);
            let group_offset = grid_offset + coordinate;
            let group = self.render_shape_group(&index).set(
                "transform",
                format!("translate({},{})", group_offset.x, group_offset.y),
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
            .set("width", self.canvas_size.x)
            .set("height", self.canvas_size.y)
            .set("fill", self.background_color.to_svg_color())
    }

    fn render_shape_group(&self, index: &I) -> Group {
        let mut group = Group::new();

        for shape in &self.shapes {
            group = group.add(shape.generate_path(index));
        }

        group
    }
}
