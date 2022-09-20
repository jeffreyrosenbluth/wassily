use crate::{
    dsl::*,
    kolor::ConvertColor,
    prelude::{chaiken, pt, TAU},
    util,
};
use num_traits::AsPrimitive;
use rand::rngs::SmallRng;
use rand_distr::{Distribution, Normal};
use tiny_skia::*;

#[derive(Debug, Clone, Copy)]
pub(crate) enum ShapeType {
    Poly,
    PolyQuad,
    PolyCubic,
    Circle,
    Line,
    Ellipse,
}

#[derive(Debug, Clone)]
pub struct Shape {
    pub points: Vec<Point>,
    pub fill_paint: Option<Ink>,
    pub stroke: Stroke,
    pub stroke_paint: Option<Ink>,
    shape: ShapeType,
    pub fillrule: FillRule,
    pub transform: Transform,
}

impl Shape {
    pub(crate) fn new(
        points: Vec<Point>,
        fill_paint: Option<Ink>,
        stroke: Stroke,
        stroke_paint: Option<Ink>,
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

    pub fn draw(self) -> DrawProgram {
        let shape = self.shape;
        match shape {
            ShapeType::Poly => self.draw_poly(),
            ShapeType::PolyQuad => self.draw_quad(),
            ShapeType::PolyCubic => self.draw_cubic(),
            ShapeType::Circle => self.draw_circle(),
            ShapeType::Line => self.draw_line(),
            ShapeType::Ellipse => self.draw_ellipse(),
        }
    }

    pub fn push(self, drawing: &mut Drawing) {
        let cmds = self.draw();
        drawing.add_cmds(cmds);
    }

    fn draw_poly(self) -> DrawProgram {
        let mut cmds = Vec::new();
        let mut pb = Trail::new();
        let head = self.points[0];
        let tail = &self.points[1..];
        pb.move_to(head.x, head.y);
        for p in tail {
            pb.line_to(p.x, p.y);
        }
        if self.fill_paint.is_some() {
            pb.close();
        }
        if let Some(fp) = self.fill_paint {
            cmds.push(DrawCmd::Fill {
                trail: pb.clone(),
                ink: fp,
                fill_rule: self.fillrule,
                transform: self.transform,
            });
        }
        if let Some(sp) = self.stroke_paint.clone() {
            cmds.push(DrawCmd::Stroke {
                trail: pb,
                ink: sp,
                stroke: self.stroke,
                transform: self.transform,
            });
        }
        cmds
    }

    fn draw_quad(self) -> DrawProgram {
        let mut cmds = Vec::new();
        let mut pb = Trail::new();
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
        if let Some(fp) = self.fill_paint.clone() {
            cmds.push(DrawCmd::Fill {
                trail: pb.clone(),
                ink: fp,
                fill_rule: self.fillrule,
                transform: self.transform,
            });
        }
        if let Some(sp) = self.stroke_paint.clone() {
            cmds.push(DrawCmd::Stroke {
                trail: pb,
                ink: sp,
                stroke: self.stroke,
                transform: self.transform,
            });
        }
        cmds
    }

    fn draw_cubic(self) -> DrawProgram {
        let mut cmds = Vec::new();
        let mut pb = Trail::new();
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
        if let Some(fp) = self.fill_paint.clone() {
            cmds.push(DrawCmd::Fill {
                trail: pb.clone(),
                ink: fp,
                fill_rule: self.fillrule,
                transform: self.transform,
            });
        }
        if let Some(sp) = self.stroke_paint.clone() {
            cmds.push(DrawCmd::Stroke {
                trail: pb,
                ink: sp,
                stroke: self.stroke,
                transform: self.transform,
            });
        }
        cmds
    }

    fn draw_circle(self) -> DrawProgram {
        if self.points.len() < 2 {
            panic!("Circle points vector contains less than 2 points");
        }
        let mut cmds = Vec::new();
        let mut pb = Trail::new();
        let cx = self.points[0].x;
        let cy = self.points[0].y;
        let r = self.points[1].x;
        pb.circle(cx, cy, r);
        if let Some(fp) = self.fill_paint.clone() {
            cmds.push(DrawCmd::Fill {
                trail: pb.clone(),
                ink: fp,
                fill_rule: self.fillrule,
                transform: self.transform,
            });
        }
        if let Some(sp) = self.stroke_paint.clone() {
            cmds.push(DrawCmd::Stroke {
                trail: pb,
                ink: sp,
                stroke: self.stroke,
                transform: self.transform,
            });
        }
        cmds
    }

    fn draw_ellipse(self) -> DrawProgram {
        if self.points.len() < 2 {
            panic!("Ellipse points vector contains less than 2 points");
        }
        let mut cmds = Vec::new();
        let mut pb = Trail::new();
        let cx = self.points[0].x;
        let cy = self.points[0].y;
        let w = self.points[1].x;
        let h = self.points[1].y;
        pb.ellipse(cx, cy, w, h);
        if let Some(fp) = self.fill_paint.clone() {
            cmds.push(DrawCmd::Fill {
                trail: pb.clone(),
                ink: fp,
                fill_rule: self.fillrule,
                transform: self.transform,
            });
        }
        if let Some(sp) = self.stroke_paint.clone() {
            cmds.push(DrawCmd::Stroke {
                trail: pb,
                ink: sp,
                stroke: self.stroke,
                transform: self.transform,
            });
        }
        cmds
    }

    fn draw_line(self) -> DrawProgram {
        if self.points.len() < 2 {
            panic!("Line points vector contains less than 2 points");
        }
        let mut cmds = Vec::new();
        let x0 = self.points[0].x;
        let y0 = self.points[0].y;
        let x1 = self.points[1].x;
        let y1 = self.points[1].y;
        let mut pb = Trail::new();
        pb.move_to(x0, y0);
        pb.line_to(x1, y1);
        if let Some(sp) = self.stroke_paint.clone() {
            cmds.push(DrawCmd::Stroke {
                trail: pb,
                ink: sp,
                stroke: self.stroke,
                transform: self.transform,
            });
        }
        cmds
    }
}

#[derive(Debug, Clone)]
pub struct ShapeBuilder {
    fill_paint: Option<Ink>,
    stroke_paint: Option<Ink>,
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

impl Default for ShapeBuilder {
    fn default() -> Self {
        Self {
            fill_paint: Some(Ink::default()),
            stroke_paint: Some(Ink::default()),
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

impl ShapeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fill_color(mut self, color: impl ConvertColor) -> Self {
        let mut ink = Ink::default();
        ink.set_color(color.to_color());
        self.fill_paint = Some(ink);
        self
    }

    pub fn fill_paint(mut self, ink: Ink) -> Self {
        self.fill_paint = Some(ink);
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
        let mut ink = Ink::default();
        ink.set_color(color.to_color());
        self.stroke_paint = Some(ink);
        self
    }

    pub fn stroke_paint(mut self, ink: Ink) -> Self {
        self.stroke_paint = Some(ink);
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
        self.shape = ShapeType::Poly;
        self.points = vec![lt, pt(rb.x, lt.y), rb, pt(lt.x, rb.y)];
        self
    }

    pub fn rect_xywh(mut self, xy: Point, wh: Point) -> Self {
        self.shape = ShapeType::Poly;
        self.points = vec![
            xy,
            pt(xy.x + wh.x, xy.y),
            pt(xy.x + wh.x, xy.y + wh.y),
            pt(xy.x, xy.y + wh.y),
        ];
        self
    }

    pub fn rect_cwh(mut self, c: Point, wh: Point) -> Self {
        self.shape = ShapeType::Poly;
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

    pub fn pearl(
        mut self,
        center: Point,
        a: f32,
        b: f32,
        sides: u32,
        iterations: u32,
        rng: &mut SmallRng,
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
        self.points = chaiken(&points, iterations, util::Trail::Closed)
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

    pub fn build(self) -> Shape {
        let mut fill_paint = None;
        let mut stroke_paint = None;
        if let Some(fs) = self.fill_paint {
            fill_paint = Some(fs);
        };
        if let Some(ss) = self.stroke_paint {
            stroke_paint = Some(ss);
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
