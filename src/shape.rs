use crate::{
    canvas::Canvas,
    kolor::ConvertColor,
    prelude::{chaiken, pt, Trail, TAU},
};
use num_traits::AsPrimitive;
use rand::RngCore;
use rand_distr::{Distribution, Normal};
use tiny_skia::*;

#[derive(Debug, Clone, Copy)]
pub(crate) enum ShapeType {
    Poly,
    PolyQuad,
    PolyCubic,
    Rect,
    Circle,
    Line,
    Ellipse,
}

#[derive(Debug, Clone)]
pub struct Shape<'a> {
    points: Vec<Point>,
    fill_paint: Box<Option<Paint<'a>>>,
    stroke: Stroke,
    stroke_paint: Box<Option<Paint<'a>>>,
    shape: ShapeType,
    fillrule: FillRule,
    transform: Transform,
}

impl<'a> Shape<'a> {
    pub(crate) fn new(
        points: Vec<Point>,
        fill_paint: Box<Option<Paint<'a>>>,
        stroke: Stroke,
        stroke_paint: Box<Option<Paint<'a>>>,
        shape: ShapeType,
        fillrule: FillRule,
        transform: Transform,
    ) -> Self {
        Self {
            points,
            fill_paint,
            stroke,
            stroke_paint,
            shape,
            fillrule,
            transform,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        let shape = self.shape;
        match shape {
            ShapeType::Poly => self.draw_poly(canvas),
            ShapeType::PolyQuad => self.draw_quad(canvas),
            ShapeType::PolyCubic => self.draw_cubic(canvas),
            ShapeType::Rect => self.draw_rect(canvas),
            ShapeType::Circle => self.draw_circle(canvas),
            ShapeType::Line => self.draw_line(canvas),
            ShapeType::Ellipse => self.draw_ellipse(canvas),
        }
    }

    fn draw_poly(&self, canvas: &mut Canvas) {
        let mut pb = PathBuilder::new();
        let head = self.points[0];
        let tail = &self.points[1..];
        pb.move_to(head.x, head.y);
        for p in tail {
            pb.line_to(p.x, p.y);
        }
        if self.fill_paint.is_some() {
            pb.close();
        }
        let path = pb.finish().unwrap();
        if let Some(mut fp) = *self.fill_paint.clone() {
            canvas.fill_path(&path, &mut fp, self.fillrule, self.transform, None);
        }
        if let Some(mut sp) = *self.stroke_paint.clone() {
            canvas.stroke_path(&path, &mut sp, &self.stroke, self.transform, None);
        }
    }

    fn draw_quad(&self, canvas: &mut Canvas) {
        let mut pb = PathBuilder::new();
        let head = self.points[0];
        pb.move_to(head.x, head.y);
        let tail = self.points[1..].chunks_exact(2);
        for t in tail {
            let control = t[0];
            let p = t[1];
            pb.quad_to(control.x, control.y, p.x, p.y);
        }
        if self.fill_paint.is_some() {
            pb.close();
        }
        let path = pb.finish().unwrap();
        if let Some(mut fp) = *self.fill_paint.clone() {
            canvas.fill_path(&path, &mut fp, self.fillrule, self.transform, None);
        }
        if let Some(mut sp) = *self.stroke_paint.clone() {
            canvas.stroke_path(&path, &mut sp, &self.stroke, self.transform, None);
        }
    }

    pub fn draw_cubic(&self, canvas: &mut Canvas) {
        let mut pb = PathBuilder::new();
        let head = self.points[0];
        pb.move_to(head.x, head.y);
        let tail = self.points[1..].chunks_exact(3);
        for t in tail {
            let control1 = t[0];
            let control2 = t[1];
            let p = t[2];
            pb.cubic_to(control1.x, control1.y, control2.x, control2.y, p.x, p.y);
        }
        if self.fill_paint.is_some() {
            pb.close();
        }
        let path = pb.finish().unwrap();
        if let Some(mut fp) = *self.fill_paint.clone() {
            canvas.fill_path(&path, &mut fp, self.fillrule, self.transform, None);
        }
        if let Some(mut sp) = *self.stroke_paint.clone() {
            canvas.stroke_path(&path, &mut sp, &self.stroke, self.transform, None);
        }
    }

    fn draw_rect(&self, canvas: &mut Canvas) {
        if self.points.len() < 2 {
            panic!("Rectangle's points vector contains less than 2 points");
        }
        let left = self.points[0].x;
        let top = self.points[0].y;
        let right = self.points[1].x;
        let bottom = self.points[1].y;
        let rect = Rect::from_ltrb(left, top, right, bottom).unwrap();
        let path = PathBuilder::from_rect(rect);
        if let Some(mut fp) = *self.fill_paint.clone() {
            canvas.fill_path(&path, &mut fp, self.fillrule, self.transform, None);
        }
        if let Some(mut sp) = *self.stroke_paint.clone() {
            canvas.stroke_path(&path, &mut sp, &self.stroke, self.transform, None);
        }
    }

    fn draw_circle(&self, canvas: &mut Canvas) {
        if self.points.len() < 2 {
            panic!("Circle points vector contains less than 2 points");
        }
        let cx = self.points[0].x;
        let cy = self.points[0].y;
        let w = self.points[1].x;
        let _h = self.points[1].y;
        let circle = PathBuilder::from_circle(cx, cy, w).unwrap();
        if let Some(mut fp) = *self.fill_paint.clone() {
            canvas.fill_path(&circle, &mut fp, self.fillrule, self.transform, None);
        }
        if let Some(mut sp) = *self.stroke_paint.clone() {
            canvas.stroke_path(&circle, &mut sp, &self.stroke, self.transform, None);
        }
    }

    fn draw_ellipse(&self, canvas: &mut Canvas) {
        if self.points.len() < 2 {
            panic!("Ellipse points vector contains less than 2 points");
        }
        let cx = self.points[0].x;
        let cy = self.points[0].y;
        let w = self.points[1].x;
        let h = self.points[1].y;
        let rect = Rect::from_xywh(cx - w / 2.0, cy - h / 2.0, w, h).unwrap();
        let ellipse = PathBuilder::from_oval(rect).unwrap();
        if let Some(mut fp) = *self.fill_paint.clone() {
            canvas.fill_path(&ellipse, &mut fp, self.fillrule, self.transform, None);
        }
        if let Some(mut sp) = *self.stroke_paint.clone() {
            canvas.stroke_path(&ellipse, &mut sp, &self.stroke, self.transform, None);
        }
    }

    fn draw_line(&self, canvas: &mut Canvas) {
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
        let path = pb.finish().unwrap();
        if let Some(mut sp) = *self.stroke_paint.clone() {
            canvas.stroke_path(&path, &mut sp, &self.stroke, self.transform, None);
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShapeBuilder<'a> {
    fill_paint: Option<Paint<'a>>,
    stroke_paint: Option<Paint<'a>>,
    stroke_width: f32,
    miter_limit: f32,
    line_cap: LineCap,
    line_join: LineJoin,
    stroke_dash: Option<StrokeDash>,
    points: Vec<Point>,
    shape: ShapeType,
    fillrule: FillRule,
    transform: Transform,
}

impl<'a> Default for ShapeBuilder<'a> {
    fn default() -> Self {
        Self {
            fill_paint: Some(Paint::default()),
            stroke_paint: Some(Paint::default()),
            stroke_width: 1.0,
            miter_limit: Default::default(),
            line_cap: Default::default(),
            line_join: Default::default(),
            stroke_dash: None,
            points: vec![],
            shape: ShapeType::Poly,
            fillrule: FillRule::Winding,
            transform: Transform::identity(),
        }
    }
}

impl<'a> ShapeBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fill_color(mut self, color: impl ConvertColor) -> Self {
        let mut paint = Paint {
            anti_alias: true,
            ..Default::default()
        };
        paint.set_color(color.to_color());
        self.fill_paint = Some(paint);
        self
    }

    pub fn fill_paint(mut self, texture: &'a Paint) -> Self {
        self.fill_paint = Some(texture.clone());
        self
    }

    pub fn no_fill(mut self) -> Self {
        self.fill_paint = None;
        self
    }

    pub fn no_stroke(mut self) -> Self {
        self.stroke_paint = None;
        self
    }

    pub fn stroke_color(mut self, color: impl ConvertColor) -> Self {
        let mut paint = Paint {
            anti_alias: true,
            ..Default::default()
        };
        paint.set_color(color.to_color());
        self.stroke_paint = Some(paint);
        self
    }

    pub fn stroke_paint(mut self, pain: &'a Paint) -> Self {
        self.stroke_paint = Some(pain.clone());
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

    pub fn stroke_dash(mut self, dash: StrokeDash) -> Self {
        self.stroke_dash = Some(dash);
        self
    }

    pub fn points(mut self, pts: &[Point]) -> Self {
        let points = pts.to_vec();
        self.points = points;
        self
    }

    pub fn rect_ltrb(mut self, lt: Point, rb: Point) -> Self {
        self.shape = ShapeType::Rect;
        self.points = vec![lt, rb];
        self
    }

    pub fn rect_xywh(mut self, xy: Point, wh: Point) -> Self {
        self.shape = ShapeType::Rect;
        self.points = vec![xy, pt(xy.x + wh.x, xy.y + wh.y)];
        self
    }

    pub fn rect_cwh(mut self, c: Point, wh: Point) -> Self {
        self.shape = ShapeType::Rect;
        let w2 = wh.x / 2.0;
        let h2 = wh.y / 2.0;
        let p = pt(c.x - w2, c.y - h2);
        self.rect_xywh(p, wh)
    }

    pub fn circle(mut self, center: Point, radius: f32) -> Self {
        self.shape = ShapeType::Circle;
        self.points = vec![center, pt(radius, radius)];
        self
    }

    pub fn ellipse(mut self, center: Point, width: f32, height: f32) -> Self {
        self.shape = ShapeType::Ellipse;
        self.points = vec![center, pt(width, height)];
        self
    }

    pub fn polygon(mut self, center: Point, radius: f32, sides: u32) -> Self {
        self.shape = ShapeType::Poly;
        let mut theta = 0.0;
        let delta = TAU / sides as f32;
        let mut pts = vec![];
        while theta < TAU {
            pts.push(pt(
                center.x + radius * theta.cos(),
                center.y + radius * theta.sin(),
            ));
            theta += delta;
        }
        self.points = pts;
        self
    }

    pub fn star(mut self, center: Point, radius1: f32, radius2: f32, sides: u32) -> Self {
        self.shape = ShapeType::Poly;
        let mut theta = 0.0;
        let delta = TAU / sides as f32;
        let half_delta = delta / 2.0;
        let mut pts = vec![];
        while theta < TAU {
            pts.push(pt(
                center.x + radius2 * theta.cos(),
                center.y + radius2 * theta.sin(),
            ));
            pts.push(pt(
                center.x + radius1 * (theta + half_delta).cos(),
                center.y + radius1 * (theta + half_delta).sin(),
            ));
            theta += delta;
        }
        self.points = pts;
        self
    }

    pub fn pearl<R: RngCore>(
        mut self,
        center: Point,
        a: f32,
        b: f32,
        sides: u32,
        iterations: u32,
        rng: &mut R,
    ) -> Self {
        self.shape = ShapeType::Poly;
        let mut points = vec![];
        for i in 0..sides {
            let normal = Normal::new(0.0, 0.25 * a.min(b)).unwrap();
            let dx = normal.sample(rng);
            let dy = normal.sample(rng);
            let u = TAU * i as f32 / sides as f32;
            let x1 = a * u.cos() + center.x + dx;
            let y1 = b * u.sin() + center.y + dy;
            points.push(pt(x1, y1));
        }
        self.points = chaiken(&points, iterations, Trail::Closed)
            .into_iter()
            .collect();
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
        self.points = vec![from, to];
        self.shape = ShapeType::Line;
        self
    }

    pub fn fill_rule(mut self, fillrule: FillRule) -> Self {
        self.fillrule = fillrule;
        self
    }

    pub fn transform(mut self, transform: &Transform) -> Self {
        let t = self.transform.post_concat(*transform);
        self.transform = t;
        self
    }

    /// Interpret points as cartiesian coordinates with center at (0, 0).
    pub fn cartesian<T: AsPrimitive<f32>>(mut self, width: T, height: T) -> Self {
        self.transform = self
            .transform
            .post_scale(1.0, -1.0)
            .post_translate(width.as_() / 2.0, height.as_() / 2.0);
        self
    }

    pub fn build(self) -> Shape<'a> {
        let mut fill_paint: Box<Option<Paint<'a>>> = Box::new(None);
        let mut stroke_paint: Box<Option<Paint<'a>>> = Box::new(None);
        if let Some(fs) = self.fill_paint {
            fill_paint = Box::new(Some(fs));
        };
        if let Some(ss) = self.stroke_paint {
            stroke_paint = Box::new(Some(ss));
        };
        let stroke = Stroke {
            width: self.stroke_width,
            miter_limit: self.miter_limit,
            line_cap: self.line_cap,
            line_join: self.line_join,
            dash: self.stroke_dash,
        };
        Shape::new(
            self.points,
            fill_paint,
            stroke,
            stroke_paint,
            self.shape,
            self.fillrule,
            self.transform,
        )
    }
}
