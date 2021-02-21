use tiny_skia::*;
use crate::util::*;

// TODO use PixelMap.fill instead.
pub fn background(canvas: &mut Canvas, width: u32, height: u32, color: Color) {
    let mut paint = Paint::default();
    paint.set_color(color);
    let rect = IntRect::from_xywh(0, 0, width, height).unwrap().to_rect();
    canvas.fill_rect(rect, &paint);
}

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
pub struct Shape<'a> {
    pub points: Box<Vec<Point>>,
    pub fill_paint: Option<Paint<'a>>,
    pub stroke: Stroke,
    pub stroke_paint: Option<Paint<'a>>,
    shape: ShapeType,
}

impl<'a> Shape<'a> {
    pub(crate) fn new(
        points: Box<Vec<Point>>,
        fill_paint: Option<Paint<'a>>,
        stroke: Stroke,
        stroke_paint: Option<Paint<'a>>,
        shape: ShapeType,
    ) -> Self {
        Self {
            points,
            fill_paint,
            stroke,
            stroke_paint,
            shape,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        match self.shape {
            ShapeType::Poly => self.draw_poly(canvas),
            ShapeType::PolyQuad => self.draw_quad(canvas),
            ShapeType::PolyCubic => self.draw_cubic(canvas),
            ShapeType::Rect => self.draw_rect(canvas),
            ShapeType::Ellipse => self.draw_ellipse(canvas),
            ShapeType::Line => self.draw_line(canvas),
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
        if let Some(fp) = &self.fill_paint {
            canvas.fill_path(&path, &fp, FillRule::Winding);
        }
        if let Some(sp) = &self.stroke_paint {
            canvas.stroke_path(&path, &sp, &self.stroke)
        }
    }

    fn draw_quad(&self, canvas: &mut Canvas) {
        let mut pb = PathBuilder::new();
        let head = self.points[0];
        let tail = &mut self.points[1..].to_vec();
        pb.move_to(head.x, head.y);
        while tail.len() >= 2 {
            let control = tail.pop().unwrap();
            let p = tail.pop().unwrap();
            pb.quad_to(control.x, control.y, p.x, p.y);
        }
        if self.fill_paint.is_some() {
            pb.close();
        }
        let path = pb.finish().unwrap();
        if let Some(fp) = &self.fill_paint {
            canvas.fill_path(&path, &fp, FillRule::Winding);
        }
        if let Some(sp) = &self.stroke_paint {
            canvas.stroke_path(&path, &sp, &self.stroke)
        }
    }

    pub fn draw_cubic(&self, canvas: &mut Canvas) {
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
        if self.fill_paint.is_some() {
            pb.close();
        }
        let path = pb.finish().unwrap();
        if let Some(fp) = &self.fill_paint {
            canvas.fill_path(&path, &fp, FillRule::Winding);
        }
        if let Some(sp) = &self.stroke_paint {
            canvas.stroke_path(&path, &sp, &self.stroke)
        }
    }

    fn draw_rect(&self, canvas: &mut Canvas) {
        if self.points.len() < 2 {
            panic!("Rectangls points vector contains less than 2 points");
        }
        let left = self.points[0].x;
        let top = self.points[0].y;
        let right = self.points[1].x;
        let bottom = self.points[1].y;
        let r = Rect::from_ltrb(left, top, right, bottom).unwrap();
        let pb = PathBuilder::from_rect(r);
        if let Some(fp) = &self.fill_paint {
            canvas.fill_path(&pb, &fp, FillRule::Winding);
        }
        if let Some(sp) = &self.stroke_paint {
            canvas.stroke_path(&pb, &sp, &self.stroke)
        }
    }

    fn draw_ellipse(&self, canvas: &mut Canvas) {
        if self.points.len() < 2 {
            panic!("Ellipse points vector contains less than 2 points");
        }
        let cx = self.points[0].x;
        let cy = self.points[0].y;
        let w = self.points[1].x;
        let _h = self.points[1].y;
        // XXX Fixme to scale to ellipse when tiny_skia updates;
        let pb = PathBuilder::from_circle(cx, cy, w).unwrap();
        if let Some(fp) = &self.fill_paint {
            canvas.fill_path(&pb, &fp, FillRule::Winding);
        }
        if let Some(sp) = &self.stroke_paint {
            canvas.stroke_path(&pb, &sp, &self.stroke)
        }
        canvas.reset_transform();
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
        if let Some(sp) = &self.stroke_paint {
            canvas.stroke_path(&path, &sp, &self.stroke)
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShapeBuilder<'a> {
    fill_shader: Option<Shader<'a>>,
    stroke_shader: Option<Shader<'a>>,
    stroke_width: f32,
    line_cap: LineCap,
    line_join: LineJoin,
    stroke_dash: Option<StrokeDash>,
    points: Box<Vec<Point>>,
    shape: ShapeType,
}

impl<'a> ShapeBuilder<'a> {
    pub fn new() -> Self {
        Self {
            fill_shader: Some(Shader::SolidColor(Color::WHITE)),
            stroke_shader: Some(Shader::SolidColor(Color::BLACK)),
            stroke_width: 1.0,
            line_cap: LineCap::default(),
            line_join: LineJoin::default(),
            stroke_dash: None,
            points: Box::new(vec![]),
            shape: ShapeType::Poly,
        }
    }

    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_shader = Some(Shader::SolidColor(color));
        self
    }

    pub fn no_fill(mut self) -> Self {
        self.fill_shader = None;
        self
    }

    pub fn no_stroke(mut self) -> Self {
        self.stroke_shader = None;
        self
    }

    pub fn stroke_color(mut self, color: Color) -> Self {
        self.stroke_shader = Some(Shader::SolidColor(color));
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
        self.points = Box::new(vec![xy, pt2(xy.x + wh.x, xy.y + wh.y)]);
        self
    }

    pub fn ellipse(mut self, center: Point, wh: Point) -> Self {
        self.shape = ShapeType::Ellipse;
        self.points = Box::new(vec![center, wh]);
        self
    }

    pub fn circle(mut self, center: Point, radius: f32) -> Self {
        self.shape = ShapeType::Ellipse;
        self.points = Box::new(vec![center, pt2(radius, radius)]);
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

    pub fn build(self) -> Shape<'a> {
        let mut fill_paint = None;
        let mut stroke_paint = None;
        if let Some(fs) = self.fill_shader {
            let mut fp = Paint::default();
            fp.shader = fs;
            fp.anti_alias = true;
            fill_paint = Some(fp);
        };
        if let Some(ss) = self.stroke_shader {
            let mut sp = Paint::default();
            sp.shader = ss;
            sp.anti_alias = true;
            stroke_paint = Some(sp);
        };
        let mut stroke = Stroke::default();
        stroke.width = self.stroke_width;
        stroke.line_cap = self.line_cap;
        stroke.line_join = self.line_join;
        stroke.width = self.stroke_width;
        Shape::new(self.points, fill_paint, stroke, stroke_paint, self.shape)
    }
}

pub fn stroke(weight: f32) -> Stroke {
    let mut stroke = Stroke::default();
    stroke.width = weight;
    stroke
}

pub fn line(
    canvas: &mut Canvas,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    stroke: &Stroke,
    stroke_paint: &Paint,
) {
    let mut pb = PathBuilder::new();
    pb.move_to(x0, y0);
    pb.line_to(x1, y1);
    let path = pb.finish().unwrap();
    canvas.stroke_path(&path, &stroke_paint, &stroke);
}