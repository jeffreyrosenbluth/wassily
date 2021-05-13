use crate::base;
use raqote::DrawTarget;
use raqote::{self, DrawOptions, Source};

pub struct Canvas(DrawTarget);

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        let dt = DrawTarget::new(width as i32, height as i32);
        Canvas(dt)
    }

    pub fn fill_path(&mut self, path: &base::Path, texture: base::Texture) {
        let raqote_path: raqote::Path = path.into();
        let source: raqote::Source = texture.into();
        self.0.fill(&raqote_path, &source, &DrawOptions::default());
    }

    pub fn stroke_path(
        &mut self,
        path: &base::Path,
        texture: base::Texture,
        stroke: &base::Stroke,
    ) {
        let raqote_path: raqote::Path = path.into();
        let source: raqote::Source = texture.into();
        let stroke = stroke.into();
        self.0
            .stroke(&raqote_path, &source, &stroke, &DrawOptions::default());
    }

    pub fn background(&mut self, color: base::RGBA) {
        let t = base::Texture::SolidColor(color);
        let c = t.into();
        self.0.clear(c);
    }

    pub fn save_png<P: AsRef<std::path::Path>>(&self, path: P) {
        self.0.write_png(path).unwrap();
    }

    // pub fn load_png<P: AsRef<std::path::Path>>(path: P) -> Self {
    //     Self(raqote::Pixmap::load_png(path).expect("Error loading png"))
    // }
}

impl From<base::FillRule> for raqote::Winding {
    fn from(fr: base::FillRule) -> Self {
        match fr {
            base::FillRule::Winding => raqote::Winding::NonZero,
            base::FillRule::EvenOdd => raqote::Winding::EvenOdd,
        }
    }
}

impl From<&base::Path> for raqote::Path {
    fn from(path: &base::Path) -> Self {
        let mut pb = raqote::PathBuilder::new();
        for cmd in path.cmds.clone() {
            match cmd {
                base::PathCmd::MoveTo(p) => pb.move_to(p.x, p.y),
                base::PathCmd::LineTo(p) => pb.line_to(p.x, p.y),
                base::PathCmd::QuadTo(c, p) => pb.quad_to(c.x, c.y, p.x, p.y),
                base::PathCmd::CubicTo(c1, c2, p) => pb.cubic_to(c1.x, c1.y, c2.x, c2.y, p.x, p.y),
                base::PathCmd::Close => pb.close(),
            }
        }
        let mut p = pb.finish().transform(&path.transform);
        p.winding = path.fill_rule.into();
        p
    }
}

impl From<base::Texture> for raqote::SolidSource {
    fn from(t: base::Texture) -> Self {
        match t {
            base::Texture::SolidColor(c) => {
                let r = c.r * 255.0;
                let g = c.g * 255.0;
                let b = c.b * 255.0;
                let a = c.a * 255.0;
                raqote::SolidSource::from_unpremultiplied_argb(a as u8, r as u8, g as u8, b as u8)
            }
        }
    }
}

impl<'a> From<base::Texture> for raqote::Source<'a> {
    fn from(t: base::Texture) -> Self {
        let c = t.into();
        Source::Solid(c)
    }
}

impl From<&base::Stroke> for raqote::StrokeStyle {
    fn from(s: &base::Stroke) -> Self {
        let mut raqote_stroke = raqote::StrokeStyle::default();
        raqote_stroke.width = s.width;
        raqote_stroke.miter_limit = s.miter_limit;
        raqote_stroke.cap = match s.line_cap {
            base::LineCap::Butt => raqote::LineCap::Butt,
            base::LineCap::Round => raqote::LineCap::Round,
            base::LineCap::Square => raqote::LineCap::Square,
        };
        raqote_stroke.join = match s.line_join {
            base::LineJoin::Miter => raqote::LineJoin::Miter,
            base::LineJoin::Round => raqote::LineJoin::Round,
            base::LineJoin::Bevel => raqote::LineJoin::Bevel,
        };
        raqote_stroke
    }
}
