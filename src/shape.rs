use crate::base::*;
use crate::prelude::{vec2, Point, BLACK};
use num_traits::AsPrimitive;

#[derive(Debug, Clone, Copy)]
pub(crate) enum ShapeType {
    Poly,
    PolyQuad,
    PolyCubic,
    Rect,
    Ellipse,
    Line,
}

#[derive(Debug, Clone, Copy)]
pub struct TaggedPoint {
    pub point: Point,
    pub pen: bool,
}

impl TaggedPoint {
    pub fn new(point: Point) -> Self {
        Self { point, pen: true }
    }

    pub fn with_pen(point: Point, pen: bool) -> Self {
        Self { point, pen }
    }
}

pub fn tagged(ps: Vec<Point>) -> Vec<TaggedPoint> {
    ps.iter().map(|p| TaggedPoint::new(*p)).collect()
}

#[derive(Debug, Clone)]
pub struct Shape {
    pub points: Vec<TaggedPoint>,
    pub fill_texture: Box<Option<Texture>>,
    pub stroke: Stroke,
    pub stroke_texture: Box<Option<Texture>>,
    shape: ShapeType,
    fillrule: FillRule,
    transform: Transform,
}

impl<'a> Shape {
    pub(crate) fn new(
        points: Vec<TaggedPoint>,
        fill_texture: Box<Option<Texture>>,
        stroke: Stroke,
        stroke_texture: Box<Option<Texture>>,
        shape: ShapeType,
        fillrule: FillRule,
        transform: Transform,
    ) -> Self {
        Self {
            points,
            fill_texture,
            stroke,
            stroke_texture,
            shape,
            fillrule,
            transform,
        }
    }

    pub fn draw<T: Sketch>(&self, canvas: &mut T) {
        let shape = self.shape;
        match shape {
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
        pb.set_fillrule(self.fillrule);
        let head = self.points[0].point;
        let tail = &self.points[1..];
        pb.move_to(head.x, head.y);
        for p in tail {
            if p.pen {
                pb.line_to(p.point.x, p.point.y);
            } else {
                pb.move_to(p.point.x, p.point.y);
            }
        }
        if self.fill_texture.is_some() {
            pb.close();
        }
        pb.set_transform(self.transform);
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
        pb.set_fillrule(self.fillrule);
        let head = self.points[0].point;
        pb.move_to(head.x, head.y);
        let tail = self.points[1..].chunks_exact(2);
        for t in tail {
            let control = t[0].point;
            let p = t[1].point;
            pb.quad_to(control.x, control.y, p.x, p.y);
        }
        if self.fill_texture.is_some() {
            pb.close();
        }
        pb.set_transform(self.transform);
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
        pb.set_fillrule(self.fillrule);
        let head = self.points[0].point;
        pb.move_to(head.x, head.y);
        let tail = self.points[1..].chunks_exact(3);
        for t in tail {
            let control1 = t[0].point;
            let control2 = t[1].point;
            let p = t[2].point;
            pb.cubic_to(control1.x, control1.y, control2.x, control2.y, p.x, p.y);
        }
        if self.fill_texture.is_some() {
            pb.close();
        }
        pb.set_transform(self.transform);
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
        let left = self.points[0].point.x;
        let top = self.points[0].point.y;
        let right = self.points[1].point.x;
        let bottom = self.points[1].point.y;
        let mut path = Path::rect(left, top, right - left, bottom - top);
        path.transform = self.transform;
        if let Some(fp) = *self.fill_texture.clone() {
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
        let cx = self.points[0].point.x;
        let cy = self.points[0].point.y;
        let w = self.points[1].point.x;
        let _h = self.points[1].point.y;
        let mut pb = Path::circle(cx, cy, w);
        pb.transform = self.transform;
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
        let x0 = self.points[0].point.x;
        let y0 = self.points[0].point.y;
        let x1 = self.points[1].point.x;
        let y1 = self.points[1].point.y;
        let mut pb = PathBuilder::new();
        pb.move_to(x0, y0);
        pb.line_to(x1, y1);
        pb.set_transform(self.transform);
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
    stroke_dash: Option<Dash>,
    points: Vec<TaggedPoint>,
    shape: ShapeType,
    fillrule: FillRule,
    transform: Transform,
}

impl Default for ShapeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ShapeBuilder {
    pub fn new() -> Self {
        Self {
            fill_texture: Some(Texture::solid_color(BLACK)),
            stroke_texture: Some(Texture::solid_color(BLACK)),
            stroke_width: 1.0,
            line_cap: Default::default(),
            line_join: Default::default(),
            stroke_dash: None,
            points: vec![],
            shape: ShapeType::Poly,
            fillrule: FillRule::Winding,
            transform: Transform::identity(),
        }
    }

    pub fn fill_color(mut self, color: RGBA) -> Self {
        self.fill_texture = Some(Texture::solid_color(color));
        self
    }

    pub fn fill_texture(mut self, texture: &Texture) -> Self {
        self.fill_texture = Some(texture.clone());
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
        self.stroke_texture = Some(Texture::solid_color(color));
        self
    }

    pub fn stroke_texture(mut self, texture: &Texture) -> Self {
        self.stroke_texture = Some(texture.clone());
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

    pub fn stroke_dash(mut self, dash: Dash) -> Self {
        self.stroke_dash = Some(dash);
        self
    }

    pub fn points(mut self, pts: &[Point]) -> Self {
        let points = pts.to_vec();
        let tagged = points.iter().map(|p| TaggedPoint::new(*p)).collect();
        self.points = tagged;
        self
    }

    pub fn tagged_points(mut self, tps: &[TaggedPoint]) -> Self {
        self.points = tps.to_vec();
        self
    }

    pub fn rect_ltrb(mut self, lt: Point, rb: Point) -> Self {
        self.shape = ShapeType::Rect;
        self.points = vec![TaggedPoint::new(lt), TaggedPoint::new(rb)];
        self
    }

    pub fn rect_xywh(mut self, xy: Point, wh: Point) -> Self {
        self.shape = ShapeType::Rect;
        self.points = vec![
            TaggedPoint::new(xy),
            TaggedPoint::new(Point::new(xy.x + wh.x, xy.y + wh.y)),
        ];
        self
    }

    pub fn ellipse(mut self, center: Point, wh: Point) -> Self {
        self.shape = ShapeType::Ellipse;
        self.points = vec![TaggedPoint::new(center), TaggedPoint::new(wh)];
        self
    }

    pub fn circle(mut self, center: Point, radius: f32) -> Self {
        self.shape = ShapeType::Ellipse;
        self.points = vec![
            TaggedPoint::new(center),
            TaggedPoint::new(Point::new(radius, radius)),
        ];
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
        self.points = vec![TaggedPoint::new(from), TaggedPoint::new(to)];
        self.shape = ShapeType::Line;
        self
    }

    pub fn fill_rule(mut self, fillrule: FillRule) -> Self {
        self.fillrule = fillrule;
        self
    }

    pub fn transform(mut self, transform: &Transform) -> Self {
        let t = self.transform.post_transform(transform);
        self.transform = t;
        self
    }

    /// Interpret points as cartiesian coordinates with center at (0, 0).
    pub fn cartesian<T: AsPrimitive<f32>>(mut self, width: T, height: T) -> Self {
        self.transform = self
            .transform
            .post_scale(1.0, -1.0)
            .post_translate(vec2(width.as_() / 2.0, height.as_() / 2.0));
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
        let stroke = Stroke {
            width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            ..Default::default()
        };
        Shape::new(
            self.points,
            fill_texture,
            stroke,
            stroke_texture,
            self.shape,
            self.fillrule,
            self.transform,
        )
    }
}

pub fn stroke(weight: f32) -> Stroke {
    Stroke {
        width: weight,
        ..Default::default()
    }
}

pub fn line<T: Sketch>(
    canvas: &mut T,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    stroke: &Stroke,
    stroke_texture: &Texture,
) {
    let mut pb = PathBuilder::new();
    pb.move_to(x0, y0);
    pb.line_to(x1, y1);
    let path = pb.finish();
    canvas.stroke_path(&path, stroke_texture, stroke);
}
