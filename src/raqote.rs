use crate::base::{self, Sketch, Texture, TextureKind};
use base::RGBA;
use num_traits::AsPrimitive;
use raqote::{self, AntialiasMode, DrawOptions, DrawTarget, SolidSource, Source};

pub struct Canvas(pub DrawTarget);

impl Canvas {
    pub fn new<T: AsPrimitive<i32>>(width: T, height: T) -> Self {
        let dt = DrawTarget::new(width.as_(), height.as_());
        Canvas(dt)
    }
}

impl Sketch for Canvas {
    fn fill_path(&mut self, path: &base::Path, texture: &Texture) {
        let raqote_path: raqote::Path = path.into();
        let source: raqote::Source = texture.into();
        let antialias = match texture.anti_alias {
            true => AntialiasMode::Gray,
            false => AntialiasMode::None,
        };
        let draw_options = DrawOptions {
            antialias,
            blend_mode: texture.mode.into(),
            ..Default::default()
        };
        self.0.fill(&raqote_path, &source, &draw_options);
    }

    fn stroke_path(&mut self, path: &base::Path, texture: &Texture, stroke: &base::Stroke) {
        let raqote_path: raqote::Path = path.into();
        let source: raqote::Source = texture.into();
        let stroke = stroke.into();
        let antialias = match texture.anti_alias {
            true => AntialiasMode::Gray,
            false => AntialiasMode::None,
        };
        let draw_options = DrawOptions {
            antialias,
            ..Default::default()
        };
        self.0.stroke(&raqote_path, &source, &stroke, &draw_options);
    }

    fn fill(&mut self, color: base::RGBA) {
        self.0.clear(color.into());
    }

    fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32, texture: &Texture) {
        let src: raqote::Source = texture.into();
        let antialias = match texture.anti_alias {
            true => AntialiasMode::Gray,
            false => AntialiasMode::None,
        };
        let draw_options = DrawOptions {
            antialias,
            blend_mode: texture.mode.into(),
            ..Default::default()
        };
        self.0.fill_rect(x, y, width, height, &src, &draw_options);
    }

    fn save<P: AsRef<std::path::Path>>(&self, path: P) {
        self.0.write_png(path).unwrap();
    }

    fn width(&self) -> u32 {
        self.0.width() as u32
    }

    fn height(&self) -> u32 {
        self.0.height() as u32
    }
}

impl From<RGBA> for SolidSource {
    fn from(c: RGBA) -> Self {
        SolidSource::from_unpremultiplied_argb(c.a, c.r, c.g, c.b)
    }
}

impl From<&RGBA> for SolidSource {
    fn from(c: &RGBA) -> Self {
        SolidSource::from_unpremultiplied_argb(c.a, c.r, c.g, c.b)
    }
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
        for cmd in &path.cmds {
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

impl From<&base::Texture> for raqote::Source<'_> {
    fn from(t: &base::Texture) -> Self {
        match &t.kind {
            TextureKind::SolidColor(c) => {
                let sc: SolidSource = c.into();
                sc.into()
            }
            TextureKind::LinearGradient(g) => {
                let stops = g
                    .stops
                    .iter()
                    .map(|s| {
                        let r = s.color.r;
                        let g = s.color.g;
                        let b = s.color.b;
                        let a = s.color.a;
                        raqote::GradientStop {
                            position: s.position,
                            color: raqote::Color::new(a, r, g, b),
                        }
                    })
                    .collect();
                let gradient = raqote::Gradient { stops };
                let spread = g.mode.into();
                Source::new_linear_gradient(gradient, g.start, g.end, spread)
            }
            TextureKind::RadialGradient(g) => {
                let stops = g
                    .stops
                    .iter()
                    .map(|s| {
                        let r = s.color.r;
                        let g = s.color.g;
                        let b = s.color.b;
                        let a = s.color.a;
                        raqote::GradientStop {
                            position: s.position,
                            color: raqote::Color::new(a, r, g, b),
                        }
                    })
                    .collect();
                let gradient = raqote::Gradient { stops };
                let spread = g.mode.into();
                Source::new_radial_gradient(gradient, g.start, g.radius, spread)
            }
        }
    }
}

impl From<&base::Stroke> for raqote::StrokeStyle {
    fn from(s: &base::Stroke) -> Self {
        let width = s.width;
        let miter_limit = s.miter_limit;
        let cap = match s.line_cap {
            base::LineCap::Butt => raqote::LineCap::Butt,
            base::LineCap::Round => raqote::LineCap::Round,
            base::LineCap::Square => raqote::LineCap::Square,
        };
        let join = match s.line_join {
            base::LineJoin::Miter => raqote::LineJoin::Miter,
            base::LineJoin::Round => raqote::LineJoin::Round,
            base::LineJoin::Bevel => raqote::LineJoin::Bevel,
        };
        let dash_array = match s.dash {
            Some(ref dash) => dash.array.clone(),
            None => {
                vec![]
            }
        };
        let dash_offset = match &s.dash {
            Some(dash) => dash.offset,
            None => 0.0,
        };
        raqote::StrokeStyle {
            width,
            miter_limit,
            cap,
            join,
            dash_array,
            dash_offset,
        }
    }
}

impl From<base::SpreadMode> for raqote::Spread {
    fn from(sm: base::SpreadMode) -> Self {
        match sm {
            base::SpreadMode::Pad => raqote::Spread::Pad,
            base::SpreadMode::Reflect => raqote::Spread::Reflect,
            base::SpreadMode::Repeat => raqote::Spread::Repeat,
        }
    }
}

impl From<base::BlendMode> for raqote::BlendMode {
    fn from(mode: base::BlendMode) -> Self {
        match mode {
            base::BlendMode::Clear => raqote::BlendMode::Clear,
            base::BlendMode::Source => raqote::BlendMode::Src,
            base::BlendMode::Destination => raqote::BlendMode::Dst,
            base::BlendMode::SourceOver => raqote::BlendMode::SrcOver,
            base::BlendMode::DestinationOver => raqote::BlendMode::DstOver,
            base::BlendMode::SourceIn => raqote::BlendMode::SrcIn,
            base::BlendMode::DestinationIn => raqote::BlendMode::DstIn,
            base::BlendMode::SourceOut => raqote::BlendMode::SrcOut,
            base::BlendMode::DestinationOut => raqote::BlendMode::DstOut,
            base::BlendMode::SourceAtop => raqote::BlendMode::SrcAtop,
            base::BlendMode::DestinationAtop => raqote::BlendMode::DstAtop,
            base::BlendMode::Xor => raqote::BlendMode::Xor,
            base::BlendMode::Plus => raqote::BlendMode::Add,
            base::BlendMode::Modulate => {
                panic!("BlendMode::Modulate is not available in raqote")
            }
            base::BlendMode::Screen => raqote::BlendMode::Screen,
            base::BlendMode::Overlay => raqote::BlendMode::Overlay,
            base::BlendMode::Darken => raqote::BlendMode::Darken,
            base::BlendMode::Lighten => raqote::BlendMode::Lighten,
            base::BlendMode::ColorDodge => raqote::BlendMode::ColorDodge,
            base::BlendMode::ColorBurn => raqote::BlendMode::ColorBurn,
            base::BlendMode::HardLight => raqote::BlendMode::HardLight,
            base::BlendMode::SoftLight => raqote::BlendMode::SoftLight,
            base::BlendMode::Difference => raqote::BlendMode::Difference,
            base::BlendMode::Exclusion => raqote::BlendMode::Exclusion,
            base::BlendMode::Multiply => raqote::BlendMode::Multiply,
            base::BlendMode::Hue => raqote::BlendMode::Hue,
            base::BlendMode::Saturation => raqote::BlendMode::Saturation,
            base::BlendMode::Color => raqote::BlendMode::Color,
            base::BlendMode::Luminosity => raqote::BlendMode::Luminosity,
        }
    }
}
