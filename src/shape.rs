use crate::base::*;
use crate::prelude::{Point, vec2};

#[derive(Debug, Clone)]
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
    pub points: Box<Vec<TaggedPoint>>,
    pub fill_texture: Box<Option<TextureKind>>,
    pub stroke: Stroke,
    pub stroke_texture: Box<Option<TextureKind>>,
    shape: ShapeType,
    fillrule: FillRule,
    transform: Transform,
}

impl<'a> Shape {
    pub(crate) fn new(
        points: Box<Vec<TaggedPoint>>,
        fill_texture: Box<Option<TextureKind>>,
        stroke: Stroke,
        stroke_texture: Box<Option<TextureKind>>,
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
            canvas.fill_path(&path, &Texture::new(fp));
        }
        if let Some(sp) = *self.stroke_texture.clone() {
            canvas.stroke_path(&path, &Texture::new(sp), &self.stroke)
        }
    }

    fn draw_quad<T: Sketch>(&self, canvas: &mut T) {
        let mut pb = PathBuilder::new();
        pb.set_fillrule(self.fillrule);
        let head = self.points[0].point;
        let tail = &mut self.points[1..].to_vec();
        pb.move_to(head.x, head.y);
        while tail.len() >= 2 {
            let control = tail.pop().unwrap().point;
            let p = tail.pop().unwrap().point;
            pb.quad_to(control.x, control.y, p.x, p.y);
        }
        if self.fill_texture.is_some() {
            pb.close();
        }
        pb.set_transform(self.transform);
        let path = pb.finish();
        if let Some(fp) = *self.fill_texture.clone() {
            canvas.fill_path(&path, &Texture::new(fp));
        }
        if let Some(sp) = *self.stroke_texture.clone() {
            canvas.stroke_path(&path, &Texture::new(sp), &self.stroke);
        }
    }

    pub fn draw_cubic<T: Sketch>(&self, canvas: &mut T) {
        let mut pb = PathBuilder::new();
        pb.set_fillrule(self.fillrule);
        let head = self.points[0].point;
        let tail = &mut self.points[1..].to_vec();
        pb.move_to(head.x, head.y);
        while tail.len() >= 3 {
            let control1 = tail.pop().unwrap().point;
            let control2 = tail.pop().unwrap().point;
            let p = tail.pop().unwrap().point;
            pb.cubic_to(control1.x, control1.y, control2.x, control2.y, p.x, p.y);
        }
        if self.fill_texture.is_some() {
            pb.close();
        }
        pb.set_transform(self.transform);
        let path = pb.finish();
        if let Some(fp) = *self.fill_texture.clone() {
            canvas.fill_path(&path, &Texture::new(fp));
        }
        if let Some(sp) = *self.stroke_texture.clone() {
            canvas.stroke_path(&path, &Texture::new(sp), &self.stroke);
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
            canvas.fill_path(&path, &Texture::new(fp));
        }
        if let Some(sp) = *self.stroke_texture.clone() {
            canvas.stroke_path(&path, &Texture::new(sp), &self.stroke);
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
            canvas.fill_path(&pb, &Texture::new(fp));
        }
        if let Some(sp) = *self.stroke_texture.clone() {
            canvas.stroke_path(&pb, &Texture::new(sp), &self.stroke);
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
            canvas.stroke_path(&path, &Texture::new(sp), &self.stroke);
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShapeBuilder {
    fill_texture: Option<TextureKind>,
    stroke_texture: Option<TextureKind>,
    stroke_width: f32,
    line_cap: LineCap,
    line_join: LineJoin,
    stroke_dash: Option<Dash>,
    points: Box<Vec<TaggedPoint>>,
    shape: ShapeType,
    fillrule: FillRule,
    transform: Transform,
}

impl<'a> ShapeBuilder {
    pub fn new() -> Self {
        Self {
            fill_texture: Some(TextureKind::white()),
            stroke_texture: Some(TextureKind::black()),
            stroke_width: 1.0,
            line_cap: LineCap::default(),
            line_join: LineJoin::default(),
            stroke_dash: None,
            points: Box::new(vec![]),
            shape: ShapeType::Poly,
            fillrule: FillRule::Winding,
            transform: Transform::identity(),
        }
    }

    pub fn fill_color(mut self, color: RGBA) -> Self {
        self.fill_texture = Some(TextureKind::SolidColor(color));
        self
    }

    pub fn fill_texture(mut self, texture: TextureKind) -> Self {
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
        self.stroke_texture = Some(TextureKind::SolidColor(color));
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
        self.points = Box::new(tagged);
        self
    }

    pub fn tagged_points(mut self, tps: &[TaggedPoint]) -> Self {
        self.points = Box::new(tps.to_vec());
        self
    }

    pub fn rect_ltrb(mut self, lt: Point, rb: Point) -> Self {
        self.shape = ShapeType::Rect;
        self.points = Box::new(vec![TaggedPoint::new(lt), TaggedPoint::new(rb)]);
        self
    }

    pub fn rect_xywh(mut self, xy: Point, wh: Point) -> Self {
        self.shape = ShapeType::Rect;
        self.points = Box::new(vec![
            TaggedPoint::new(xy),
            TaggedPoint::new(Point::new(xy.x + wh.x, xy.y + wh.y)),
        ]);
        self
    }

    pub fn ellipse(mut self, center: Point, wh: Point) -> Self {
        self.shape = ShapeType::Ellipse;
        self.points = Box::new(vec![TaggedPoint::new(center), TaggedPoint::new(wh)]);
        self
    }

    pub fn circle(mut self, center: Point, radius: f32) -> Self {
        self.shape = ShapeType::Ellipse;
        self.points = Box::new(vec![
            TaggedPoint::new(center),
            TaggedPoint::new(Point::new(radius, radius)),
        ]);
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
        self.points = Box::new(vec![TaggedPoint::new(from), TaggedPoint::new(to)]);
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
    pub fn cartesian(mut self, width: f32, height: f32) -> Self {
        self.transform = self.transform
            .post_scale(1.0, -1.0)
            .post_translate(vec2(width / 2.0, height / 2.0));
        self
    }

    pub fn build(self) -> Shape {
        let mut fill_texture: Box<Option<TextureKind>> = Box::new(None);
        let mut stroke_texture: Box<Option<TextureKind>> = Box::new(None);
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
            self.fillrule,
            self.transform,
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
    stroke_texture: &Texture,
) {
    let mut pb = PathBuilder::new();
    pb.move_to(x0, y0);
    pb.line_to(x1, y1);
    let path = pb.finish();
    canvas.stroke_path(&path, &stroke_texture, &stroke);
}
