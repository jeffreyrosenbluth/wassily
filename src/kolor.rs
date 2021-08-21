//! Utilities to manage colors and palettes.

use crate::{base::RGBA, prelude::Point};
use color_thief::{get_palette, ColorFormat};
use image::{DynamicImage, GenericImageView};
use num_traits::AsPrimitive;
use palette::{
    rgb::{Rgb, Rgba},
    white_point::D65,
    Alpha, IntoColor, Lab, LabHue, Laba, Lcha, Srgb, Srgba,
};
use rand::prelude::*;
use rand_distr::Normal;
use rand_pcg::Pcg64;
use std::{ops::Index, ops::IndexMut, path::Path, usize};

/// Perturb a `RGBA` value.
pub struct Jiggle {
    rng: Pcg64,
    normal: Normal<f32>,
}

impl Jiggle {
    /// `std_dev` as percentage of color channel, 0.01 to 0.2 works well.
    /// Larger standard deviations will produce colors very far from the input
    /// color.
    pub fn new(seed: u64, std_dev: f32) -> Self {
        let rng = Pcg64::seed_from_u64(seed);
        let normal = Normal::new(0.0, std_dev).unwrap();
        Self { rng, normal }
    }

    /// Perturb the r, g, b channels of an `RGBA` color using a normal distribution.
    /// The value is clamped to [0, 1] and applied as a percentage.
    pub fn jiggle(&mut self, color: RGBA) -> RGBA {
        let (r, g, b, a) = color.as_f32s();
        RGBA::rgba(
            (r + self.normal.sample(&mut self.rng)).clamp(0.0, 1.0),
            (g + self.normal.sample(&mut self.rng)).clamp(0.0, 1.0),
            (b + self.normal.sample(&mut self.rng)).clamp(0.0, 1.0),
            a,
        )
    }
}

impl RGBA {
    /// Black with opacity alpha [0.0, 1.0].
    pub fn black(alpha: f32) -> Self {
        Self::rgba(0.0, 0.0, 0.0, alpha)
    }

    /// White with opacity alpha [0.0, 1.0].
    pub fn white(alpha: f32) -> Self {
        Self::rgba(1.0, 1.0, 1.0, alpha)
    }

    /// Red with opacity alpha [0.0, 1.0].
    pub fn red(alpha: f32) -> Self {
        Self::rgba(1.0, 0.0, 0.0, alpha)
    }

    /// Green with opacity alpha [0.0, 1.0].
    pub fn green(alpha: f32) -> Self {
        Self::rgba(0.0, 1.0, 0.0, alpha)
    }

    /// Blue with opacity alpha [0.0, 1.0].
    pub fn blue(alpha: f32) -> Self {
        Self::rgba(0.0, 0.0, 1.0, alpha)
    }

    /// Convert a `RGBA` to a [palette::Lcha].
    pub fn lcha(self) -> Lcha<D65> {
        let (r, g, b, a) = self.as_f32s();
        let srgb: Alpha<Rgb, f32> = Rgba::new(r, g, b, a);
        srgb.into_color()
    }

    pub fn spread(self) -> Self {
        let mut lcha = self.lcha();
        let l1 = lcha.l / 50.0 - 1.0;
        let l2 = l1.abs().sqrt() * l1.signum();
        let l3 = 50.0 * (l2 + 1.0);
        lcha.l = l3;
        let c: Srgba = lcha.into_color();
        c.into()
    }

    pub fn lerp(color1: &Self, color2: &Self, t: f32) -> Self {
        let s = t.clamp(0.0, 1.0);
        let c1 = color1.as_f32s();
        let c2 = color2.as_f32s();
        let r = c1.0 + s * (c2.0 - c1.0);
        let g = c1.1 + s * (c2.1 - c1.1);
        let b = c1.2 + s * (c2.2 - c1.2);
        let a = c1.3 + s * (c2.3 - c1.3);
        RGBA::rgba(r, g, b, a)
    }
}

impl From<image::Rgba<u8>> for RGBA {
    fn from(p: image::Rgba<u8>) -> Self {
        RGBA::rgba8(p.0[0], p.0[1], p.0[2], p.0[3])
    }
}

impl From<Srgba> for RGBA {
    fn from(rgb: Srgba) -> Self {
        let c = rgb.into_components();
        RGBA::rgba(c.0, c.1, c.2, c.3)
    }
}

impl From<Srgb> for RGBA {
    fn from(rgb: Srgb) -> Self {
        let c = rgb.into_components();
        RGBA::rgba(c.0, c.1, c.2, 1.0)
    }
}
/// A Palette of colors and functions to manage them.
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
        let colors = colors;
        Palette {
            colors,
            rng,
            current: 0,
        }
    }

    /// Set the seed of the random number generator used in all of the random
    /// color functions.
    pub fn set_seed(&mut self, seed: u64) {
        self.rng = Pcg64::seed_from_u64(seed);
    }

    /// The index of the color list.
    pub fn set_index(&mut self, i: usize) {
        self.current = i % self.colors.len();
    }

    /// Generate a palatte from the colors in an image. If `n` is None
    /// use each unique color in the image otherwise choose n colors.
    pub fn with_img<T: AsRef<Path>>(path: T, n: Option<usize>) -> Self {
        let img = image::open(path).expect("Could not find image file");
        let mut cs: Vec<RGBA> = vec![];
        let w = img.width();
        let h = img.height();
        if let Some(n) = n {
            let delta = (w as f32 * h as f32 / n as f32).sqrt();
            let mut x = 0.0;
            let mut y = 0.0;
            while x < w as f32 {
                while y < h as f32 {
                    let p = img.get_pixel(x as u32, y as u32);
                    cs.push(p.into());
                    y += delta;
                }
                x += delta;
                y = 0.0;
            }
            cs.truncate(n)
        } else {
            for (_, _, p) in img.pixels() {
                cs.push(p.into());
            }
            cs.sort_by_cached_key(|c| c.as_tuple());
            cs.dedup_by_key(|c| c.as_tuple());
        }
        Self::new(cs)
    }

    /// Create a palette of colors using the [color_thief] package.
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
        let palette = palette.into_iter().map(|c| RGBA::rgba8(c.r, c.g, c.b, 255));
        Self::new(palette.collect())
    }

    pub fn spread(&mut self) {
        self.colors = self.colors.iter().map(|c| c.spread()).collect();
    }

    /// Rotate the [palette::LabHue] of each color.
    pub fn rotate_hue(&mut self, degrees: f32) {
        self.colors = self
            .colors
            .iter()
            .map(|c| {
                let mut l = c.lcha();
                let hue = (l.hue.to_degrees() + degrees) % 360.0;
                l.hue = LabHue::from_degrees(hue);
                let rgba: Srgba = l.into_color();
                rgba.into()
            })
            .collect();
    }

    /// Sort the colors by hue using the CIELCh color space.
    pub fn sort_by_hue(&mut self) {
        self.colors.sort_by_cached_key(|c| {
            let lcha = c.lcha();
            (1000.0 * lcha.hue.to_radians()) as u32
        })
    }

    /// Sort the colors by chroma using the CIELCh color space.
    pub fn sort_by_chroma(&mut self) {
        self.colors.sort_by_cached_key(|c| {
            let lcha = c.lcha();
            (1000.0 * lcha.chroma) as u32
        })
    }

    /// Sort the colors by lightness using the CIELCh color space.
    pub fn sort_by_lightness(&mut self) {
        self.colors.sort_by_cached_key(|c| {
            let lcha = c.lcha();
            (1000.0 * lcha.l) as u32
        })
    }

    /// Sort the colors by alpha(opacity) using the CIELCh color space.
    pub fn sort_by_alpha(&mut self) {
        self.colors.sort_by_cached_key(|c| {
            let lcha = c.lcha();
            (1000.0 * lcha.alpha) as u32
        })
    }

    /// Choose a color from the palette at random.
    pub fn rand_color(&mut self) -> RGBA {
        self.colors[self.rng.gen_range(0..self.colors.len())]
    }

    /// Generate a random opaque color independent of the `Palette` colors.
    pub fn rand_lab(&mut self) -> RGBA {
        let l: f32 = self.rng.gen_range(0.0..100.0);
        let a: f32 = self.rng.gen_range(-128.0..127.0);
        let b: f32 = self.rng.gen_range(-128.0..127.0);
        let rgb: Srgb = Lab::new(l, a, b).into_color();
        rgb.into()
    }

    /// Generate a random color and random opacity independent of the `Palette` colors.
    pub fn rand_laba(&mut self) -> RGBA {
        let l: f32 = self.rng.gen_range(0.0..100.0);
        let a: f32 = self.rng.gen_range(-128.0..127.0);
        let b: f32 = self.rng.gen_range(-128.0..127.0);
        let o: f32 = self.rng.gen_range(0.0..1.0);
        let rgba: Srgba = Laba::new(l, a, b, o).into_color();
        rgba.into()
    }

    /// Perturb the colors in the palette using a normal distrtibution with
    /// standard deviation `std_dev` considered as a percentage.
    pub fn jiggle(&mut self, seed: u64, std_dev: f32) {
        let mut j = Jiggle::new(seed, std_dev);
        let cs: Vec<RGBA> = self.colors.iter().map(|c| j.jiggle(*c)).collect();
        self.colors = cs;
    }

    /// The number of colors in the `Palette`.
    pub fn len(&self) -> usize {
        self.colors.len()
    }
}

/// Allow colors to be accessed as if `Palette` was an array, e.g. `palette[42]`.
impl Index<usize> for Palette {
    type Output = RGBA;

    fn index(&self, index: usize) -> &Self::Output {
        &self.colors[index]
    }
}

/// Allow colors to be accessed and mutated as if `Palette` was an array, e.g. `palette[42] = GRAY`.
impl IndexMut<usize> for Palette {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.colors[index]
    }
}

impl IntoIterator for Palette {
    type Item = RGBA;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.colors.into_iter()
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

/// [Procedural Color Palettess](https://iquilezles.org/www/articles/palettes/palettes.htm).
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

    /// Create a procedural color as a function of the angle `theta` (radians).
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

/// Get a color from an image by mapping the canvas coordinates to image coordinates.
pub fn get_color<T: AsPrimitive<f32>>(img: &DynamicImage, width: T, height: T, p: Point) -> RGBA {
    let x = ((p.x * img.width() as f32 / width.as_()) as u32).clamp(0, img.width() - 1);
    let y = ((p.y * img.height() as f32 / height.as_()) as u32).clamp(0, img.height() - 1);
    let p = img.get_pixel(x, y);
    p.into()
}
