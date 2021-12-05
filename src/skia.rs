use crate::base::{self, Sketch, Texture, TextureKind, RGBA};
use image::{buffer::ConvertBuffer, RgbImage, RgbaImage, ImageFormat};
use image::imageops::rotate180;
use skia::StrokeDash;
use tiny_skia as skia;
use tiny_skia::{Pixmap, PixmapRef};
use num_traits::AsPrimitive;

#[derive(Clone, PartialEq, Debug)]
pub struct Canvas(Pixmap);

impl Canvas {
    pub fn new<T: AsPrimitive<u32>>(width: T, height: T) -> Self {
        let pixmap = Pixmap::new(width.as_(), height.as_()).expect("Pixmap::new failed");
        Canvas(pixmap)
    }

    pub fn load_png<P: AsRef<std::path::Path>>(path: P) -> Self {
        Self(skia::Pixmap::load_png(path).expect("Error loading png"))
    }

    pub fn save_jpg<P: AsRef<std::path::Path>>(&self, path: P) {
        let img: RgbaImage = self.into();
        img.save_with_format(path, ImageFormat::Jpeg).expect("Error writing jpeg");
    }

    pub fn save_tiff<P: AsRef<std::path::Path>>(&self, path: P) {
        let img: RgbaImage = self.into();
        img.save_with_format(path, ImageFormat::Tiff).expect("Error writing tiff");
    }
}

impl Sketch for Canvas {
    fn fill_path(&mut self, path: &base::Path, texture: &base::Texture) {
        let skia_path: skia::Path = path.into();
        let mut paint: skia::Paint = texture.into();
        paint.anti_alias = texture.anti_alias;
        paint.blend_mode = texture.mode.into();
        let fill_rule: skia::FillRule = path.fill_rule.into();
        let transform = to_transform(path.transform);
        self.0
            .fill_path(&skia_path, &paint, fill_rule, transform, None);
    }

    fn stroke_path(&mut self, path: &base::Path, texture: &base::Texture, stroke: &base::Stroke) {
        let skia_path: skia::Path = path.into();
        let mut paint: skia::Paint = texture.into();
        paint.anti_alias = texture.anti_alias;
        paint.blend_mode = texture.mode.into();
        let stroke = stroke.into();
        let transform = to_transform(path.transform);
        self.0
            .stroke_path(&skia_path, &paint, &stroke, transform, None);
    }

    fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32, texture: &base::Texture) {
        let mut paint: skia::Paint = texture.into();
        paint.anti_alias = texture.anti_alias;
        paint.blend_mode = texture.mode.into();
        let rect: skia::Rect = skia::Rect::from_xywh(x, y, width, height).unwrap();
        self.0
            .fill_rect(rect, &paint, skia::Transform::identity(), None);
    }

    fn fill(&mut self, color: RGBA) {
        let c = skia::Color::from_rgba8(color.r, color.g, color.b, color.a);
        self.0.fill(c);
    }

    fn save<P: AsRef<std::path::Path>>(&self, path: P) {
        self.0.save_png(path).unwrap();
    }

    fn pixel(&mut self, x: f32, y: f32, color: RGBA) {
        let width = self.0.width();
        let pixel_map = self.0.pixels_mut();
        let k = y as usize * width as usize + x as usize;
        let c: skia::Color = color.into();
        pixel_map[k] = c.premultiply().to_color_u8();
    }
}

impl From<&RgbaImage> for Canvas {
    fn from(ib: &RgbaImage) -> Self {
        let ib = rotate180(ib);
        let w = ib.width();
        let h = ib.height();
        let data = ib.into_vec();
        let pixmap = PixmapRef::from_bytes(&data, w, h).unwrap();
        Canvas(pixmap.to_owned())
    }
}

impl From<RgbaImage> for Canvas {
    fn from(ib: RgbaImage) -> Self {
        let ib = rotate180(&ib);
        let w = ib.width();
        let h = ib.height();
        let data = ib.into_vec();
        let pixmap = PixmapRef::from_bytes(&data, w, h).unwrap();
        Canvas(pixmap.to_owned())
    }
}

impl From<&RgbImage> for Canvas {
    fn from(ib: &RgbImage) -> Self {
        let buf: RgbaImage = rotate180(&ib.convert());
        let w = buf.width();
        let h = buf.height();
        let data = buf.into_vec();
        let pixmap = PixmapRef::from_bytes(&data, w, h).unwrap();
        Canvas(pixmap.to_owned())
    }
}

impl From<RgbImage> for Canvas {
    fn from(ib: RgbImage) -> Self {
        let buf: RgbaImage = rotate180(&ib.convert());
        let w = buf.width();
        let h = buf.height();
        let data = buf.into_vec();
        let pixmap = PixmapRef::from_bytes(&data, w, h).unwrap();
        Canvas(pixmap.to_owned())
    }
}

impl From<&Canvas> for RgbaImage {
    fn from(canvas: &Canvas) -> Self {
        let w = canvas.0.width();
        let h = canvas.0.height();
        let data = canvas.0.data().to_vec();
        rotate180(&RgbaImage::from_vec(w, h, data).unwrap())
    }
}

impl From<Canvas> for RgbaImage {
    fn from(canvas: Canvas) -> Self {
        let w = canvas.0.width();
        let h = canvas.0.height();
        let data = canvas.0.data().to_vec();
        rotate180(&RgbaImage::from_vec(w, h, data).unwrap())
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
        for cmd in &path.cmds {
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

impl From<&base::RGBA> for skia::Color {
    fn from(c: &base::RGBA) -> Self {
        skia::Color::from_rgba8(c.r, c.g, c.b, c.a)
    }
}

impl From<base::RGBA> for skia::Color {
    fn from(c: base::RGBA) -> Self {
        skia::Color::from_rgba8(c.r, c.g, c.b, c.a)
    }
}

impl<'a> From<&Texture> for skia::Paint<'a> {
    fn from(t: &Texture) -> Self {
        let mut p = Self::default();
        match &t.kind {
            TextureKind::SolidColor(c) => {
                p.set_color(c.into());
                p
            }
            TextureKind::LinearGradient(g) => {
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
            TextureKind::RadialGradient(g) => {
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
        let width = s.width;
        let miter_limit = s.miter_limit;
        let line_cap = match s.line_cap {
            base::LineCap::Butt => skia::LineCap::Butt,
            base::LineCap::Round => skia::LineCap::Round,
            base::LineCap::Square => skia::LineCap::Square,
        };
        let line_join = match s.line_join {
            base::LineJoin::Miter => skia::LineJoin::Miter,
            base::LineJoin::Round => skia::LineJoin::Round,
            base::LineJoin::Bevel => skia::LineJoin::Bevel,
        };
        let dash = match s.dash {
            Some(ref dash) => StrokeDash::new(dash.array.clone(), dash.offset),
            None => None,
        };
        skia::Stroke {width, miter_limit, line_cap, line_join, dash}
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

impl From<base::BlendMode> for skia::BlendMode {
    fn from(mode: base::BlendMode) -> Self {
        match mode {
            base::BlendMode::Clear => skia::BlendMode::Clear,
            base::BlendMode::Source => skia::BlendMode::Source,
            base::BlendMode::Destination => skia::BlendMode::Destination,
            base::BlendMode::SourceOver => skia::BlendMode::SourceOver,
            base::BlendMode::DestinationOver => skia::BlendMode::DestinationOver,
            base::BlendMode::SourceIn => skia::BlendMode::SourceIn,
            base::BlendMode::DestinationIn => skia::BlendMode::DestinationIn,
            base::BlendMode::SourceOut => skia::BlendMode::SourceOut,
            base::BlendMode::DestinationOut => skia::BlendMode::DestinationOut,
            base::BlendMode::SourceAtop => skia::BlendMode::SourceAtop,
            base::BlendMode::DestinationAtop => skia::BlendMode::DestinationAtop,
            base::BlendMode::Xor => skia::BlendMode::Xor,
            base::BlendMode::Plus => skia::BlendMode::Plus,
            base::BlendMode::Modulate => skia::BlendMode::Modulate,
            base::BlendMode::Screen => skia::BlendMode::Screen,
            base::BlendMode::Overlay => skia::BlendMode::Overlay,
            base::BlendMode::Darken => skia::BlendMode::Darken,
            base::BlendMode::Lighten => skia::BlendMode::Lighten,
            base::BlendMode::ColorDodge => skia::BlendMode::ColorDodge,
            base::BlendMode::ColorBurn => skia::BlendMode::ColorBurn,
            base::BlendMode::HardLight => skia::BlendMode::HardLight,
            base::BlendMode::SoftLight => skia::BlendMode::SoftLight,
            base::BlendMode::Difference => skia::BlendMode::Difference,
            base::BlendMode::Exclusion => skia::BlendMode::Exclusion,
            base::BlendMode::Multiply => skia::BlendMode::Multiply,
            base::BlendMode::Hue => skia::BlendMode::Hue,
            base::BlendMode::Saturation => skia::BlendMode::Saturation,
            base::BlendMode::Color => skia::BlendMode::Color,
            base::BlendMode::Luminosity => skia::BlendMode::Luminosity,
        }
    }
}
