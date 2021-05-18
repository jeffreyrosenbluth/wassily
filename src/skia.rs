use crate::base::{self, Sketch, RGBA, Texture};
use tiny_skia as skia;
use tiny_skia::Pixmap;

#[derive(Clone, PartialEq, Debug)]
pub struct Canvas(Pixmap);

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        let pixmap = Pixmap::new(width, height).expect("Pixmap::new failed");
        Canvas(pixmap)
    }

    pub fn load_png<P: AsRef<std::path::Path>>(path: P) -> Self {
        Self(skia::Pixmap::load_png(path).expect("Error loading png"))
    }
}

impl Sketch for Canvas {
    fn fill_path(&mut self, path: &base::Path, texture: &base::Texture) {
        let skia_path: skia::Path = path.into();
        let mut paint: skia::Paint = texture.into();
        paint.anti_alias = true;
        let fill_rule: skia::FillRule = path.fill_rule.into();
        let transform = to_transform(path.transform);
        self.0
            .fill_path(&skia_path, &paint, fill_rule, transform, None);
    }

    fn stroke_path(&mut self, path: &base::Path, texture: &base::Texture, stroke: &base::Stroke) {
        let skia_path: skia::Path = path.into();
        let mut paint: skia::Paint = texture.into();
        paint.anti_alias = true;
        let stroke = stroke.into();
        let transform = to_transform(path.transform);
        self.0
            .stroke_path(&skia_path, &paint, &stroke, transform, None);
    }

    fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32, texture: &base::Texture) {
        let mut paint: skia::Paint = texture.into();
        paint.anti_alias = true;
        let rect: skia::Rect = skia::Rect::from_xywh(x, y, width, height).unwrap();
        self.0
            .fill_rect(rect, &paint, skia::Transform::identity(), None);
    }

    fn fill(&mut self, color: RGBA) {
        let c = skia::Color::from_rgba(color.r, color.g, color.b, color.a);
        self.0.fill(c.unwrap());
    }

    fn save<P: AsRef<std::path::Path>>(&self, path: P) {
        self.0.save_png(path).unwrap();
    }
}

impl From<base::FillRule> for skia::FillRule {
    fn from(fr: base::FillRule) -> Self {
        match fr {
            base::FillRule::Winding => skia::FillRule::Winding,
            base::FillRule::EvenOdd => skia::FillRule::EvenOdd,
        }
    }
}

impl From<&base::Path> for skia::Path {
    fn from(path: &base::Path) -> Self {
        let mut pb = skia::PathBuilder::new();
        for cmd in path.cmds.clone() {
            match cmd {
                base::PathCmd::MoveTo(p) => pb.move_to(p.x, p.y),
                base::PathCmd::LineTo(p) => pb.line_to(p.x, p.y),
                base::PathCmd::QuadTo(c, p) => pb.quad_to(c.x, c.y, p.x, p.y),
                base::PathCmd::CubicTo(c1, c2, p) => pb.cubic_to(c1.x, c1.y, c2.x, c2.y, p.x, p.y),
                base::PathCmd::Close => pb.close(),
            }
        }
        pb.finish().unwrap()
    }
}

impl From<base::RGBA> for skia::Color {
    fn from(c: base::RGBA) -> Self {
        let r = c.r;
        let g = c.g;
        let b = c.b;
        let a = c.a;
        skia::Color::from_rgba(r, g, b, a).unwrap()
    }
}

impl<'a> From<&Texture> for skia::Paint<'a> {
    fn from(t: &Texture) -> Self {
        let mut p = Self::default();
        match t {
            Texture::SolidColor(c) => {
                p.set_color((*c).into());
                p
            }
            Texture::LinearGradient(g) => {
                let start = skia::Point {
                    x: g.start.x,
                    y: g.start.y,
                };
                let end = skia::Point {
                    x: g.end.x,
                    y: g.end.y,
                };
                let stops = g
                    .stops
                    .iter()
                    .map(|s| skia::GradientStop::new(s.position, s.color.into()))
                    .collect();
                let mode = g.mode.into();
                let transform = to_transform(g.transform);
                p.shader = skia::LinearGradient::new(start, end, stops, mode, transform).unwrap();
                p
            }
            Texture::RadialGradient(g) => {
                let start = skia::Point {
                    x: g.start.x,
                    y: g.start.y,
                };
                let end = skia::Point {
                    x: g.end.x,
                    y: g.end.y,
                };
                let stops = g
                    .stops
                    .iter()
                    .map(|s| skia::GradientStop::new(s.position, s.color.into()))
                    .collect();
                let mode = g.mode.into();
                let transform = to_transform(g.transform);
                let radius = g.radius;
                p.shader =
                    skia::RadialGradient::new(start, end, radius, stops, mode, transform).unwrap();
                p
            }
        }
    }
}

fn to_transform(t: base::Transform) -> skia::Transform {
    skia::Transform::from_row(t.m11, t.m12, t.m21, t.m22, t.m31, t.m32)
}

impl From<&base::Stroke> for skia::Stroke {
    fn from(s: &base::Stroke) -> Self {
        let mut skia_stroke = skia::Stroke::default();
        skia_stroke.width = s.width;
        skia_stroke.miter_limit = s.miter_limit;
        skia_stroke.line_cap = match s.line_cap {
            base::LineCap::Butt => skia::LineCap::Butt,
            base::LineCap::Round => skia::LineCap::Round,
            base::LineCap::Square => skia::LineCap::Square,
        };
        skia_stroke.line_join = match s.line_join {
            base::LineJoin::Miter => skia::LineJoin::Miter,
            base::LineJoin::Round => skia::LineJoin::Round,
            base::LineJoin::Bevel => skia::LineJoin::Bevel,
        };
        skia_stroke
    }
}

impl From<base::SpreadMode> for skia::SpreadMode {
    fn from(sm: base::SpreadMode) -> Self {
        match sm {
            base::SpreadMode::Pad => skia::SpreadMode::Pad,
            base::SpreadMode::Reflect => skia::SpreadMode::Reflect,
            base::SpreadMode::Repeat => skia::SpreadMode::Repeat,
        }
    }
}
