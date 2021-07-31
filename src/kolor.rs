use crate::{base::RGBA, prelude::Point};
use color_thief::{get_palette, ColorFormat};
use image::{DynamicImage, GenericImageView};
use num_traits::AsPrimitive;
use palette::{
    rgb::{Rgb, Rgba},
    white_point::D65,
    Alpha, ConvertInto, Lab, LabHue, Laba, Lcha, Srgb, Srgba,
};
use rand::prelude::*;
use rand_distr::Normal;
use rand_pcg::Pcg64;
use std::{ops::Index, path::Path, usize};

pub struct Jiggle {
    rng: Pcg64,
    normal: Normal<f32>,
}

impl Jiggle {
    pub fn new(seed: u64, std_dev: f32) -> Self {
        let rng = Pcg64::seed_from_u64(seed);
        let normal = Normal::new(0.0, std_dev).unwrap();
        Self { rng, normal }
    }

    pub fn jiggle(&mut self, color: RGBA) -> RGBA {
        RGBA::rgba(
            (color.r + self.normal.sample(&mut self.rng)).clamp(0.0, 1.0),
            (color.g + self.normal.sample(&mut self.rng)).clamp(0.0, 1.0),
            (color.b + self.normal.sample(&mut self.rng)).clamp(0.0, 1.0),
            color.a,
        )
    }
}

impl RGBA {
    pub fn black(alpha: f32) -> Self {
        Self::rgba(0.0, 0.0, 0.0, alpha)
    }

    pub fn white(alpha: f32) -> Self {
        Self::rgba(1.0, 1.0, 1.0, alpha)
    }

    pub fn red(alpha: f32) -> Self {
        Self::rgba(1.0, 0.0, 0.0, alpha)
    }

    pub fn green(alpha: f32) -> Self {
        Self::rgba(0.0, 1.0, 0.0, alpha)
    }

    pub fn blue(alpha: f32) -> Self {
        Self::rgba(0.0, 0.0, 1.0, alpha)
    }

    /// Convert a 'RGBA' to a palette Lcha.
    pub fn lcha(self) -> Lcha<D65> {
        let r = self.r;
        let g = self.g;
        let b = self.b;
        let a = self.a;
        let srgb: Alpha<Rgb, f32> = Rgba::new(r, g, b, a);
        srgb.into()
    }
}

/// Each color channel's (red, green, and blue) value is a function of some
/// angle (theta). c(theta) = a + b * cos(freq * theta + phase).
#[derive(Debug, Clone, Copy)]
pub struct CosChannel {
    pub a: f32,
    pub b: f32,
    pub freq: f32,  // radians
    pub phase: f32, // radians
}

impl CosChannel {
    pub fn new(a: f32, b: f32, phase: f32) -> Self {
        let freq = 1.0;
        Self { a, b, freq, phase }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CosColor {
    r: CosChannel,
    g: CosChannel,
    b: CosChannel,
}

impl CosColor {
    pub fn new(r: CosChannel, g: CosChannel, b: CosChannel) -> Self {
        Self { r, g, b }
    }

    pub fn cos_color(&self, theta: f32) -> RGBA {
        let r = self.r;
        let g = self.g;
        let b = self.b;
        let red = r.a + r.b * (r.freq * theta + r.phase).cos();
        let green = g.a + g.b * (g.freq * theta + g.phase).cos();
        let blue = b.a + b.b * (b.freq * theta + b.phase).cos();
        RGBA::rgba(
            red.clamp(0.0, 1.0),
            green.clamp(0.0, 1.0),
            blue.clamp(0.0, 1.0),
            1.0,
        )
    }
}

#[derive(Clone, Debug)]
pub struct Palette {
    pub colors: Vec<RGBA>,
    rng: Pcg64,
    pub current: usize,
}

impl Default for Palette {
    fn default() -> Self {
        Palette::new(vec![])
    }
}

impl Palette {
    /// Generate a palatte from a vector of 'RGBA's
    pub fn new(colors: Vec<RGBA>) -> Self {
        let rng = Pcg64::seed_from_u64(0);
        let mut colors = colors;
        colors.sort_by_cached_key(|c| (1000.0 * (c.r * c.r + c.g * c.g + c.b * c.b)) as u32);
        colors.dedup_by_key(|c| c.as_8());
        Palette {
            colors,
            rng,
            current: 0,
        }
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.rng = Pcg64::seed_from_u64(seed);
    }

    pub fn set_index(&mut self, i: usize) {
        self.current = i % self.colors.len();
    }

    /// Generate a palatte from the colors in an image and sort them by
    /// there euclidean distance from Black;
    pub fn with_img<T: AsRef<Path>>(path: T, n: usize) -> Self {
        let img = image::open(path).expect("Could not find image file");
        let mut cs = vec![];
        let w = img.width();
        let h = img.height();
        let delta = (w as f32 * h as f32 / n as f32).sqrt();
        let mut x = 0.0;
        let mut y = 0.0;
        while x < w as f32 {
            while y < h as f32 {
                let p = img.get_pixel(x as u32, y as u32);
                let r = p.0[0] as f32 / 255.0;
                let g = p.0[1] as f32 / 255.0;
                let b = p.0[2] as f32 / 255.0;
                let a = p.0[3] as f32 / 255.0;
                let c = RGBA::rgba(r, g, b, a);
                cs.push(c);
                y += delta;
            }
            x += delta;
            y = 0.0;
        }
        cs.truncate(n);
        cs.sort_by_cached_key(|c| (1000.0 * (c.r * c.r + c.g * c.g + c.b * c.b)) as u32);
        cs.dedup_by_key(|c| c.as_8());
        Self::new(cs)
    }

    pub fn steal<T: AsRef<Path>>(path: T, max_colors: u8) -> Self {
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
            RGBA::rgba(
                c.r as f32 / 255.0,
                c.g as f32 / 255.0,
                c.b as f32 / 255.0,
                1.0,
            )
        });
        Self::new(palette.collect())
    }

    pub fn rotate_hue(&mut self, degrees: f32) {
        self.colors = self
            .colors
            .iter()
            .map(|c| {
                let mut l = c.lcha();
                let hue = (l.hue.to_degrees() + degrees) % 360.0;
                l.hue = LabHue::from_degrees(hue);
                let rgba: Srgba = l.convert_into();
                let c = rgba.into_components();
                RGBA::rgba(c.0, c.1, c.2, c.3)
            })
            .collect();
    }

    pub fn sort_by_hue(&mut self) {
        self.colors.sort_by_cached_key(|c| {
            let lcha = c.lcha();
            (1000.0 * lcha.hue.to_radians()) as u32
        })
    }

    pub fn sort_by_chroma(&mut self) {
        self.colors.sort_by_cached_key(|c| {
            let lcha = c.lcha();
            (1000.0 * lcha.chroma) as u32
        })
    }

    pub fn sort_by_lightness(&mut self) {
        self.colors.sort_by_cached_key(|c| {
            let lcha = c.lcha();
            (1000.0 * lcha.l) as u32
        })
    }

    pub fn sort_by_alpha(&mut self) {
        self.colors.sort_by_cached_key(|c| {
            let lcha = c.lcha();
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
        RGBA::rgba(c.0, c.1, c.2, 1.0)
    }

    pub fn rand_laba(&mut self) -> RGBA {
        let l: f32 = self.rng.gen_range(0.0..100.0);
        let a: f32 = self.rng.gen_range(-128.0..127.0);
        let b: f32 = self.rng.gen_range(-128.0..127.0);
        let o: f32 = self.rng.gen_range(0.0..1.0);
        let rgba: Srgba = Laba::new(l, a, b, o).convert_into();
        let c = rgba.into_components();
        RGBA::rgba(c.0, c.1, c.2, c.3)
    }

    pub fn jiggle(&mut self, seed: u64, std_dev: f32) {
        let mut j = Jiggle::new(seed, std_dev);
        let cs: Vec<RGBA> = self.colors.iter().map(|c| j.jiggle(*c)).collect();
        self.colors = cs;
    }
}

impl Index<usize> for Palette {
    type Output = RGBA;

    fn index(&self, index: usize) -> &Self::Output {
        &self.colors[index]
    }
}

impl IntoIterator for Palette {
    type Item = RGBA;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.colors.into_iter()
    }
}

pub fn get_color<T: AsPrimitive<f32>>(img: &DynamicImage, width: T, height: T, p: Point) -> RGBA {
    let x = p.x * img.width() as f32 / width.as_();
    let y = p.y * img.height() as f32 / height.as_();
    let p = img.get_pixel(x as u32, y as u32);
    let r = p.0[0] as f32 / 255.0;
    let g = p.0[1] as f32 / 255.0;
    let b = p.0[2] as f32 / 255.0;
    let a = p.0[3] as f32 / 255.0;
    RGBA::rgba(r, g, b, a)
}
