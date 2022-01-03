use crate::kolor::rgb;
use crate::{
    prelude::paint_shader,
    quiet::{noise2d, NoiseOpts},
};
use noise::{Fbm, Perlin};
use tiny_skia::{BlendMode, Paint, Pattern, Pixmap, Rect, Transform, SpreadMode, FilterQuality};

/// Add film grain to a canvas. 'scale' = 0.1 and 'factor' = 0.1 are good
/// defaults.
// pub fn grain(canvas: &mut Pixmap, scale: f32, factor: f32) {
//     let noise_opts = NoiseOpts::default().scales(scale).factor(factor);
//     let nf = Fbm::<Perlin>::default();
//     for i in 0..canvas.width() {
//         for j in 0..canvas.height() {
//             let n = noise2d(&nf, &noise_opts, i as f32, j as f32);
//             let n1 = (n + 1.0) / 2.0;
//             let n2 = n1.clamp(0.0, 1.0);
//             let c = rgb(n2, n2, n2);
//             let mut paint = Paint::default();
//             paint.set_color(c);
//             paint.blend_mode = BlendMode::Overlay;
//             let rect = Rect::from_xywh(i as f32, j as f32, 1.0, 1.0).unwrap();
//             canvas
//                 .fill_rect(rect, &paint, Transform::identity(), None)
//                 .unwrap()
//         }
//     }
// }

pub struct Grain(Pixmap);

impl Grain {
    pub fn new(width: u32, height: u32, scale: f32, factor: f32) -> Self {
        let mut pixmap = Pixmap::new(width, height).unwrap();
        let noise_opts = NoiseOpts::default().scales(scale).factor(factor);
        let nf = Fbm::<Perlin>::default();
        for i in 0..width {
            for j in 0..height {
                let n = noise2d(&nf, &noise_opts, i as f32, j as f32);
                let n1 = (n + 1.0) / 2.0;
                let n2 = n1.clamp(0.0, 1.0);
                let c = rgb(n2, n2, n2);
                let mut paint = Paint::default();
                paint.set_color(c);
                paint.blend_mode = BlendMode::Overlay;
                let rect = Rect::from_xywh(i as f32, j as f32, 1.0, 1.0).unwrap();
                pixmap
                    .fill_rect(rect, &paint, Transform::identity(), None)
                    .unwrap()
            }
        }
        Grain(pixmap)
    }
    pub fn grain<'a>(&'a self) -> Paint<'a> {
        let pattern = Pattern::new(
            self.0.as_ref(),
            SpreadMode::Repeat,
            FilterQuality::Bicubic,
            1.0,
            Transform::identity(),
        );
        paint_shader(pattern)
    }
}
