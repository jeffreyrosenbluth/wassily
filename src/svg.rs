use core::str;

use crate::base::{self, Sketch, RGBA};
use svg::node::element as vg;
use svg::node::element::path::Data;
use svg::Document;
use num_traits::AsPrimitive;

// The usize is for gradient ids'
pub struct Canvas(pub Document, usize);

impl Canvas {
    pub fn new<T: AsPrimitive<u32>>(width: T, height: T) -> Self {
        let doc = Document::new();
        let doc = doc
            .set("width", width.as_())
            .set("height", height.as_())
            .set("viewbox", (0, 0, width.as_(), height.as_()));
        Canvas(doc, 0)
    }
}

impl Sketch for Canvas {
    fn fill_path(&mut self, path: &base::Path, texture: &base::Texture) {
        let doc = self.0.clone();
        let svg_path: vg::Path = path.into();
        let color = match texture.kind {
            base::TextureKind::SolidColor(c) => c.to_svg(),
            base::TextureKind::LinearGradient(_) => {todo!()}
            base::TextureKind::RadialGradient(_) => {todo!()}
        };
        let svg_path = svg_path
            .set("fill", color.0)
            .set("fill-opacity", color.1)
            .set("transform", transform(path))
            .set("fill-rule", fill_rule(path));

        self.0 = doc.add(svg_path);
    }

    fn stroke_path(&mut self, path: &base::Path, texture: &base::Texture, stroke: &base::Stroke) {
        let doc = self.0.clone();
        let svg_path: vg::Path = path.into();
        let color = match texture.kind {
            base::TextureKind::SolidColor(c) => c.to_svg(),
            base::TextureKind::LinearGradient(_) => {todo!()}
            base::TextureKind::RadialGradient(_) => {todo!()}
        };
        let svg_path = svg_path
            .set("stroke", color.0)
            .set("stroke-opacity", color.1)
            .set("fill", "none")
            .set("stroke-miterlimit", stroke.miter_limit)
            .set("stroke-width", stroke.width)
            .set("stroke-linecap", linecap(stroke))
            .set("stroke-linejoin", linejoin(stroke))
            .set("transform", transform(path));
        self.0 = doc.add(svg_path);
    }

    fn fill(&mut self, color: base::RGBA) {
        let doc = self.0.clone();
        let color = color.to_svg();
        let rect = vg::Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", color.0)
            .set("fill-opacity", color.1);
        self.0 = doc.add(rect);
    }

    fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32, texture: &base::Texture) {
        let doc = self.0.clone();
        let color = match texture.kind {
            base::TextureKind::SolidColor(c) => c.to_svg(),
            base::TextureKind::LinearGradient(_) => {todo!()}
            base::TextureKind::RadialGradient(_) => {todo!()}
        };
        let rect = vg::Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("width", width)
            .set("height", height)
            .set("fill", color.0).set("fill-opacity", color.1);
        self.0 = doc.add(rect);
    }

    fn save<P: AsRef<std::path::Path>>(&self, path: P) {
        svg::save(path, &self.0).unwrap();
    }
}

impl From<&base::Path> for vg::Path {
    fn from(path: &base::Path) -> Self {
        let mut pb = Data::new();
        for cmd in path.cmds.clone() {
            match cmd {
                base::PathCmd::MoveTo(p) => pb = pb.move_to((p.x, p.y)),
                base::PathCmd::LineTo(p) => pb = pb.line_to((p.x, p.y)),
                base::PathCmd::QuadTo(c, p) => pb = pb.quadratic_curve_to((c.x, c.y, p.x, p.y)),
                base::PathCmd::CubicTo(c1, c2, p) => {
                    pb = pb.cubic_curve_to((c1.x, c1.y, c2.x, c2.y, p.x, p.y))
                }
                base::PathCmd::Close => pb = pb.close(),
            }
        }
        vg::Path::new().set("d", pb)
    }
}

impl RGBA {
    fn to_svg(self) -> (String, f32) {
        let (r, g, b, a) = self.as_tuple();
        (format!("rgb({},{},{})", r, g, b), a as f32 / 255.0)
    }
}

fn fill_rule(path: &base::Path) -> &'static str {
    match path.fill_rule {
        base::FillRule::Winding => "nonzero",
        base::FillRule::EvenOdd => "evenodd",
    }
}

fn transform(path: &base::Path) -> String {
    format!(
        "matrix({},{},{},{},{},{})",
        path.transform.m11,
        path.transform.m12,
        path.transform.m21,
        path.transform.m22,
        path.transform.m31,
        path.transform.m32
    )
}

fn linecap(lc: &base::Stroke) -> &'static str {
    match lc.line_cap {
        base::LineCap::Butt => "butt",
        base::LineCap::Round => "round",
        base::LineCap::Square => "square",
    }
}

fn linejoin(lj: &base::Stroke) -> &'static str {
    match lj.line_join {
        base::LineJoin::Bevel => "bevel",
        base::LineJoin::Miter => "miter",
        base::LineJoin::Round => "round",
    }
}
