//! # Canvas
//! A context for drawing.

use crate::prelude::pt;
use image::{ImageFormat, RgbImage, RgbaImage};
use tiny_skia::*;

/// A `Canvas` is an abstraction over a tiny-skia `Pixmap` that allows for drawing at different
/// scales. This allows for using the same drawing commands to produce images of different sizes.
///
/// ## Example
///
/// ```no_run
/// use wassily::prelude::*;
///
/// fn draw(canvas: &mut Canvas) {
///     canvas.fill(*GRAY);
///     let center = pt(canvas.width / 2, canvas.height / 2);
///     ShapeBuilder::new()
///         .ellipse(center, 300.0, 200.0)
///         .fill_color(*INDIGO)
///         .stroke_color(*ORANGE)
///         .stroke_weight(10.0)
///         .build()
///         .draw(canvas);
/// }
///
/// fn main() {
///    let mut canvas1 = Canvas::new(500, 500);
///     // Draw a 300x200 ellipse at the center of a 500 x 500 canvas.
///     draw(&mut canvas1);
///
///     let mut canvas2 = Canvas::with_scale(500, 500, 2.0);
///     // Draw a 600x400 ellipse at the center of a 1000 x 1000 canvas. That is,
///     // the exact same image as above, but, double the width and height.
///     draw(&mut canvas2);
///
///     canvas1.save_png("./scale1.png");
///     canvas2.save_png("./scale2.png");
/// }
/// ```
pub struct Canvas {
    /// The underlying tiny-skia `Pixmap`.
    pub pixmap: Pixmap,
    // The width of the unscaled map, in pixels. You should use this with in any drawing commands
    // so that it will be scaled correctly at differing scales.
    width: u32,
    // The height of the unscaled map, in pixels.
    height: u32,
    /// The scale factor of the canvas. This is the factor by which the width and height are
    /// scaled. For example, a scale of 2.0 means that the width and height are doubled when
    /// rendering.
    pub scale: f32,
}

impl Canvas {
    /// Create a new unscaled `Canvas`.
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas {
            pixmap: Pixmap::new(width, height).unwrap(),
            width,
            height,
            scale: 1.0,
        }
    }

    /// Create a new `Canvas` with a scale factor. The actual size of the canvas is the width times
    /// the scale and the height times the scale. However, all drawing commands should use the
    /// pre-scaled `width` and `height`.
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

    // The width of the unscaled map, in pixels. You should use this with in any drawing commands
    // so that it will be scaled correctly at differing scales.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The pre-scaled height of the canvas in pixels as a u32.
    pub fn height(&self) -> u32 {
        self.height
    }

    ///The pre-scaled width of the canvas in pixels as a f32.
    pub fn w_f32(&self) -> f32 {
        self.width as f32
    }

    /// The pre-scaled height of the canvas in pixels as a f32.
    pub fn h_f32(&self) -> f32 {
        self.height as f32
    }

    /// The pre-scaled width of the canvas in pixels as a usize.
    pub fn w_usize(&self) -> usize {
        self.width as usize
    }

    /// The pre-scaled height of the canvas in pixels as a usize.
    pub fn h_usize(&self) -> usize {
        self.height as usize
    }

    /// Fill the entire canvas with a `Color`.
    pub fn fill(&mut self, color: Color) {
        self.pixmap.fill(color);
    }

    /// Fill the path with a `Paint`.
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
            .fill_path(path, paint, fill_rule, transform, clip_mask);
    }

    /// Fill a rectangle with a `Paint`.
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

    /// Stroke a path with a `Paint`.
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
            .stroke_path(path, paint, stroke, transform, clip_mask);
    }

    /// Color a single pixel on the canvas. !!! Warning this function will not scale with the
    /// canvas.
    pub fn dot(&mut self, x: f32, y: f32, color: Color) {
        let width = self.pixmap.width();
        let pixel_map = self.pixmap.pixels_mut();
        let k = y as usize * width as usize + x as usize;
        pixel_map[k] = color.premultiply().to_color_u8();
    }

    /// Save the `Canvas` as a png.
    pub fn save_png<P: AsRef<std::path::Path>>(&self, path: P) {
        self.pixmap.save_png(path).expect("Error writing png");
    }

    /// Save the `Canvas` as a jpg.
    pub fn save_jpg<P: AsRef<std::path::Path>>(&self, path: P) {
        let img: RgbaImage = self.into();
        img.save_with_format(path, ImageFormat::Jpeg)
            .expect("Error writing jpeg");
    }

    /// Save the `Canvas` as a tiff.
    pub fn save_tiff<P: AsRef<std::path::Path>>(&self, path: P) {
        let img: RgbaImage = self.into();
        img.save_with_format(path, ImageFormat::Tiff)
            .expect("Error writing tiff");
    }
}

/// Create a tiny-skia `Paint` from a solid `Color`.
pub fn paint_solid<'a>(color: Color) -> Paint<'a> {
    let mut paint = Paint {
        anti_alias: true,
        ..Default::default()
    };
    paint.set_color(color);
    paint
}

/// Create a tiny-skia `Paint` from a `Shader`.
pub fn paint_shader(shader: Shader) -> Paint {
    Paint {
        anti_alias: true,
        shader,
        ..Default::default()
    }
}

/// Create a linear gradient to draw a line.
pub fn paint_lg<'a>(x0: f32, y0: f32, x1: f32, y1: f32, stops: Vec<GradientStop>) -> Paint<'a> {
    let lg = LinearGradient::new(
        pt(x0, y0),
        pt(x1, y1),
        stops,
        SpreadMode::Pad,
        Transform::identity(),
    )
    .unwrap();
    paint_shader(lg)
}

fn image_to_canvas(width: u32, height: u32, data: Vec<u8>) -> Canvas {
    let pixmap = PixmapRef::from_bytes(&data, width, height).unwrap();
    Canvas {
        pixmap: pixmap.to_owned(),
        width,
        height,
        scale: 1.0,
    }
}

/// Convert a `&RgbaImage` to a `Canvas`.
impl From<&RgbaImage> for Canvas {
    fn from(ib: &RgbaImage) -> Self {
        image_to_canvas(ib.width(), ib.height(), ib.clone().into_vec())
    }
}

/// Convert a `&RgbImage` to a `Canvas`.
impl From<&RgbImage> for Canvas {
    fn from(ib: &RgbImage) -> Self {
        let mut data4: Vec<u8> = Vec::new();
        let data = ib.clone().into_vec();
        for d in data.chunks(3) {
            data4.extend(d);
            data4.push(255)
        }
        image_to_canvas(ib.width(), ib.height(), data4)
    }
}

/// Convert a `RgbaImage` to a `Canvas`.
impl From<RgbaImage> for Canvas {
    fn from(ib: RgbaImage) -> Self {
        image_to_canvas(ib.width(), ib.height(), ib.into_vec())
    }
}

/// Convert a `RgbImage` to a `Canvas`.
impl From<RgbImage> for Canvas {
    fn from(ib: RgbImage) -> Self {
        let mut data4: Vec<u8> = Vec::new();
        let data = ib.clone().into_vec();
        for d in data.chunks(3) {
            data4.extend(d);
            data4.push(255)
        }
        image_to_canvas(ib.width(), ib.height(), data4)
    }
}

/// Convert a `Canvas` to a `RgbaImage`.
impl From<Canvas> for RgbaImage {
    fn from(canvas: Canvas) -> Self {
        let data = canvas.pixmap.data().to_vec();
        RgbaImage::from_vec(canvas.width(), canvas.height(), data).unwrap()
    }
}

/// Convert a `&Canvas` to a `RgbaImage`.
impl From<&Canvas> for RgbaImage {
    fn from(canvas: &Canvas) -> Self {
        let data = canvas.pixmap.data().to_vec();
        RgbaImage::from_vec(canvas.width(), canvas.height(), data).unwrap()
    }
}
