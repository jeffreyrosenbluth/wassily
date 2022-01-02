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

pub fn pixel(pixmap: &mut Pixmap, x: f32, y: f32, color: Color) {
    let width = pixmap.width();
    let pixel_map = pixmap.pixels_mut();
    let k = y as usize * width as usize + x as usize;
    pixel_map[k] = color.premultiply().to_color_u8();
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