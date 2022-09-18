use crate::math::Algebra;
use tiny_skia::{
    ClipMask, Color, FillRule, Paint, Path, PathBuilder, Pixmap, Point, Rect, Stroke, Transform,
};

struct Drawing<'a> {
    cmds: Vec<DrawCmd<'a>>,
    width: u32,
    height: u32,
    scale: f32,
    canvas: Pixmap,
}

impl<'a> Drawing<'a> {
    fn new(cmds: Vec<DrawCmd<'a>>, width: u32, height: u32, scale: f32) -> Self {
        let w = (width as f32 * scale).floor() as u32;
        let h = (height as f32 * scale).floor() as u32;
        let canvas = Pixmap::new(w, h).unwrap();
        Self {
            cmds,
            width,
            height,
            scale,
            canvas,
        }
    }
}

enum PathCmd {
    MoveTo(Point),
    LineTo(Point),
    QuadTo(Point, Point),
    CubicTo(Point, Point, Point),
    Close,
}

struct Trail {
    path_cmds: Vec<PathCmd>,
}

impl Trail {
    fn to_path(&self) -> Path {
        let mut pb = PathBuilder::new();
        for cmd in &self.path_cmds {
            match cmd {
                PathCmd::MoveTo(p) => pb.move_to(p.x, p.y),
                PathCmd::LineTo(p) => pb.line_to(p.x, p.y),
                PathCmd::QuadTo(c, p) => pb.quad_to(c.x, c.y, p.x, p.y),
                PathCmd::CubicTo(c1, c2, p) => pb.cubic_to(c1.x, c1.y, c2.x, c2.y, p.x, p.y),
                PathCmd::Close => pb.close(),
            }
        }
        pb.finish().unwrap()
    }

    fn scale(&self, s: f32) -> Self {
        let path_cmds = self.path_cmds.iter().map(|cmd| match cmd {
            PathCmd::MoveTo(p) => PathCmd::MoveTo(p.scale(s)),
            PathCmd::LineTo(p) => PathCmd::LineTo(p.scale(s)),
            PathCmd::QuadTo(c, p) => PathCmd::QuadTo(c.scale(s), p.scale(s)),
            PathCmd::CubicTo(c1, c2, p) => PathCmd::CubicTo(c1.scale(s), c2.scale(s), p.scale(s)),
            PathCmd::Close => PathCmd::Close,
        });
        Trail {
            path_cmds: path_cmds.collect(),
        }
    }
}

enum DrawCmd<'a> {
    Fill {
        trail: &'a Trail,
        paint: &'a Paint<'a>,
        fill_rule: FillRule,
        transform: Transform,
        clip_mask: Option<&'a ClipMask>,
    },
    Stroke {
        trail: &'a Trail,
        paint: &'a Paint<'a>,
        stroke: &'a Stroke,
        transform: Transform,
        clip_mask: Option<&'a ClipMask>,
    },
    FillRect {
        rect: Rect,
        paint: &'a Paint<'a>,
        transform: Transform,
        clip_mask: Option<&'a ClipMask>,
    },
    Clear {
        color: Color,
    },
}

impl<'a> DrawCmd<'a> {
    fn eval(self, canvas: &mut Pixmap, scale: f32) {
        match self {
            DrawCmd::Fill {
                trail,
                paint,
                fill_rule,
                transform,
                clip_mask,
            } => {
                let mut transform = transform;
                transform.tx = scale * transform.tx;
                transform.ty = scale * transform.ty;
                transform = transform.pre_scale(scale, scale);
                canvas.fill_path(&trail.to_path(), paint, fill_rule, transform, clip_mask);
            }
            DrawCmd::Stroke {
                trail,
                paint,
                stroke,
                transform,
                clip_mask,
            } => {
                let mut transform = transform;
                transform.tx = scale * transform.tx;
                transform.ty = scale * transform.ty;
                transform = transform.pre_scale(scale, scale);
                canvas.stroke_path(&trail.to_path(), paint, stroke, transform, clip_mask);
            }
            DrawCmd::FillRect {
                rect,
                paint,
                transform,
                clip_mask,
            } => {
                let mut transform = transform;
                transform.tx = scale * transform.tx;
                transform.ty = scale * transform.ty;
                transform = transform.pre_scale(scale, scale);
                canvas.fill_rect(rect, paint, transform, clip_mask);
            }
            DrawCmd::Clear { color } => {
                canvas.fill(color);
            }
        }
    }
}
