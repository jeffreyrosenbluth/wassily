use crate::base::{self, Sketch};
use svg::Document;
use svg::node::element as vg;
use svg::node::element::path::Data;

pub struct Canvas(Document);

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        let doc = Document::new();
        let doc = doc.set("viewbox", (0, 0, width, height));
        Canvas(doc)
    }
}

impl Sketch for Canvas {
    fn fill_path(&mut self, path: &base::Path, texture: base::Texture) {
        let svg_path: vg::Path = path.into();
    }

    fn stroke_path( &mut self, path: &base::Path, texture: base::Texture, stroke: &base::Stroke) {
        todo!()
    }

    fn background(&mut self, color: base::RGBA) {
        todo!()
    }

    fn save_png<P: AsRef<std::path::Path>>(&self, path: P) {
        todo!()
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
                base::PathCmd::CubicTo(c1, c2, p) => pb = pb.cubic_curve_to((c1.x, c1.y, c2.x, c2.y, p.x, p.y)),
                base::PathCmd::Close => pb = pb.close(),
            }
        }
        let p = vg::Path::new().set("d", pb);
        p
    }
}