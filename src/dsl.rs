use crate::math::pt;
use image::{ImageFormat, RgbImage, RgbaImage};
use tiny_skia::{
    BlendMode, Color, FillRule, LinearGradient, Paint, Path, PathBuilder, Pixmap, PixmapRef, Point,
    RadialGradient, Rect, Shader, Stroke, Transform,
};

pub type DrawProgram = Vec<DrawCmd>;

pub struct Drawing {
    pub cmds: Vec<DrawCmd>,
    pub width: u32,
    pub height: u32,
    pub scale: f32,
    pub pixmap: Pixmap,
}

impl Drawing {
    pub fn new(width: u32, height: u32, color: Color, scale: f32) -> Self {
        let w = (width as f32 * scale).floor() as u32;
        let h = (height as f32 * scale).floor() as u32;
        let canvas = Pixmap::new(w, h).unwrap();
        Self {
            cmds: vec![DrawCmd::Clear { color }],
            width,
            height,
            scale,
            pixmap: canvas,
        }
    }

    /// Scaling a drawing will clear any previous pixelmap
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
        let w = (self.width as f32 * scale).floor() as u32;
        let h = (self.height as f32 * scale).floor() as u32;
        self.pixmap = Pixmap::new(w, h).unwrap();
    }

    pub fn add_cmds(&mut self, cmds: Vec<DrawCmd>) {
        self.cmds.extend(cmds);
    }

    pub fn w_f32(&self) -> f32 {
        self.width as f32
    }

    pub fn h_f32(&self) -> f32 {
        self.height as f32
    }

    pub fn render(&mut self) {
        for cmd in &self.cmds {
            cmd.eval(&mut self.pixmap, self.scale);
        }
    }

    pub fn save_png<P: AsRef<std::path::Path>>(&self, path: P) {
        self.pixmap.save_png(path).expect("Error writing png");
    }

    pub fn save_jpg<P: AsRef<std::path::Path>>(&self, path: P) {
        let img: RgbaImage = self.into();
        img.save_with_format(path, ImageFormat::Jpeg)
            .expect("Error writing jpeg");
    }

    pub fn save_tiff<P: AsRef<std::path::Path>>(&self, path: P) {
        let img: RgbaImage = self.into();
        img.save_with_format(path, ImageFormat::Tiff)
            .expect("Error writing tiff");
    }
}

impl From<&RgbaImage> for Drawing {
    fn from(ib: &RgbaImage) -> Self {
        let width = ib.width();
        let height = ib.height();
        let data = ib.clone().into_vec();
        let pixmap = PixmapRef::from_bytes(&data, width, height).unwrap();
        Drawing {
            cmds: Vec::new(),
            width,
            height,
            scale: 1.0,
            pixmap: pixmap.to_owned(),
        }
    }
}

impl From<&RgbImage> for Drawing {
    fn from(ib: &RgbImage) -> Self {
        let width = ib.width();
        let height = ib.height();
        let mut data4: Vec<u8> = Vec::new();
        let data = ib.clone().into_vec();
        for d in data.chunks(3) {
            data4.extend(d);
            data4.push(255)
        }
        let pixmap = PixmapRef::from_bytes(&data4, width, height).unwrap();
        Drawing {
            cmds: Vec::new(),
            width,
            height,
            scale: 1.0,
            pixmap: pixmap.to_owned(),
        }
    }
}

impl From<RgbaImage> for Drawing {
    fn from(ib: RgbaImage) -> Self {
        let width = ib.width();
        let height = ib.height();
        let data = ib.into_vec();
        let pixmap = PixmapRef::from_bytes(&data, width, height).unwrap();
        Drawing {
            cmds: Vec::new(),
            width,
            height,
            scale: 1.0,
            pixmap: pixmap.to_owned(),
        }
    }
}

impl From<RgbImage> for Drawing {
    fn from(ib: RgbImage) -> Self {
        let width = ib.width();
        let height = ib.height();
        let data = ib.into_vec();
        let pixmap = PixmapRef::from_bytes(&data, width, height).unwrap();
        Drawing {
            cmds: Vec::new(),
            width,
            height,
            scale: 1.0,
            pixmap: pixmap.to_owned(),
        }
    }
}

impl From<Drawing> for RgbaImage {
    fn from(canvas: Drawing) -> Self {
        let w = canvas.width;
        let h = canvas.height;
        let data = canvas.pixmap.data().to_vec();
        RgbaImage::from_vec(w, h, data).unwrap()
    }
}

impl From<&Drawing> for RgbaImage {
    fn from(canvas: &Drawing) -> Self {
        let w = canvas.width;
        let h = canvas.height;
        let data = canvas.pixmap.data().to_vec();
        RgbaImage::from_vec(w, h, data).unwrap()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PathCmd {
    MoveTo(Point),
    LineTo(Point),
    QuadTo(Point, Point),
    CubicTo(Point, Point, Point),
    Close,
    Ellipse(Point, Point),
    Circle(Point, f32),
}

#[derive(Clone, Debug)]
pub struct Trail {
    pub path_cmds: Vec<PathCmd>,
}

impl Trail {
    pub fn new() -> Self {
        Self {
            path_cmds: Vec::new(),
        }
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        let point = pt(x, y);
        self.path_cmds.push(PathCmd::MoveTo(point));
    }

    pub fn line_to(&mut self, x: f32, y: f32) {
        let point = pt(x, y);
        self.path_cmds.push(PathCmd::LineTo(point));
    }

    pub fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32) {
        let control = pt(cx, cy);
        let point = pt(x, y);
        self.path_cmds.push(PathCmd::QuadTo(control, point));
    }

    pub fn cubic_to(&mut self, c1x: f32, c1y: f32, c2x: f32, c2y: f32, x: f32, y: f32) {
        let control1 = pt(c1x, c1y);
        let control2 = pt(c2x, c2y);
        let point = pt(x, y);
        self.path_cmds
            .push(PathCmd::CubicTo(control1, control2, point));
    }

    pub fn close(&mut self) {
        self.path_cmds.push(PathCmd::Close);
    }

    pub fn ellipse(&mut self, cx: f32, cy: f32, w: f32, h: f32) {
        self.path_cmds.push(PathCmd::Ellipse(pt(cx, cy), pt(w, h)));
    }

    pub fn circle(&mut self, cx: f32, cy: f32, r: f32) {
        self.path_cmds.push(PathCmd::Circle(pt(cx, cy), r));
    }

    fn to_path(&self) -> Path {
        let mut pb = PathBuilder::new();
        for cmd in &self.path_cmds {
            match cmd {
                PathCmd::MoveTo(p) => pb.move_to(p.x, p.y),
                PathCmd::LineTo(p) => pb.line_to(p.x, p.y),
                PathCmd::QuadTo(c, p) => pb.quad_to(c.x, c.y, p.x, p.y),
                PathCmd::CubicTo(c1, c2, p) => pb.cubic_to(c1.x, c1.y, c2.x, c2.y, p.x, p.y),
                PathCmd::Close => pb.close(),
                PathCmd::Ellipse(c, wh) => {
                    let oval =
                        Rect::from_xywh(c.x - wh.x / 2.0, c.y - wh.y / 2.0, wh.x, wh.y).unwrap();
                    pb.push_oval(oval)
                }
                PathCmd::Circle(p, r) => pb.push_circle(p.x, p.y, *r),
            }
        }
        pb.finish().unwrap()
    }
}

#[derive(Debug, Clone)]
pub enum Texture {
    SolidColor(Color),
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
}

#[derive(Debug, Clone)]
pub struct Ink {
    pub texture: Texture,
    pub blend_mode: BlendMode,
    pub anti_alias: bool,
    pub force_hq_pipeline: bool,
}

impl Ink {
    pub fn set_color(&mut self, color: Color) {
        self.texture = Texture::SolidColor(color);
    }

    fn to_paint<'a>(self) -> Paint<'a> {
        let mut paint = Paint::default();
        paint.blend_mode = self.blend_mode;
        paint.anti_alias = self.anti_alias;
        paint.force_hq_pipeline = self.force_hq_pipeline;
        match self.texture {
            Texture::SolidColor(c) => paint.set_color(c),
            Texture::LinearGradient(lg) => paint.shader = Shader::LinearGradient(lg),
            Texture::RadialGradient(rg) => paint.shader = Shader::RadialGradient(rg),
        }
        paint
    }

    pub fn solid(color: Color) -> Self {
        let mut ink = Ink::default();
        ink.set_color(color);
        ink
    }

    pub fn texture(texture: Texture) -> Self {
        let mut ink = Ink::default();
        ink.texture = texture;
        ink
    }
}

impl Default for Ink {
    fn default() -> Self {
        Ink {
            texture: Texture::SolidColor(Color::BLACK),
            blend_mode: BlendMode::default(),
            anti_alias: true,
            force_hq_pipeline: false,
        }
    }
}

pub enum DrawCmd {
    Fill {
        trail: Trail,
        ink: Ink,
        fill_rule: FillRule,
        transform: Transform,
    },
    Stroke {
        trail: Trail,
        ink: Ink,
        stroke: Stroke,
        transform: Transform,
    },
    FillRect {
        rect: Rect,
        ink: Ink,
        transform: Transform,
    },
    Clear {
        color: Color,
    },
}

impl DrawCmd {
    fn eval(&self, canvas: &mut Pixmap, scale: f32) {
        match self {
            DrawCmd::Fill {
                trail,
                ink,
                fill_rule,
                transform,
            } => {
                let mut transform = *transform;
                transform.tx = scale * transform.tx;
                transform.ty = scale * transform.ty;
                transform = transform.pre_scale(scale, scale);
                canvas.fill_path(
                    &trail.to_path(),
                    &ink.clone().to_paint(),
                    *fill_rule,
                    transform,
                    None,
                );
            }
            DrawCmd::Stroke {
                trail,
                ink,
                stroke,
                transform,
            } => {
                let mut transform = *transform;
                transform.tx = scale * transform.tx;
                transform.ty = scale * transform.ty;
                transform = transform.pre_scale(scale, scale);
                canvas.stroke_path(
                    &trail.to_path(),
                    &ink.clone().to_paint(),
                    stroke,
                    transform,
                    None,
                );
            }
            DrawCmd::FillRect {
                rect,
                ink,
                transform,
            } => {
                let mut transform = *transform;
                transform.tx = scale * transform.tx;
                transform.ty = scale * transform.ty;
                transform = transform.pre_scale(scale, scale);
                canvas.fill_rect(*rect, &ink.clone().to_paint(), transform, None);
            }
            DrawCmd::Clear { color } => {
                canvas.fill(*color);
            }
        }
    }
}
