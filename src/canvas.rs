use color::{AlphaColor, Srgb};
use svg::{
    Document,
    node::element::{Group, Rectangle},
};

use crate::{geometry::Coordinate, point_set::PointSet, shape::Shape};

pub struct Canvas<I> {
    pub canvas_size: Coordinate,
    pub background_color: AlphaColor<Srgb>,

    pub grid: Box<dyn PointSet<Index = I>>,
    pub shapes: Vec<Box<dyn Shape<Index = I>>>,
}

impl<I> Canvas<I> {
    pub fn new(
        canvas_size: Coordinate,
        background_color: AlphaColor<Srgb>,
        grid: impl PointSet<Index = I> + 'static,
    ) -> Self {
        Self {
            canvas_size,
            background_color,
            grid: Box::new(grid),
            shapes: Vec::new(),
        }
    }

    pub fn render(&self, index_filter: impl Fn(&I) -> bool) -> Document {
        let mut document = Document::new()
            .set("viewBox", (0, 0, self.canvas_size.x, self.canvas_size.y))
            .set("width", self.canvas_size.x)
            .set("height", self.canvas_size.y);

        let background = self.render_background();
        document = document.add(background);

        let grid_bb = self.grid.bounding_box();
        let grid_offset = (self.canvas_size - grid_bb) / 2.0;

        for index in self.grid.index_iter().filter(index_filter) {
            let coordinate = self.grid.index_to_coordinate(&index);
            let group_offset = grid_offset + coordinate;
            let group = self.render_shape_group(&index).set(
                "transform",
                format!("translate({},{})", group_offset.x, group_offset.y),
            );
            document = document.add(group);
        }

        document
    }

    pub fn add_shape(&mut self, shape: impl Shape<Index = I> + 'static) {
        self.shapes.push(Box::new(shape));
    }

    fn render_background(&self) -> Rectangle {
        let [r, g, b, a] = self.background_color.to_rgba8().to_u8_array();
        let opacity = a as f64 / 255.0;

        Rectangle::new()
            .set("width", self.canvas_size.x)
            .set("height", self.canvas_size.y)
            .set("fill", format!("rgb({},{},{})", r, g, b))
            .set("fill-opacity", opacity)
    }

    fn render_shape_group(&self, index: &I) -> Group {
        let mut group = Group::new();

        for shape in &self.shapes {
            group = group.add(shape.generate_path(index));
        }

        group
    }
}
