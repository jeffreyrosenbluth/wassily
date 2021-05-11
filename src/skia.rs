use crate::base;
use tiny_skia as skia;
use tiny_skia::Pixmap;

#[derive(Clone, PartialEq, Debug)]
pub struct Canvas(Pixmap);

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        let pixmap = Pixmap::new(width, height).expect("Pixmap::new failed");
        Canvas(pixmap)
    }

    pub fn fill_path(
        &mut self,
        path: &base::Path,
        texture: base::Texture,
        transform: base::Transform,
    ) {
        let skia_path: skia::Path = path.into();
        let paint = texture.into();
        let fill_rule: skia::FillRule = path.fill_rule.into();
        let transform = to_transform(path.transform);
        self.0
            .fill_path(&skia_path, &paint, fill_rule, transform, None);
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
        for cmd in path.cmds {
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

impl<'a> From<base::Texture> for skia::Paint<'a> {
    fn from(t: base::Texture) -> Self {
        match t {
            base::Texture::SolidColor(c) => {
                let p = Self::default();
                let r = c.r * 255.0; 
                let g = c.g * 255.0; 
                let b = c.b * 255.0; 
                let a = c.a * 255.0; 
                p.set_color_rgba8(r as u8, g as u8, b as u8, a as u8);
                p
            }
        }
    }
}

    fn to_transform(t: base::Transform) -> skia::Transform {
        skia::Transform::from_row(t.m11, t.m22, t.m21, t.m12, t.m31, t.m32)
}
