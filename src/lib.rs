use tiny_skia::*;

pub const TAU: f32 = std::f32::consts::TAU;
pub const PI: f32 = std::f32::consts::PI;

pub fn pt2(x: f32, y: f32) -> Point {
    Point::from_xy(x, y)
}

pub fn map_range(x: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (x - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
}

pub fn background(canvas: &mut Canvas, width: u32, height: u32, color: Color) {
    let mut paint = Paint::default();
    paint.set_color(color);
    let rect = IntRect::from_xywh(0, 0, width, height).unwrap().to_rect();
    canvas.fill_rect(rect, &paint);
}

#[derive(Debug, Clone)]
pub struct Shape<'a> {
    points: Box<Vec<Point>>,
    fill_paint: Option<Paint<'a>>,
    stroke: Stroke,
    stroke_paint: Option<Paint<'a>>,
}

impl<'a> Shape<'a> {
    pub(crate) fn new(
        points: Box<Vec<Point>>,
        fill_paint: Option<Paint<'a>>,
        stroke: Stroke,
        stroke_paint: Option<Paint<'a>>,
    ) -> Self {
        Self {
            points,
            fill_paint,
            stroke,
            stroke_paint,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
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

    pub fn draw_quad(&self, canvas: &mut Canvas) {
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

    pub fn draw_rect(&self, canvas: &mut Canvas) {
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
    
    pub fn draw_ellipse(&self, canvas: &mut Canvas) {
        if self.points.len() < 2 {
            panic!("Ellipse points vector contains less than 2 points");
        }
        let cx = self.points[0].x;
        let cy = self.points[0].y;
        let w = self.points[1].x;
        let h = self.points[1].y;
        let pb = PathBuilder::from_circle(cx, cy, 1.0).unwrap();
        canvas.scale(w, h);
        if let Some(fp) = &self.fill_paint {
            canvas.fill_path(&pb, &fp, FillRule::Winding);
        }
        if let Some(sp) = &self.stroke_paint {
            canvas.stroke_path(&pb, &sp, &self.stroke)
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
        self.points = Box::new(vec![lt, rb]);
        self
    }

    pub fn rect_xywh(mut self, xy: Point, wh: Point) -> Self {
        self.points = Box::new(vec![xy, pt2(xy.x + wh.x, xy.y + wh.y)]);
        self
    }

    pub fn ellipse(mut self, center: Point, wh: Point) -> Self {
        self.points = Box::new(vec![center, wh]);
        self
    }

    pub fn circle(mut self, center: Point, radius: f32) -> Self {
        self.points = Box::new(vec![center, pt2(radius, radius)]);
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
        Shape::new(self.points, fill_paint, stroke, stroke_paint)
    }
}

pub fn circle(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    radius: f32,
    fill_paint: &Paint,
    stroke: &Stroke,
    stroke_paint: &Paint,
) {
    let path =
        PathBuilder::from_circle(cx, cy, radius).expect("Circle radius must be greater than 0");
    canvas.fill_path(&path, &fill_paint, FillRule::Winding);
    canvas.stroke_path(&path, &stroke_paint, &stroke);
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

pub fn polygon(
    canvas: &mut Canvas,
    points: &[Point],
    fill_paint: &Paint,
    stroke: &Stroke,
    stroke_paint: &Paint,
) {
    let mut pb = PathBuilder::new();
    let head = points[0];
    let tail = &points[1..];
    pb.move_to(head.x, head.y);
    for p in tail {
        pb.line_to(p.x, p.y);
    }
    pb.close();
    let path = pb.finish().unwrap();
    canvas.fill_path(&path, &fill_paint, FillRule::Winding);
    canvas.stroke_path(&path, &stroke_paint, &stroke);
}

pub fn polyline(canvas: &mut Canvas, points: &[Point], stroke: &Stroke, stroke_paint: &Paint) {
    let mut pb = PathBuilder::new();
    let head = points[0];
    let tail = &points[1..];
    pb.move_to(head.x, head.y);
    for p in tail {
        pb.line_to(p.x, p.y);
    }
    let path = pb.finish().unwrap();
    canvas.stroke_path(&path, &stroke_paint, &stroke);
}

pub fn polycurve(
    canvas: &mut Canvas,
    points: &[Point],
    stroke: &Stroke,
    stroke_paint: &Paint,
    control: Point,
) {
    let mut pb = PathBuilder::new();
    let head = points[0];
    let tail = &points[1..];
    pb.move_to(head.x, head.y);
    for p in tail {
        pb.quad_to(control.x, control.y, p.x, p.y);
    }
    let path = pb.finish().unwrap();
    canvas.stroke_path(&path, &stroke_paint, &stroke);
}

pub fn polycurve3(
    canvas: &mut Canvas,
    points: &[Point],
    stroke: &Stroke,
    stroke_paint: &Paint,
    control1: Point,
    control2: Point,
) {
    let mut pb = PathBuilder::new();
    let head = points[0];
    let tail = &points[1..];
    pb.move_to(head.x, head.y);
    for p in tail {
        pb.cubic_to(control1.x, control1.y, control2.x, control2.y, p.x, p.y);
    }
    let path = pb.finish().unwrap();
    canvas.stroke_path(&path, &stroke_paint, &stroke);
}
// -----------------------------------------------------------------------------
// Create a grid of values based on a function of it's coordinates. Used for
// example for flow fields.
pub struct Grid<T> {
    pub width: f32,
    pub height: f32,
    pub spacing: f32,
    pub grid: Vec<T>,
    pub pts: Vec<Point>,
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn new(width: f32, height: f32, spacing: f32, gen: impl Fn(f32, f32) -> T) -> Self {
        let rows = (height / spacing) as usize;
        let cols = (width / spacing) as usize;
        let mut grid = vec![];
        let mut pts = vec![];
        for i in 0..rows {
            let y = i as f32 * spacing;
            for j in 0..cols {
                let x = j as f32 * spacing;
                grid.push(gen(x, y));
                pts.push(pt2(x, y));
            }
        }
        Self {
            width,
            height,
            spacing,
            grid,
            pts,
        }
    }

    pub fn rows(&self) -> usize {
        (self.height / self.spacing) as usize
    }

    pub fn cols(&self) -> usize {
        (self.width / self.spacing) as usize
    }

    pub fn get(&self, x: f32, y: f32) -> T {
        let n = self.rows();
        let m = self.cols();
        let xn = x;
        let yn = y;

        let mut col = if xn < 0.0 {
            0
        } else {
            (x / self.spacing) as usize
        };
        let mut row = if yn < 0.0 {
            0
        } else {
            (y / self.spacing) as usize
        };

        while col >= m {
            col -= 1;
        }
        while row >= n {
            row -= 1;
        }

        self.grid[row * m + col]
    }

    pub fn iter<'a>(&'a self) -> GridIter<'a, T> {
        GridIter {
            grid: self,
            i: 0,
            j: 0,
        }
    }

    pub fn x_bounds(&self) -> (f32, f32) {
        (0.0, self.width)
    }

    pub fn y_bounds(&self) -> (f32, f32) {
        (0.0, self.height)
    }
}

pub struct GridIter<'a, T>
where
    T: Copy,
{
    grid: &'a Grid<T>,
    i: usize,
    j: usize,
}

impl<'a, T> Iterator for GridIter<'a, T>
where
    T: Copy,
{
    type Item = (Point, T);

    fn next(&mut self) -> Option<Self::Item> {
        let n = (self.grid.width / self.grid.spacing) as usize;
        if self.i * n + self.j >= self.grid.grid.len() {
            return None;
        };
        let x = self.j as f32 * self.grid.spacing;
        let y = self.i as f32 * self.grid.spacing;
        let result = (pt2(x, y), self.grid.grid[self.i * n + self.j]);

        if self.j >= n - 1 {
            self.j = 0;
            self.i += 1;
        } else {
            self.j += 1;
        };

        Some(result)
    }
}

pub fn gen_points(
    f: impl Fn(f32) -> f32,
    g: impl Fn(f32) -> f32,
    delta: f32,
    max: f32,
) -> Vec<Point> {
    let mut points = vec![];
    let mut t = 0.0;
    while t <= max {
        let x = f(t);
        let y = g(t);
        points.push(pt2(x, y));
        t += delta;
    }
    points
}
