pub use crate::prelude::{point2, Point, Transform, Vector};

pub trait Sketch {
    fn fill_path(&mut self, path: &Path, texture: &Texture);
    fn stroke_path(&mut self, path: &Path, texture: &Texture, stroke: &Stroke);
    fn fill(&mut self, color: RGBA);
    fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32, texture: &Texture) {
        let rect = Path::rect(x, y, width, height);
        self.fill_path(&rect, texture);
    }
    fn save<P: AsRef<std::path::Path>>(&self, path: P);
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct RGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub const WHITE: RGBA = RGBA::new(1.0, 1.0, 1.0, 1.0);
pub const BLACK: RGBA = RGBA::new(0.0, 0.0, 0.0, 1.0);

impl RGBA {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn with_8(r: u8, g: u8, b: u8, a: u8) -> Self {
        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;
        let a = a as f32 / 255.0;
        Self { r, g, b, a }
    }

    pub fn as_8(&self) -> (u8, u8, u8, u8) {
        let r = (self.r * 255.0) as u8;
        let g = (self.g * 255.0) as u8;
        let b = (self.b * 255.0) as u8;
        let a = (self.a * 255.0) as u8;
        (r, g, b, a)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct GradientStop {
    pub position: f32,
    pub color: RGBA,
}

impl GradientStop {
    pub fn new(position: f32, color: RGBA) -> Self {
        Self { position, color }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SpreadMode {
    Pad,
    Reflect,
    Repeat,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Gradient {
    pub start: Point,
    pub end: Point,
    pub radius: f32,
    pub stops: Vec<GradientStop>,
    pub mode: SpreadMode,
    pub transform: Transform,
}

impl Gradient {
    pub fn new(
        start: Point,
        end: Point,
        radius: f32,
        stops: Vec<GradientStop>,
        mode: SpreadMode,
        transform: Transform,
    ) -> Self {
        Self {
            start,
            end,
            radius,
            stops,
            mode,
            transform,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum TextureKind {
    SolidColor(RGBA),
    LinearGradient(Gradient),
    RadialGradient(Gradient),
}

impl TextureKind {
    pub fn white() -> Self {
        TextureKind::SolidColor(WHITE)
    }

    pub fn black() -> Self {
        TextureKind::SolidColor(BLACK)
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum BlendMode {
    Clear,
    Source,
    Destination,
    SourceOver,
    DestinationOver,
    SourceIn,
    DestinationIn,
    SourceOut,
    DestinationOut,
    SourceAtop,
    DestinationAtop,
    Xor,
    Plus,
    Modulate,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Multiply,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Texture {
    pub kind: TextureKind,
    pub mode: BlendMode,
    pub anti_alias: bool,
}

impl Texture {
    pub fn new(kind: TextureKind) -> Self {
        Self {
            kind,
            mode: BlendMode::SourceOver,
            anti_alias: true,
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct Stroke {
    pub width: f32,
    pub miter_limit: f32,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
    pub dash: Option<Dash>,
}

impl Default for Stroke {
    fn default() -> Self {
        Stroke {
            width: 1.0,
            miter_limit: 4.0,
            line_cap: LineCap::default(),
            line_join: LineJoin::default(),
            dash: None,
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

#[derive(Clone, Debug, PartialEq)]
pub struct Dash {
    pub array: Vec<f32>,
    pub offset: f32,
}

impl Dash {
    pub fn new(array: Vec<f32>, offset: f32) -> Self {
        Self { array, offset }
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
    pub cmds: Vec<PathCmd>,
    pub fill_rule: FillRule,
    pub transform: Transform,
}

impl Path {
    pub fn rect(x: f32, y: f32, w: f32, h: f32) -> Self {
        let mut pb = PathBuilder::new();
        pb.push_rect(x, y, w, h);
        pb.finish()
    }

    pub fn circle(x: f32, y: f32, r: f32) -> Self {
        let mut pb = PathBuilder::new();
        pb.push_circle(x, y, r);
        pb.finish()
    }

    pub fn ellipse(x: f32, y: f32, w: f32, h: f32) -> Self {
        let mut pb = PathBuilder::new();
        pb.push_ellipse(x, y, w, h);
        pb.finish()
    }
}

pub struct PathBuilder {
    path: Path,
    position: Point,
}

impl From<Path> for PathBuilder {
    fn from(path: Path) -> Self {
        PathBuilder {
            path,
            position: point2(0.0, 0.0),
        }
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
            position: point2(0.0, 0.0),
        }
    }

    /// Moves the current point to `x`, `y`
    pub fn move_to(&mut self, x: f32, y: f32) {
        self.position = point2(x, y);
        self.path.cmds.push(PathCmd::MoveTo(Point::new(x, y)))
    }

    pub fn move_by(&mut self, x: f32, y: f32) {
        let (x, y) = (self.position.x + x, self.position.y + y);
        self.position = point2(x, y);
        self.path.cmds.push(PathCmd::MoveTo(Point::new(x, y)))
    }

    /// Adds a line segment from the current point to `x`, `y`
    pub fn line_to(&mut self, x: f32, y: f32) {
        self.position = point2(x, y);
        self.path.cmds.push(PathCmd::LineTo(Point::new(x, y)))
    }

    pub fn line_by(&mut self, x: f32, y: f32) {
        let (x, y) = (self.position.x + x, self.position.y + y);
        self.position = point2(x, y);
        self.path.cmds.push(PathCmd::LineTo(Point::new(x, y)))
    }

    /// Adds a quadratic bezier from the current point to `x`, `y`,
    /// using a control point of `cx`, `cy`
    pub fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32) {
        self.position = point2(x, y);
        self.path
            .cmds
            .push(PathCmd::QuadTo(Point::new(cx, cy), Point::new(x, y)))
    }

    /// Adds a rect to the path
    pub fn push_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.move_to(x, y);
        self.line_to(x + w, y);
        self.line_to(x + w, y + h);
        self.line_to(x, y + h);
        self.close();
    }

    /// Create an elliptical path
    pub fn push_ellipse(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let k = 0.5522848;
        let x1 = x;
        let y1 = y;
        let cx = k * w / 2.0;
        let cy = k * h / 2.0;
        let x2 = x + w / 2.0;
        let y2 = y + h / 2.0;
        let x = x - w / 2.0;
        let y = y - h / 2.0;
        self.move_to(x, y1);
        self.cubic_to(x, y1 - cy, x1 - cx, y, x1, y);
        self.cubic_to(x1 + cx, y, x2, y1 - cy, x2, y1);
        self.cubic_to(x2, y1 + cy, x1 + cx, y2, x1, y2);
        self.cubic_to(x1 - cx, y2, x, y1 + cy, x, y1);
        self.close();
    }

    pub fn push_circle(&mut self, x: f32, y: f32, r: f32) {
        self.push_ellipse(x, y, 2.0 * r, 2.0 * r);
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

    pub fn set_fillrule(&mut self, fillrule: FillRule) {
        self.path.fill_rule = fillrule;
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.path.transform = transform;
    }

    /// Completes the current path
    pub fn finish(self) -> Path {
        self.path
    }
}