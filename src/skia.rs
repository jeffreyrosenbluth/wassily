use image::imageops::rotate180;
use image::{ImageFormat, RgbaImage};
use tiny_skia::*;

pub fn save_jpg<P: AsRef<std::path::Path>>(pixmap: &Pixmap, path: P) {
    let img = pixmap_to_image(pixmap);
    img.save_with_format(path, ImageFormat::Jpeg)
        .expect("Error writing jpeg");
}

pub fn save_tiff<P: AsRef<std::path::Path>>(pixmap: &Pixmap, path: P) {
    let img = pixmap_to_image(pixmap);
    img.save_with_format(path, ImageFormat::Tiff)
        .expect("Error writing tiff");
}

pub fn image_to_pixmap(ib: RgbaImage) -> Pixmap {
    let ib = rotate180(&ib);
    let w = ib.width();
    let h = ib.height();
    let data = ib.into_vec();
    let pixmap = PixmapRef::from_bytes(&data, w, h).unwrap();
    pixmap.to_owned()
}

pub fn pixmap_to_image(pixmap: &Pixmap) -> RgbaImage {
    let w = pixmap.width();
    let h = pixmap.height();
    let data = pixmap.data().to_vec();
    rotate180(&RgbaImage::from_vec(w, h, data).unwrap())
}

pub fn paint_solid<'a>(color: Color) -> Paint<'a> {
    let mut paint = Paint::default();
    paint.anti_alias = true;
    paint.set_color(color);
    paint
}

pub fn paint_shader<'a>(shader: Shader<'a>) -> Paint<'a> {
    let mut paint = Paint::default();
    paint.anti_alias = true;
    paint.shader = shader;
    paint
}

pub fn fill_path(pixmap: &mut Pixmap, path: &Path, paint: &Paint) {
    pixmap.fill_path(path, paint, FillRule::Winding, Transform::identity(), None);
}

pub fn fill_rect(pixmap: &mut Pixmap, x: f32, y: f32, w: f32, h: f32, paint: &Paint) {
    let rect = Rect::from_xywh(x, y, w, h).unwrap();
    pixmap.fill_rect(rect, paint, Transform::identity(), None);
}

pub fn pixel(pixmap: &mut Pixmap, x: f32, y: f32, color: Color) {
    let width = pixmap.width();
    let pixel_map = pixmap.pixels_mut();
    let k = y as usize * width as usize + x as usize;
    pixel_map[k] = color.premultiply().to_color_u8();
}

pub fn stroke_path(pixmap: &mut Pixmap, path: &Path, weight: f32, paint: &Paint) {
    let mut stroke = Stroke::default();
    stroke.width = weight;
    pixmap.stroke_path(path, paint, &stroke, Transform::identity(), None);
}