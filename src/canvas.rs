use image::{ImageFormat, RgbImage, RgbaImage};
use tiny_skia::*;

pub struct Canvas {
    pub pixmap: Pixmap,
    pub width: u32,
    pub height: u32,
    pub scale: f32,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas {
            pixmap: Pixmap::new(width, height).unwrap(),
            width,
            height,
            scale: 1.0,
        }
    }

    pub fn with_scale(width: u32, height: u32, scale: f32) -> Canvas {
        let w = scale * width as f32;
        let h = scale * height as f32;
        Canvas {
            pixmap: Pixmap::new(w as u32, h as u32).unwrap(),
            width,
            height,
            scale,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn fill(&mut self, color: Color) {
        self.pixmap.fill(color);
    }

    pub fn fill_path(
        &mut self,
        path: &Path,
        paint: &mut Paint,
        fill_rule: FillRule,
        transform: Transform,
        clip_mask: Option<&ClipMask>,
    ) {
        let mut transform = transform;
        transform = transform.post_scale(self.scale, self.scale);
        self.pixmap
            .fill_path(&path, paint, fill_rule, transform, clip_mask);
    }

    pub fn fill_rect(
        &mut self,
        rect: Rect,
        paint: &mut Paint,
        transform: Transform,
        clip_mask: Option<&ClipMask>,
    ) {
        let mut transform = transform;
        transform = transform.post_scale(self.scale, self.scale);
        self.pixmap.fill_rect(rect, paint, transform, clip_mask);
    }

    pub fn stroke_path(
        &mut self,
        path: &Path,
        paint: &mut Paint,
        stroke: &Stroke,
        transform: Transform,
        clip_mask: Option<&ClipMask>,
    ) {
        let mut transform = transform;
        transform = transform.post_scale(self.scale, self.scale);
        self.pixmap
            .stroke_path(&path, paint, &stroke, transform, clip_mask);
    }

    pub fn dot(&mut self, x: f32, y: f32, color: Color) {
        let width = self.pixmap.width();
        let pixel_map = self.pixmap.pixels_mut();
        let k = y as usize * width as usize + x as usize;
        pixel_map[k] = color.premultiply().to_color_u8();
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

    pub fn w_f32(&self) -> f32 {
        self.width as f32
    }

    pub fn h_f32(&self) -> f32 {
        self.height as f32
    }

    pub fn w_usize(&self) -> usize {
        self.width as usize
    }

    pub fn h_usize(&self) -> usize {
        self.height as usize
    }
}

pub fn paint_solid<'a>(color: Color) -> Paint<'a> {
    let mut paint = Paint {
        anti_alias: true,
        ..Default::default()
    };
    paint.set_color(color);
    paint
}

pub fn paint_shader<'a>(shader: Shader<'a>) -> Paint<'a> {
    Paint {
        anti_alias: true,
        shader,
        ..Default::default()
    }
}

impl From<&RgbaImage> for Canvas {
    fn from(ib: &RgbaImage) -> Self {
        let width = ib.width();
        let height = ib.height();
        let data = ib.clone().into_vec();
        let pixmap = PixmapRef::from_bytes(&data, width, height).unwrap();
        Canvas {
            pixmap: pixmap.to_owned(),
            width,
            height,
            scale: 1.0,
        }
    }
}

impl From<&RgbImage> for Canvas {
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
        Canvas {
            pixmap: pixmap.to_owned(),
            width,
            height,
            scale: 1.0,
        }
    }
}

impl From<RgbaImage> for Canvas {
    fn from(ib: RgbaImage) -> Self {
        let width = ib.width();
        let height = ib.height();
        let data = ib.into_vec();
        let pixmap = PixmapRef::from_bytes(&data, width, height).unwrap();
        Canvas {
            pixmap: pixmap.to_owned(),
            width,
            height,
            scale: 1.0,
        }
    }
}

impl From<RgbImage> for Canvas {
    fn from(ib: RgbImage) -> Self {
        let width = ib.width();
        let height = ib.height();
        let data = ib.into_vec();
        let pixmap = PixmapRef::from_bytes(&data, width, height).unwrap();
        Canvas {
            pixmap: pixmap.to_owned(),
            width,
            height,
            scale: 1.0,
        }
    }
}

impl From<Canvas> for RgbaImage {
    fn from(canvas: Canvas) -> Self {
        let w = canvas.width();
        let h = canvas.height();
        let data = canvas.pixmap.data().to_vec();
        RgbaImage::from_vec(w, h, data).unwrap()
    }
}

impl From<&Canvas> for RgbaImage {
    fn from(canvas: &Canvas) -> Self {
        let w = canvas.width();
        let h = canvas.height();
        let data = canvas.pixmap.data().to_vec();
        RgbaImage::from_vec(w, h, data).unwrap()
    }
}
