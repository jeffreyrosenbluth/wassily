use image::imageops::rotate180;
use image::{ImageFormat, RgbaImage, RgbImage};
use std::ops::{Deref, DerefMut};
use tiny_skia::*;

pub struct Canvas(pub Pixmap);

impl Deref for Canvas {
    type Target = Pixmap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Canvas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&RgbaImage> for Canvas {
    fn from(ib: &RgbaImage) -> Self {
        // let ib = rotate180(ib);
        let w = ib.width();
        let h = ib.height();
        let data = ib.clone().into_vec();
        let pixmap = PixmapRef::from_bytes(&data, w, h).unwrap();
        Canvas(pixmap.to_owned())
    }
}

impl From<&RgbImage> for Canvas {
    fn from(ib: &RgbImage) -> Self {
        let w = ib.width();
        let h = ib.height();
        let mut data4: Vec<u8>  = Vec::new();
        let data = ib.clone().into_vec();
        for d in data.chunks(3) {
            data4.extend(d);
            data4.push(255)
        }
        let pixmap = PixmapRef::from_bytes(&data4, w, h).unwrap();
        Canvas(pixmap.to_owned())
    }
}

impl From<RgbaImage> for Canvas {
    fn from(ib: RgbaImage) -> Self {
        let ib = rotate180(&ib);
        let w = ib.width();
        let h = ib.height();
        let mut data4: Vec<u8>  = Vec::new();
        let data = ib.into_vec();
        for d in data.chunks(3) {
            data4.extend(d);
            data4.push(255)
        }
        let pixmap = PixmapRef::from_bytes(&data4, w, h).unwrap();
        Canvas(pixmap.to_owned())
    }
}

impl From<RgbImage> for Canvas {
    fn from(ib: RgbImage) -> Self {
        let ib = rotate180(&ib);
        let w = ib.width();
        let h = ib.height();
        let data = ib.into_vec();
        let pixmap = PixmapRef::from_bytes(&data, w, h).unwrap();
        Canvas(pixmap.to_owned())
    }
}

impl From<Canvas> for RgbaImage {
    fn from(canvas: Canvas) -> Self {
        let w = canvas.width();
        let h = canvas.height();
        let data = canvas.data().to_vec();
        rotate180(&RgbaImage::from_vec(w, h, data).unwrap())
    }
}

impl From<&Canvas> for RgbaImage {
    fn from(canvas: &Canvas) -> Self {
        let w = canvas.width();
        let h = canvas.height();
        let data = canvas.data().to_vec();
        rotate180(&RgbaImage::from_vec(w, h, data).unwrap())
    }
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas(Pixmap::new(width, height).unwrap())
    }

    pub fn fill_path(&mut self, path: &Path, paint: &Paint) {
        self.0
            .fill_path(path, paint, FillRule::Winding, Transform::identity(), None);
    }
    pub fn fill_rect(&mut self, x: f32, y: f32, w: f32, h: f32, paint: &Paint) {
        let rect = Rect::from_xywh(x, y, w, h).unwrap();
        self.0.fill_rect(rect, paint, Transform::identity(), None);
    }

    pub fn stroke_path(&mut self, path: &Path, weight: f32, paint: &Paint) {
        let stroke = Stroke {width: weight, ..Default::default()};
        self.0
            .stroke_path(path, paint, &stroke, Transform::identity(), None);
    }

    pub fn dot(&mut self, x: f32, y: f32, color: Color) {
        let width = self.width();
        let pixel_map = self.pixels_mut();
        let k = y as usize * width as usize + x as usize;
        pixel_map[k] = color.premultiply().to_color_u8();
    }

    pub fn save_png<P: AsRef<std::path::Path>>(&self, path: P) {
        self.0.save_png(path).expect("Error writing png");
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
        self.0.width() as f32
    }

    pub fn h_f32(&self) -> f32 {
        self.0.height() as f32
    }

    pub fn w_usize(&self) -> usize {
        self.0.width() as usize
    }

    pub fn h_usize(&self) -> usize {
        self.0.height() as usize
    }
}

pub fn paint_solid<'a>(color: Color) -> Paint<'a> {
    let mut paint = Paint {anti_alias: true, ..Default::default() };
    paint.set_color(color);
    paint
}

pub fn paint_shader<'a>(shader: Shader<'a>) -> Paint<'a> {
    Paint {anti_alias: true, shader, ..Default::default() }
}
