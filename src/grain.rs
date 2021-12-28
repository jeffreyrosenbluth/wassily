use crate::base::{BlendMode, Sketch, Texture, RGBA};
use crate::quiet::{noise2d, NoiseOpts};
use noise::{Fbm, Perlin};

pub struct Grain {
    scale: f32,
    factor: f32,
    width: u32,
    height: u32,
}

impl Grain {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            scale: 0.1,
            factor: 0.1,
            width,
            height,
        }
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn set_factor(&mut self, factor: f32) {
        self.factor = factor;
    }

    pub fn apply<S: Sketch>(&self, canvas: &mut S) {
        let noise_opts = NoiseOpts::default().scales(self.scale).factor(self.factor);
        let nf = Fbm::<Perlin>::default();
        for i in 0..self.width {
            for j in 0..self.height {
                let n = noise2d(&nf, &noise_opts, i as f32, j as f32);
                let n1 = (n + 1.0) / 2.0 * 255.0;
                let c = RGBA::gray(n1 as u8);
                let mut texture = Texture::solid_color(c);
                texture.mode(BlendMode::Overlay);
                canvas.fill_rect(i as f32, j as f32, 1.0, 1.0, &texture);
            }
        }
    }
}
