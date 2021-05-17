use crate::base::*;
use crate::prelude::Point;

#[derive(Debug, Clone)]
pub(crate) enum ShapeType {
    Poly,
    PolyQuad,
    PolyCubic,
    Rect,
    Ellipse,
    Line,
}

#[derive(Debug, Clone)]
pub struct Shape {
    pub points: Box<Vec<Point>>,
    pub fill_texture: Box<Option<Texture>>,
    pub stroke: Stroke,
    pub stroke_texture: Box<Option<Texture>>,
    shape: ShapeType,
}

impl<'a> Shape {
    pub(crate) fn new(
        points: Box<Vec<Point>>,
        fill_texture: Box<Option<Texture>>,
        stroke: Stroke,
        stroke_texture: Box<Option<Texture>>,
        shape: ShapeType,
    ) -> Self {
        Self {
            points,
            fill_texture,
            stroke,
            stroke_texture,
            shape,
        }
    }

    pub fn draw<T: Sketch>(&self, canvas: &mut T) {
        match self.shape {
            ShapeType::Poly => self.draw_poly(canvas),
            ShapeType::PolyQuad => self.draw_quad(canvas),
            ShapeType::PolyCubic => self.draw_cubic(canvas),
            ShapeType::Rect => self.draw_rect(canvas),
            ShapeType::Ellipse => self.draw_ellipse(canvas),
            ShapeType::Line => self.draw_line(canvas),
        }
    }

    fn draw_poly<T: Sketch>(&self, canvas: &mut T) {
        let mut pb = PathBuilder::new();
        let head = self.points[0];
        let tail = &self.points[1..];
        pb.move_to(head.x, head.y);
        for p in tail {
            pb.line_to(p.x, p.y);
        }
        if self.fill_texture.is_some() {
            pb.close();
        }
        let path = pb.finish();
        if let Some(fp) = *self.fill_texture.clone() {
            canvas.fill_path(&path, &fp);
        }
        if let Some(sp) = *self.stroke_texture.clone() {
            canvas.stroke_path(&path, &sp, &self.stroke)
        }
    }

    fn draw_quad<T: Sketch>(&self, canvas: &mut T) {
        let mut pb = PathBuilder::new();
        let head = self.points[0];
        let tail = &mut self.points[1..].to_vec();
        pb.move_to(head.x, head.y);
        while tail.len() >= 2 {
            let control = tail.pop().unwrap();
            let p = tail.pop().unwrap();
            pb.quad_to(control.x, control.y, p.x, p.y);
        }
        if self.fill_texture.is_some() {
            pb.close();
        }
        let path = pb.finish();
        if let Some(fp) = *self.fill_texture.clone() {
            canvas.fill_path(&path, &fp);
        }
        if let Some(sp) = *self.stroke_texture.clone() {
            canvas.stroke_path(&path, &sp, &self.stroke);
        }
    }

    pub fn draw_cubic<T: Sketch>(&self, canvas: &mut T) {
        let mut pb = PathBuilder::new();
        let head = self.points[0];
        let tail = &mut self.points[1..].to_vec();
        pb.move_to(head.x, head.y);
        while tail.len() >= 3 {
            let control1 = tail.pop().unwrap();
            let control2 = tail.pop().unwrap();
            let p = tail.pop().unwrap();
            pb.cubic_to(control1.x, control1.y, control2.x, control2.y, p.x, p.y);
        }
        if self.fill_texture.is_some() {
            pb.close();
        }
        let path = pb.finish();
        if let Some(fp) = *self.fill_texture.clone() {
            canvas.fill_path(&path, &fp);
        }
        if let Some(sp) = *self.stroke_texture.clone() {
            canvas.stroke_path(&path, &sp, &self.stroke);
        }
    }

    fn draw_rect<T: Sketch>(&self, canvas: &mut T) {
        if self.points.len() < 2 {
            panic!("Rectangle's points vector contains less than 2 points");
        }
        let left = self.points[0].x;
        let top = self.points[0].y;
        let right = self.points[1].x;
        let bottom = self.points[1].y;
        let path = Path::rect(left, top, right - left, bottom - top);
        if let Some(fp) = *self.fill_texture.clone(){
            canvas.fill_path(&path, &fp);
        }
        if let Some(sp) = *self.stroke_texture.clone() {
            canvas.stroke_path(&path, &sp, &self.stroke);
        }
    }

    fn draw_ellipse<T: Sketch>(&self, canvas: &mut T) {
        if self.points.len() < 2 {
            panic!("Ellipse points vector contains less than 2 points");
        }
        let cx = self.points[0].x;
        let cy = self.points[0].y;
        let w = self.points[1].x;
        let _h = self.points[1].y;
        let pb = Path::circle(cx, cy, w);
        if let Some(fp) = *self.fill_texture.clone() {
            canvas.fill_path(&pb, &fp);
        }
        if let Some(sp) = *self.stroke_texture.clone() {
            canvas.stroke_path(&pb, &sp, &self.stroke);
        }
    }

    fn draw_line<T: Sketch>(&self, canvas: &mut T) {
        if self.points.len() < 2 {
            panic!("Line points vector contains less than 2 points");
        }
        let x0 = self.points[0].x;
        let y0 = self.points[0].y;
        let x1 = self.points[1].x;
        let y1 = self.points[1].y;
        let mut pb = PathBuilder::new();
        pb.move_to(x0, y0);
        pb.line_to(x1, y1);
        let path = pb.finish();
        if let Some(sp) = *self.stroke_texture.clone() {
            canvas.stroke_path(&path, &sp, &self.stroke);
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShapeBuilder {
    fill_texture: Option<Texture>,
    stroke_texture: Option<Texture>,
    stroke_width: f32,
    line_cap: LineCap,
    line_join: LineJoin,
    // stroke_dash: Option<StrokeDash>,
    points: Box<Vec<Point>>,
    shape: ShapeType,
}

impl<'a> ShapeBuilder {
    pub fn new() -> Self {
        Self {
            fill_texture: Some(Texture::white()),
            stroke_texture: Some(Texture::black()),
            stroke_width: 1.0,
            line_cap: LineCap::default(),
            line_join: LineJoin::default(),
            // stroke_dash: None,
            points: Box::new(vec![]),
            shape: ShapeType::Poly,
        }
    }

    pub fn fill_color(mut self, color: RGBA) -> Self {
        self.fill_texture = Some(Texture::SolidColor(color));
        self
    }

    pub fn fill_texture(mut self, texture: Texture) -> Self {
        self.fill_texture = Some(texture);
        self
    }

    pub fn no_fill(mut self) -> Self {
        self.fill_texture = None;
        self
    }

    pub fn no_stroke(mut self) -> Self {
        self.stroke_texture = None;
        self
    }

    pub fn stroke_color(mut self, color: RGBA) -> Self {
        self.stroke_texture = Some(Texture::SolidColor(color));
        self
    }

    pub fn stroke_weight(mut self, weight: f32) -> Self {
        self.stroke_width = weight;
        self
    }

    pub fn line_cap(mut self, cap: LineCap) -> Self {
        self.line_cap = cap;
        self
    }

    pub fn line_join(mut self, join: LineJoin) -> Self {
        self.line_join = join;
        self
    }

    // XXX FIXME
    // pub fn stroke_dash(mut self, dash: StrokeDash) -> Self {
    //     self.stroke_dash = Some(dash);
    //     self
    // }

    pub fn points(mut self, pts: &[Point]) -> Self {
        self.points = Box::new(pts.to_vec());
        self
    }

    pub fn rect_ltrb(mut self, lt: Point, rb: Point) -> Self {
        self.shape = ShapeType::Rect;
        self.points = Box::new(vec![lt, rb]);
        self
    }

    pub fn rect_xywh(mut self, xy: Point, wh: Point) -> Self {
        self.shape = ShapeType::Rect;
        self.points = Box::new(vec![xy, Point::new(xy.x + wh.x, xy.y + wh.y)]);
        self
    }

    pub fn ellipse(mut self, center: Point, wh: Point) -> Self {
        self.shape = ShapeType::Ellipse;
        self.points = Box::new(vec![center, wh]);
        self
    }

    pub fn circle(mut self, center: Point, radius: f32) -> Self {
        self.shape = ShapeType::Ellipse;
        self.points = Box::new(vec![center, Point::new(radius, radius)]);
        self
    }

    pub fn quad(mut self) -> Self {
        self.shape = ShapeType::PolyQuad;
        self
    }

    pub fn cubic(mut self) -> Self {
        self.shape = ShapeType::PolyCubic;
        self
    }

    pub fn line(mut self, from: Point, to: Point) -> Self {
        self.points = Box::new(vec![from, to]);
        self.shape = ShapeType::Line;
        self
    }

    pub fn build(self) -> Shape {
        let mut fill_texture: Box<Option<Texture>> = Box::new(None);
        let mut stroke_texture: Box<Option<Texture>> = Box::new(None);
        if let Some(fs) = self.fill_texture {
            fill_texture = Box::new(Some(fs));
        };
        if let Some(ss) = self.stroke_texture {
            stroke_texture = Box::new(Some(ss));
        };
        let mut stroke = Stroke::default();
        stroke.width = self.stroke_width;
        stroke.line_cap = self.line_cap;
        stroke.line_join = self.line_join;
        stroke.width = self.stroke_width;
        Shape::new(
            self.points,
            fill_texture,
            stroke,
            stroke_texture,
            self.shape,
        )
    }
}

pub fn stroke(weight: f32) -> Stroke {
    let mut stroke = Stroke::default();
    stroke.width = weight;
    stroke
}

pub fn line<T: Sketch>(
    canvas: &mut T,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    stroke: &Stroke,
    stroke_paint: Texture,
) {
    let mut pb = PathBuilder::new();
    pb.move_to(x0, y0);
    pb.line_to(x1, y1);
    let path = pb.finish();
    canvas.stroke_path(&path, &stroke_paint, &stroke);
}
