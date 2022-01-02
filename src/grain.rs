use crate::quiet::{noise2d, NoiseOpts};
use noise::{Fbm, Perlin};
use tiny_skia::{Pixmap, Paint, BlendMode, Rect, Transform};
use crate::kolor::rgb;

/// Add film grain to a canvas. 'scale' = 0.1 and 'factor' = 0.1 are good
/// defaults.
pub fn grain(canvas: &mut Pixmap, scale: f32, factor: f32) {
    let noise_opts = NoiseOpts::default().scales(scale).factor(factor);
    let nf = Fbm::<Perlin>::default();
    for i in 0..canvas.width() {
        for j in 0..canvas.height() {
            let n = noise2d(&nf, &noise_opts, i as f32, j as f32);
            let n1 = (n + 1.0) / 2.0;
            let c = rgb(n1, n1, n1);
            let mut paint = Paint::default();
            paint.set_color(c);
            paint.blend_mode = BlendMode::Overlay;
            let rect = Rect::from_xywh(i as f32, j as f32, 1.0, 1.0).unwrap();
            canvas.fill_rect(rect, &paint, Transform::identity(), None).unwrap()
        }
    }
}