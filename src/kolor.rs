use crate::base::RGBA;
use color_thief::{get_palette, ColorFormat};
use image::GenericImageView;
use palette::{
    rgb::{Rgb, Rgba},
    white_point::D65,
    Alpha, ConvertInto, Lab, Laba, Lcha, Srgb, Srgba,
};
use rand::prelude::*;
use rand_pcg::Pcg64;
use std::path::Path;

pub fn black(alpha: f32) -> RGBA {
    RGBA::new(0.0, 0.0, 0.0, alpha)
}

pub fn white(alpha: f32) -> RGBA {
    RGBA::new(1.0, 1.0, 1.0, alpha)
}

pub fn red(alpha: f32) -> RGBA {
    RGBA::new(1.0, 0.0, 0.0, alpha)
}

pub fn green(alpha: f32) -> RGBA {
    RGBA::new(0.0, 1.0, 0.0, alpha)
}

pub fn blue(alpha: f32) -> RGBA {
    RGBA::new(0.0, 0.0, 1.0, alpha)
}

/// Convert a 'RGBA' to a palette Lcha.
pub fn lcha(c: RGBA) -> Lcha<D65> {
    let r = c.r;
    let g = c.g;
    let b = c.b;
    let a = c.a;
    let srgb: Alpha<Rgb, f32> = Rgba::new(r, g, b, a);
    srgb.into()
}

/// Each color channel's (red, green, and blue) value is a function of some
/// angle (theta). c(theta) = a + b * cos(freq * theta + phase).
pub struct CosChannel {
    pub a: f32,
    pub b: f32,
    pub freq: f32,  // radians
    pub phase: f32, // radians
}

impl CosChannel {
    pub fn new(phase: f32) -> Self {
        let a = 0.5;
        let b = 0.5;
        let freq = 1.0;
        Self { a, b, freq, phase }
    }
}

pub fn cos_color(r: CosChannel, g: CosChannel, b: CosChannel, theta: f32) -> RGBA {
    let red = r.a + r.b * (r.freq * theta + r.phase).cos();
    let green = g.a + g.b * (g.freq * theta + g.phase).cos();
    let blue = b.a + b.b * (b.freq * theta + b.phase).cos();
    RGBA::new(
        red.clamp(0.0, 1.0),
        green.clamp(0.0, 1.0),
        blue.clamp(0.0, 1.0),
        1.0,
    )
}

pub struct Palette {
    pub colors: Vec<RGBA>,
    rng: Pcg64,
}

impl Palette {
    /// Generate a palatte from a vector of 'RGBA's
    pub fn new(colors: Vec<RGBA>) -> Self {
        let rng = Pcg64::seed_from_u64(0);
        Palette { colors, rng }
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.rng = Pcg64::seed_from_u64(seed);
    }

    /// Generate a palatte from the colors in an image and sort them by
    /// there euclidean distance from Black;
    pub fn with_img(path: &Path, n: usize) -> Self {
        let img = image::open(path).expect("Could not find image file");
        let mut cs = vec![];
        let w = img.width();
        let h = img.height();
        let delta = (w as f32 * h as f32 / n as f32).sqrt();
        let mut x = 0.0;
        let mut y = 0.0;
        while x <= w as f32 {
            while y <= h as f32 {
                let p = img.get_pixel(x as u32, y as u32);
                let r = p.0[0] as f32 / 255.0;
                let g = p.0[1] as f32 / 255.0;
                let b = p.0[2] as f32 / 255.0;
                let a = p.0[3] as f32 / 255.0;
                let c = RGBA::new(r, g, b, a);
                cs.push(c);
                y += delta;
            }
            x += delta;
            y = 0.0;
        }
        cs.truncate(n);
        cs.sort_by_cached_key(|c| (1000.0 * (c.r * c.r + c.g * c.g + c.b * c.b)) as u32);
        Self::new(cs)
    }

    pub fn steal(path: &Path, max_colors: u8) -> Self {
        fn find_color(t: image::ColorType) -> ColorFormat {
            match t {
                image::ColorType::Rgb8 => ColorFormat::Rgb,
                image::ColorType::Rgba8 => ColorFormat::Rgba,
                _ => unreachable!(),
            }
        }
        let img = image::open(path).expect("Could not find image file");
        let color_type = find_color(img.color());
        let palette = get_palette(img.as_bytes(), color_type, 10, max_colors).unwrap();
        let palette = palette.into_iter().map(|c| {
            RGBA::new(
                c.r as f32 / 255.0,
                c.g as f32 / 255.0,
                c.b as f32 / 255.0,
                1.0,
            )
        });
        Self::new(palette.collect())
    }

    pub fn sort_by_hue(&mut self) {
        self.colors.sort_by_cached_key(|c| {
            let lcha = lcha(*c);
            (1000.0 * lcha.hue.to_radians()) as u32
        })
    }

    pub fn sort_by_chroma(&mut self) {
        self.colors.sort_by_cached_key(|c| {
            let lcha = lcha(*c);
            (1000.0 * lcha.chroma) as u32
        })
    }

    pub fn sort_by_lightness(&mut self) {
        self.colors.sort_by_cached_key(|c| {
            let lcha = lcha(*c);
            (1000.0 * lcha.l) as u32
        })
    }

    pub fn sort_by_alpha(&mut self) {
        self.colors.sort_by_cached_key(|c| {
            let lcha = lcha(*c);
            (1000.0 * lcha.alpha) as u32
        })
    }

    pub fn rand_color(&mut self) -> RGBA {
        self.colors[self.rng.gen_range(0..self.colors.len())]
    }

    pub fn rand_lab(&mut self) -> RGBA {
        let l: f32 = self.rng.gen_range(0.0..100.0);
        let a: f32 = self.rng.gen_range(-128.0..127.0);
        let b: f32 = self.rng.gen_range(-128.0..127.0);
        let rgb: Srgb = Lab::new(l, a, b).convert_into();
        let c = rgb.into_components();
        RGBA::new(c.0, c.1, c.2, 1.0)
    }

    pub fn rand_laba(&mut self) -> RGBA {
        let l: f32 = self.rng.gen_range(0.0..100.0);
        let a: f32 = self.rng.gen_range(-128.0..127.0);
        let b: f32 = self.rng.gen_range(-128.0..127.0);
        let o: f32 = self.rng.gen_range(0.0..1.0);
        let rgba: Srgba = Laba::new(l, a, b, o).convert_into();
        let c = rgba.into_components();
        RGBA::new(c.0, c.1, c.2, c.3)
    }
}
