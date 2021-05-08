use crate::{Point, Transform, Vector};
use lyon_geom::{Angle, Arc};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct RGBA {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl RGBA {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn white() -> Self {
        RGBA {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }

    pub fn black() -> Self {
        RGBA {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Texture {
    SolidColor(RGBA),
}

impl Texture {
    pub fn white() -> Self {
        Texture::SolidColor(RGBA::white())
    }

    pub fn black() -> Self {
        Texture::SolidColor(RGBA::black())
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct Stroke {
    pub width: f32,
    pub miter_limit: f32,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
}

impl Default for Stroke {
    fn default() -> Self {
        Stroke {
            width: 1.0,
            miter_limit: 4.0,
            line_cap: LineCap::default(),
            line_join: LineJoin::default(),
        }
    }
}

/// Draws at the beginning and end of an open path contour.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

impl Default for LineCap {
    fn default() -> Self {
        LineCap::Butt
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

impl Default for LineJoin {
    fn default() -> Self {
        LineJoin::Miter
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PathCmd {
    MoveTo(Point),
    LineTo(Point),
    QuadTo(Point, Point),
    CubicTo(Point, Point, Point),
    Close,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FillRule {
    Winding,
    EvenOdd,
}

pub struct Path {
    pub(crate) cmds: Vec<PathCmd>,
    pub(crate) fill_rule: FillRule,
    pub(crate) transform: Transform,
}

pub struct PathBuilder {
    path: Path,
}
impl From<Path> for PathBuilder {
    fn from(path: Path) -> Self {
        PathBuilder { path }
    }
}

impl PathBuilder {
    pub fn new() -> PathBuilder {
        PathBuilder {
            path: Path {
                cmds: Vec::new(),
                fill_rule: FillRule::Winding,
                transform: Transform::identity(),
            },
        }
    }

    /// Moves the current point to `x`, `y`
    pub fn move_to(&mut self, x: f32, y: f32) {
        self.path.cmds.push(PathCmd::MoveTo(Point::new(x, y)))
    }

    /// Adds a line segment from the current point to `x`, `y`
    pub fn line_to(&mut self, x: f32, y: f32) {
        self.path.cmds.push(PathCmd::LineTo(Point::new(x, y)))
    }

    /// Adds a quadratic bezier from the current point to `x`, `y`,
    /// using a control point of `cx`, `cy`
    pub fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32) {
        self.path
            .cmds
            .push(PathCmd::QuadTo(Point::new(cx, cy), Point::new(x, y)))
    }

    /// Adds a rect to the path
    pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.move_to(x, y);
        self.line_to(x + width, y);
        self.line_to(x + width, y + height);
        self.line_to(x, y + height);
        self.close();
    }

    /// Adds a cubic bezier from the current point to `x`, `y`,
    /// using control points `cx1`, `cy1` and `cx2`, `cy2`
    pub fn cubic_to(&mut self, cx1: f32, cy1: f32, cx2: f32, cy2: f32, x: f32, y: f32) {
        self.path.cmds.push(PathCmd::CubicTo(
            Point::new(cx1, cy1),
            Point::new(cx2, cy2),
            Point::new(x, y),
        ))
    }

    /// Closes the current subpath
    pub fn close(&mut self) {
        self.path.cmds.push(PathCmd::Close)
    }

    /// Adds an arc approximated by quadratic beziers with center `x`, `y`
    /// and radius `r` starting at `start_angle` and sweeping by `sweep_angle`.
    /// For a positive `sweep_angle` the sweep is done clockwise, for a negative
    /// `sweep_angle` the sweep is done counterclockwise.
    pub fn arc(&mut self, x: f32, y: f32, r: f32, start_angle: f32, sweep_angle: f32) {
        //XXX: handle the current point being the wrong spot
        let a: Arc<f32> = Arc {
            center: Point::new(x, y),
            radii: Vector::new(r, r),
            start_angle: Angle::radians(start_angle),
            sweep_angle: Angle::radians(sweep_angle),
            x_rotation: Angle::zero(),
        };
        let start = a.from();
        self.line_to(start.x, start.y);
        a.for_each_quadratic_bezier(&mut |q| {
            self.quad_to(q.ctrl.x, q.ctrl.y, q.to.x, q.to.y);
        });
    }

    /// Completes the current path
    pub fn finish(self) -> Path {
        self.path
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Canvas {
    width: f32,
    height: f32,
}

impl Canvas {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn fill_path(
        &mut self,
        path: &Path,
        fill_texture: &Texture,
        fill_rule: FillRule,
        transform: Transform,
    ) {
    }

    pub fn stroke_path(
        &mut self,
        path: &Path,
        stroke_texture: &Texture,
        stroke: &Stroke,
        transform: Transform,
    ) {
    }
}
