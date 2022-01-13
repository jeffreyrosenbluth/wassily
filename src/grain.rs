use crate::canvas::{paint_shader, Canvas};
use crate::kolor::rgb;
use crate::quiet::{noise2d, NoiseOpts};
use noise::{Fbm, Perlin};
use tiny_skia::{BlendMode, FilterQuality, Paint, Pattern, SpreadMode, Transform};

pub struct Grain(Canvas);

impl Grain {
    pub fn new(width: u32, height: u32, scale: f32, factor: f32) -> Self {
        let mut canvas = Canvas::new(width, height);
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
                canvas.fill_rect(i as f32, j as f32, 1.0, 1.0, &paint);
            }
        }
        Grain(canvas)
    }
    pub fn grain(&self) -> Paint {
        let pattern = Pattern::new(
            self.0.as_ref(),
            SpreadMode::Repeat,
            FilterQuality::Bicubic,
            1.0,
            Transform::identity(),
        );
        let mut p = paint_shader(pattern);
        p.blend_mode = BlendMode::Overlay;
        p
    }
}
