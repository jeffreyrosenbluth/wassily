//! Since wassily is designed to have multiple rendering backends we must of a set
//! of internal data structures that are converted to the rendering backend  before
//! rendering. Those data structures are defined here. Backends must implement the
//! Sketch trait.

use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

pub use crate::prelude::{pt, Point, Transform, Vector};

pub trait Sketch {
    fn fill_path(&mut self, path: &Path, texture: &Texture);
    fn stroke_path(&mut self, path: &Path, texture: &Texture, stroke: &Stroke);
    fn fill(&mut self, color: RGBA);
    fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32, texture: &Texture) {
        let rect = Path::rect(x, y, width, height);
        self.fill_path(&rect, texture);
    }
    fn pixel(&mut self, x: f32, y: f32, color: RGBA) {
        let mut tex = Texture::solid_color(color);
        tex.anti_alias = false;
        self.fill_rect(x, y, 1.0, 1.0, &tex);
    }
    fn save<P: AsRef<std::path::Path>>(&self, path: P);
}

/// Unified color format for wassily, formats from external crates e.g. image-rs, tiny-skia,
/// palette ... should be converted to `RGBA` to use in wassily. Hopefully the functions
/// in kolor.rs are sufficient for most purposes.
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Default, Serialize, Deserialize)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBA {
    /// Construct an `RGBA` value from 4 f32s (red, green, blue, alpha) between 0
    /// and 1. Numbers outside of this range will be clamped.
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        let r = (255.0 * r.clamp(0.0, 1.0)) as u8;
        let g = (255.0 * g.clamp(0.0, 1.0)) as u8;
        let b = (255.0 * b.clamp(0.0, 1.0)) as u8;
        let a = (255.0 * a.clamp(0.0, 1.0)) as u8;
        Self { r, g, b, a }
    }

    /// Construct an `RGBA` value from 3 f32s (red, green, blue) between 0 and 1.
    /// The alpha value will be set to 255 - opaque. Numbers outside of
    /// this range will be clamped.
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        let r = (255.0 * r.clamp(0.0, 1.0)) as u8;
        let g = (255.0 * g.clamp(0.0, 1.0)) as u8;
        let b = (255.0 * b.clamp(0.0, 1.0)) as u8;
        Self { r, g, b, a: 255 }
    }

    /// Construct an `RGBA` value from 3 u8s (red, green, blue) between 0
    /// and 255. Alpha is set to 255 - opaque.
    pub const fn rgb8(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Construct an `RGBA` value from 4 u8s (red, green, blue, alpha) between 0
    /// and 255.
    pub const fn rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Return the `RGBA` as a 4-tuple of u8s.
    pub fn as_tuple(&self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    /// Return the `RGBA` as a 4-tuple of f32s.
    pub fn as_f32s(&self) -> (f32, f32, f32, f32) {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;
        let a = self.a as f32 / 255.0;
        (r, g, b, a)
    }
}

// Mostly for debugging purposes.
impl Display for RGBA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:}, {:}, {:}, {:}]", self.r, self.g, self.b, self.a)
    }
}

impl FromStr for RGBA {
    type Err = std::num::ParseIntError;
    fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
        let r: u8 = u8::from_str_radix(&hex_code[1..3], 16)?;
        let g: u8 = u8::from_str_radix(&hex_code[3..5], 16)?;
        let b: u8 = u8::from_str_radix(&hex_code[5..7], 16)?;
        Ok(RGBA::rgb8(r, g, b))
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
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

impl Default for SpreadMode {
    fn default() -> Self {
        Self::Pad
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
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
        TextureKind::SolidColor(RGBA::rgb8(255, 255, 255))
    }

    pub fn black() -> Self {
        TextureKind::SolidColor(RGBA::rgb8(0, 0, 0))
    }
}

impl Default for TextureKind {
    fn default() -> Self {
        Self::SolidColor(RGBA::rgb8(0, 0, 0))
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

impl Default for BlendMode {
    fn default() -> Self {
        Self::SourceOver
    }
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

    pub fn solid_color(color: RGBA) -> Self {
        Self {
            kind: TextureKind::SolidColor(color),
            mode: BlendMode::SourceOver,
            anti_alias: true,
        }
    }

    pub fn mode(&mut self, mode: BlendMode) {
        self.mode = mode;
    }
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            kind: Default::default(),
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
            line_cap: Default::default(),
            line_join: Default::default(),
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

impl Default for FillRule {
    fn default() -> Self {
        FillRule::Winding
    }
}

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone, Default)]
pub struct PathBuilder {
    path: Path,
    position: Point,
}

impl From<Path> for PathBuilder {
    fn from(path: Path) -> Self {
        PathBuilder {
            path,
            position: pt(0.0, 0.0),
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
            position: pt(0.0, 0.0),
        }
    }

    /// Moves the current point to `x`, `y`
    pub fn move_to(&mut self, x: f32, y: f32) {
        self.position = pt(x, y);
        self.path.cmds.push(PathCmd::MoveTo(Point::new(x, y)))
    }

    pub fn move_by(&mut self, x: f32, y: f32) {
        let (x, y) = (self.position.x + x, self.position.y + y);
        self.position = pt(x, y);
        self.path.cmds.push(PathCmd::MoveTo(Point::new(x, y)))
    }

    /// Adds a line segment from the current point to `x`, `y`
    pub fn line_to(&mut self, x: f32, y: f32) {
        self.position = pt(x, y);
        self.path.cmds.push(PathCmd::LineTo(Point::new(x, y)))
    }

    pub fn line_by(&mut self, x: f32, y: f32) {
        let (x, y) = (self.position.x + x, self.position.y + y);
        self.position = pt(x, y);
        self.path.cmds.push(PathCmd::LineTo(Point::new(x, y)))
    }

    /// Adds a quadratic bezier from the current point to `x`, `y`,
    /// using a control point of `cx`, `cy`
    pub fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32) {
        self.position = pt(x, y);
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
