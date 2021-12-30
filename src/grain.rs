use crate::base::{BlendMode, Sketch, Texture, RGBA};
use crate::quiet::{noise2d, NoiseOpts};
use noise::{Fbm, Perlin};

/// Add film grain to a canvas. 'scale' = 0.1 and 'factor' = 0.1 are good
/// defaults.
pub fn grain<S: Sketch>(canvas: &mut S, scale: f32, factor: f32) {
    let noise_opts = NoiseOpts::default().scales(scale).factor(factor);
    let nf = Fbm::<Perlin>::default();
    for i in 0..canvas.width() {
        for j in 0..canvas.height() {
            let n = noise2d(&nf, &noise_opts, i as f32, j as f32);
            let n1 = (n + 1.0) / 2.0 * 255.0;
            let c = RGBA::gray(n1 as u8);
            let mut texture = Texture::solid_color(c);
            texture.mode(BlendMode::Overlay);
            canvas.fill_rect(i as f32, j as f32, 1.0, 1.0, &texture);
        }
    }
}