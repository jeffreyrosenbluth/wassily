//! Utilities to manage colors and palettes.

use crate::quiet::white::normal_xy;
use color_thief::{get_palette, ColorFormat};
use image::{DynamicImage, GenericImageView};
use num_traits::AsPrimitive;
use palette::{
    rgb::{Rgb, Rgba},
    Alpha, FromColor, Hsluva, Hue, IntoColor, Laba, Mix, Srgba,
};
use rand::prelude::*;
use rand_distr::Normal;
use rand_pcg::Pcg64;
use std::{ops::Index, ops::IndexMut, path::Path, usize};
use tiny_skia::{Color, Point};

const PI: f32 = std::f32::consts::PI;

pub fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color::from_rgba(r, g, b, 1.0).expect("color components must be between 0 and 1")
}

pub fn rgb8(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgba8(r, g, b, 255)
}

pub fn as_f32s(c: Color) -> (f32, f32, f32, f32) {
    (c.red(), c.green(), c.blue(), c.alpha())
}

pub fn as_u8s(c: Color) -> (u8, u8, u8, u8) {
    let r = c.red() * 255.0;
    let g = c.green() * 255.0;
    let b = c.blue() * 255.0;
    let a = c.alpha() * 255.0;
    (r as u8, g as u8, b as u8, a as u8)
}

pub fn jiggle_xy(color: Color, x: u32, y: u32, mean: f32, std: f32) -> Color {
    let (r, g, b, a) = as_f32s(color);
    Color::from_rgba(
        (r + (std * normal_xy(123, x, y) as f32 + mean)).clamp(0.0, 1.0),
        (g + (std * normal_xy(456, x, y) as f32 + mean)).clamp(0.0, 1.0),
        (b + (std * normal_xy(789, x, y) as f32 + mean)).clamp(0.0, 1.0),
        a,
    )
    .unwrap()
}

pub fn jiggle_xy_lightness(color: Color, x: u32, y: u32, mean: f32, std: f32) -> Color {
    let mut l: Hsluva = color_to_hsluva(color);
    l.l += (std * normal_xy(123, x, y) as f32 + mean) * 100.0;
    let rgba = Srgba::from_color(l);
    srgba_to_color(rgba)
}

pub fn jiggle_xy_saturation(color: Color, x: u32, y: u32, mean: f32, std: f32) -> Color {
    let mut l: Hsluva = color_to_hsluva(color);
    l.saturation += (std * normal_xy(123, x, y) as f32 + mean) * 100.0;
    let rgba = Srgba::from_color(l);
    srgba_to_color(rgba)
}

pub fn jiggle_xy_hue(color: Color, x: u32, y: u32, mean: f32, std: f32) -> Color {
    let mut l: Hsluva = color_to_hsluva(color);
    l.hue += (std * normal_xy(123, x, y) as f32 + mean) * 360.0;
    let rgba = Srgba::from_color(l);
    srgba_to_color(rgba)
}

/// Perturb a `Color` value.
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

    /// Perturb the r, g, b channels of an `Color` color using a normal distribution.
    /// The value is clamped to [0, 1] and applied as a percentage.
    pub fn jiggle(&mut self, color: Color) -> Color {
        let (r, g, b, a) = as_f32s(color);
        Color::from_rgba(
            (r + self.normal.sample(&mut self.rng)).clamp(0.0, 1.0),
            (g + self.normal.sample(&mut self.rng)).clamp(0.0, 1.0),
            (b + self.normal.sample(&mut self.rng)).clamp(0.0, 1.0),
            a,
        )
        .unwrap()
    }

    pub fn jiggle_lightness(&mut self, color: Color) -> Color {
        let mut l: Hsluva = color_to_hsluva(color);
        l.l += self.normal.sample(&mut self.rng) * 100.0;
        let rgba = Srgba::from_color(l);
        srgba_to_color(rgba)
    }

    pub fn jiggle_saturation(&mut self, color: Color) -> Color {
        let mut l: Hsluva = color_to_hsluva(color);
        l.saturation += self.normal.sample(&mut self.rng) * 100.0;
        let rgba = Srgba::from_color(l);
        srgba_to_color(rgba)
    }

    pub fn jiggle_hue(&mut self, color: Color) -> Color {
        let mut l: Hsluva = color_to_hsluva(color);
        l.hue += self.normal.sample(&mut self.rng) * 360.0;
        let rgba = Srgba::from_color(l);
        srgba_to_color(rgba)
    }
}

//     /// Black with opacity alpha [0.0, 1.0].
pub fn black(alpha: f32) -> Color {
    Color::from_rgba(0.0, 0.0, 0.0, alpha).unwrap()
}

/// White with opacity alpha [0.0, 1.0].
pub fn white(alpha: f32) -> Color {
    Color::from_rgba(1.0, 1.0, 1.0, alpha).unwrap()
}

/// Gray, set r, g, and b to the same value 0..255.
pub fn gray(n: u8) -> Color {
    Color::from_rgba8(n, n, n, 255)
}

pub fn grayscale(color: Color) -> u8 {
    let (r, g, b, _) = as_f32s(color);
    (0.2989 * r as f32 + 0.5870 * g as f32 + 0.1140 * b as f32).clamp(0.0, 255.0) as u8
}

pub fn rotate_hue(color: Color, degrees: f32) -> Color {
    let l: Hsluva = color_to_hsluva(color);
    let rgba = Srgba::from_color(l.shift_hue(degrees));
    srgba_to_color(rgba)
}

/// Change the lighness of a color to it's square, i.e. tightening
/// it away lighter or darker which ever is closer.
pub fn tighten(color: Color) -> Color {
    let mut hsluva: Hsluva = color_to_hsluva(color);
    let l1 = hsluva.l / 50.0 - 1.0;
    let l2 = l1.abs() * l1.abs() * l1.signum();
    let l3 = 50.0 * (l2 + 1.0);
    hsluva.l = l3;
    let c: Srgba = hsluva.into_color();
    srgba_to_color(c)
}

/// Change the lighness of a color to it's square root, i.e. spreading
/// it towards lighter or darker which ever is closer.
pub fn spread(color: Color) -> Color {
    let mut hsluva: Hsluva = color_to_hsluva(color);
    let l1 = hsluva.l / 50.0 - 1.0;
    let l2 = l1.abs().sqrt() * l1.signum();
    let l3 = 50.0 * (l2 + 1.0);
    hsluva.l = l3;
    let c: Srgba = hsluva.into_color();
    srgba_to_color(c)
}

pub fn lerp(color1: Color, color2: Color, t: f32) -> Color {
    let s = t.clamp(0.0, 1.0);
    let c1 = color_to_srgba(color1).into_linear();
    let c2 = color_to_srgba(color2).into_linear();
    let c = Srgba::from_linear(c1.mix(&c2, s));
    srgba_to_color(c)
}

pub fn tint(color: Color, t: f32) -> Color {
    lerp(color, rgb8(255, 255, 255), t)
}

pub fn tone(color: Color, t: f32) -> Color {
    lerp(color, rgb8(127, 127, 127), t)
}

pub fn shade(color: Color, t: f32) -> Color {
    lerp(color, rgb8(0, 0, 0), t)
}

pub fn color_to_hsluva(color: Color) -> Hsluva {
    let (r, g, b, a) = as_f32s(color);
    let srgb: Alpha<Rgb, f32> = Rgba::new(r, g, b, a);
    srgb.into_color()
}

pub fn color_to_srgba(color: Color) -> Srgba {
    let (r, g, b, a) = as_f32s(color);
    let srgb: Alpha<Rgb, f32> = Rgba::new(r, g, b, a);
    srgb.into_color()
}

pub fn image_rgba_to_color(p: image::Rgba<u8>) -> Color {
    Color::from_rgba8(p.0[0], p.0[1], p.0[2], p.0[3])
}

pub fn color_to_image_rgba(c: Color) -> image::Rgba<u8> {
    let r = c.red() * 255.0;
    let g = c.green() * 255.0;
    let b = c.blue() * 255.0;
    let a = c.alpha() * 255.0;
    image::Rgba([r as u8, g as u8, b as u8, a as u8])
}

pub fn srgba_to_color(rgb: Srgba) -> Color {
    let c = rgb.into_components();
    Color::from_rgba(c.0, c.1, c.2, c.3).unwrap()
}

/// A Palette of colors and functions to manage them.
#[derive(Clone, Debug)]
pub struct Palette {
    pub colors: Vec<Color>,
    rng: Pcg64,
    pub current: usize,
}

impl Default for Palette {
    fn default() -> Self {
        Palette::new(vec![])
    }
}

impl Palette {
    /// Generate a palatte from a vector of 'Color's
    pub fn new(colors: Vec<Color>) -> Self {
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
        let mut cs: Vec<Color> = vec![];
        let w = img.width();
        let h = img.height();
        if let Some(n) = n {
            let delta = (w as f32 * h as f32 / n as f32).sqrt();
            let mut x = 0.0;
            let mut y = 0.0;
            while x < w as f32 {
                while y < h as f32 {
                    let p = img.get_pixel(x as u32, y as u32);
                    cs.push(image_rgba_to_color(p));
                    y += delta;
                }
                x += delta;
                y = 0.0;
            }
            cs.truncate(n)
        } else {
            for (_, _, p) in img.pixels() {
                cs.push(image_rgba_to_color(p));
            }
            cs.sort_by_cached_key(|c| as_u8s(*c));
            cs.dedup_by_key(|c| as_u8s(*c));
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
        let palette = palette
            .into_iter()
            .map(|c| Color::from_rgba8(c.r, c.g, c.b, 255));
        Self::new(palette.collect())
    }

    pub fn tighten(&mut self) {
        self.colors = self.colors.iter().map(|c| tighten(*c)).collect();
    }

    /// Change the lighness of the colors to their square root, i.e. spreading
    /// them towards lighter or darker which ever is closer.
    pub fn spread(&mut self) {
        self.colors = self.colors.iter().map(|c| spread(*c)).collect();
    }

    /// Rotate the [palette::LabHue] of each color.
    pub fn rotate_hue(&mut self, degrees: f32) {
        self.colors = self
            .colors
            .iter()
            .map(|c| rotate_hue(*c, degrees))
            .collect();
    }

    /// Sort the colors by hue using the CIELCh color space.
    pub fn sort_by_hue(&mut self) {
        self.colors.sort_by_cached_key(|c| {
            let hsluva = color_to_hsluva(*c);
            (1000.0 * hsluva.hue.to_radians()) as u32
        })
    }

    /// Sort the colors by chroma using the CIELCh color space.
    pub fn sort_by_saturation(&mut self) {
        self.colors.sort_by_cached_key(|c| {
            let hsluva = color_to_hsluva(*c);
            (1000.0 * hsluva.saturation) as u32
        })
    }

    /// Sort the colors by lightness using the CIELCh color space.
    pub fn sort_by_lightness(&mut self) {
        self.colors.sort_by_cached_key(|c| {
            let hsluva = color_to_hsluva(*c);
            (1000.0 * hsluva.l) as u32
        })
    }

    /// Sort the colors by alpha(opacity) using the CIELCh color space.
    pub fn sort_by_alpha(&mut self) {
        self.colors.sort_by_cached_key(|c| {
            let hsluva = color_to_hsluva(*c);
            (1000.0 * hsluva.alpha) as u32
        })
    }

    /// Choose a color from the palette at random.
    pub fn rand_color(&mut self) -> Color {
        self.colors[self.rng.gen_range(0..self.colors.len())]
    }

    /// Generate a random opaque color independent of the `Palette` colors.
    pub fn rand_lab(&mut self) -> Color {
        let l: f32 = self.rng.gen_range(0.0..100.0);
        let a: f32 = self.rng.gen_range(-128.0..127.0);
        let b: f32 = self.rng.gen_range(-128.0..127.0);
        let rgba: Srgba = Laba::new(l, a, b, 1.0).into_color();
        srgba_to_color(rgba)
    }

    /// Generate a random color and random opacity independent of the `Palette` colors.
    pub fn rand_laba(&mut self) -> Color {
        let l: f32 = self.rng.gen_range(0.0..100.0);
        let a: f32 = self.rng.gen_range(-128.0..127.0);
        let b: f32 = self.rng.gen_range(-128.0..127.0);
        let o: f32 = self.rng.gen_range(0.0..1.0);
        let rgba: Srgba = Laba::new(l, a, b, o).into_color();
        srgba_to_color(rgba)
    }

    /// Perturb the colors in the palette using a normal distrtibution with
    /// standard deviation `std_dev` considered as a percentage.
    pub fn jiggle(&mut self, seed: u64, std_dev: f32) {
        let mut j = Jiggle::new(seed, std_dev);
        let cs: Vec<Color> = self.colors.iter().map(|c| j.jiggle(*c)).collect();
        self.colors = cs;
    }

    /// The number of colors in the `Palette`.
    pub fn len(&self) -> usize {
        self.colors.len()
    }
}

/// Allow colors to be accessed as if `Palette` was an array, e.g. `palette[42]`.
impl Index<usize> for Palette {
    type Output = Color;

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
    type Item = Color;

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

impl Default for CosChannel {
    fn default() -> Self {
        Self {
            a: 0.5,
            b: 0.5,
            freq: 1.0,
            phase: 0.0,
        }
    }
}

/// [Procedural Color Palettess](https://iquilezles.org/www/articles/palettes/palettes.htm).
#[derive(Debug, Clone, Copy)]
pub struct CosColor {
    pub r: CosChannel,
    pub g: CosChannel,
    pub b: CosChannel,
}

impl CosColor {
    pub fn new(r: CosChannel, g: CosChannel, b: CosChannel) -> Self {
        Self { r, g, b }
    }

    /// Create a procedural color as a function of the angle `theta` (radians).
    pub fn cos_color(&self, theta: f32) -> Color {
        let r = self.r;
        let g = self.g;
        let b = self.b;
        let red = r.a + r.b * (r.freq * theta + r.phase).cos();
        let green = g.a + g.b * (g.freq * theta + g.phase).cos();
        let blue = b.a + b.b * (b.freq * theta + b.phase).cos();
        Color::from_rgba(
            red.clamp(0.0, 1.0),
            green.clamp(0.0, 1.0),
            blue.clamp(0.0, 1.0),
            1.0,
        )
        .unwrap()
    }

    pub fn rainbow() -> Self {
        let r = CosChannel::default();
        let mut g = CosChannel::default();
        let mut b = CosChannel::default();
        g.phase = 0.33 * 2.0 * PI;
        b.phase = 0.66 * 2.0 * PI;
        Self { r, g, b }
    }

    pub fn berry() -> Self {
        let mut r = CosChannel::default();
        let mut g = CosChannel::default();
        let mut b = CosChannel::default();
        r.phase = 0.3 * 2.0 * PI;
        g.phase = 0.2 * 2.0 * PI;
        b.phase = 0.2 * 2.0 * PI;
        Self { r, g, b }
    }

    pub fn rain_forest() -> Self {
        let mut r = CosChannel::default();
        let mut g = CosChannel::default();
        let mut b = CosChannel::default();
        r.phase = 0.8 * 2.0 * PI;
        g.phase = 0.9 * 2.0 * PI;
        b.freq = 0.5;
        b.phase = 0.3 * 2.0 * PI;
        Self { r, g, b }
    }

    pub fn pink_gold() -> Self {
        let r = CosChannel::default();
        let mut g = CosChannel::default();
        let mut b = CosChannel::default();
        g.freq = 0.7;
        g.phase = 0.15 * 2.0 * PI;
        b.freq = 0.4;
        b.phase = 0.2 * 2.0 * PI;
        Self { r, g, b }
    }

    pub fn fuschia() -> Self {
        let mut r = CosChannel::default();
        let mut g = CosChannel::default();
        let mut b = CosChannel::default();
        r.freq = 1.0;
        r.phase = 0.5 * 2.0 * PI;
        g.freq = 1.0;
        g.phase = 0.2 * 2.0 * PI;
        b.freq = 0.0;
        b.phase = 0.25 * 2.0 * PI;
        Self { r, g, b }
    }

    pub fn watermelon() -> Self {
        let mut r = CosChannel::default();
        let mut g = CosChannel::default();
        let mut b = CosChannel::default();
        r.a = 0.8;
        r.b = 0.2;
        r.freq = 2.0;
        r.phase = 0.0;
        g.a = 0.5;
        g.b = 0.4;
        g.freq = 1.0;
        g.phase = 0.25 * 2.0 * PI;
        b.a = 0.4;
        b.b = 0.2;
        b.freq = 1.0;
        b.phase = 0.25 * 2.0 * PI;
        Self { r, g, b }
    }
}

impl Default for CosColor {
    fn default() -> Self {
        let r = CosChannel::default();
        let mut g = CosChannel::default();
        let mut b = CosChannel::default();
        g.phase = 0.2 * PI;
        b.phase = 0.4 * PI;
        Self { r, g, b }
    }
}

/// Get a color from an image by mapping the canvas coordinates to image coordinates.
pub fn get_color<T: AsPrimitive<f32>>(
    img: &DynamicImage,
    width: T,
    height: T,
    p: Point,
) -> Option<Color> {
    if p.x < 0.0 || p.x >= width.as_() || p.y < 0.0 || p.y >= height.as_() {
        None
    } else {
        let x = (p.x * img.width() as f32 / width.as_()) as u32;
        let y = (p.y * img.height() as f32 / height.as_()) as u32;
        let p = img.get_pixel(x, y);
        Some(image_rgba_to_color(p))
    }
}

/// Get a color from an image by mapping the canvas coordinates to image coordinates. If the
/// point 'p' is out of bounds wrap around as if the image is a torus.
pub fn get_color_wrap<T: AsPrimitive<f32>>(
    img: &DynamicImage,
    width: T,
    height: T,
    p: Point,
) -> Color {
    let x = ((p.x * img.width() as f32 / width.as_()) as i32).rem_euclid(img.width() as i32);
    let y = ((p.y * img.height() as f32 / height.as_()) as i32).rem_euclid(img.height() as i32);
    let p = img.get_pixel(x as u32, y as u32);
    image_rgba_to_color(p)
}

/// Get a color from an image by mapping the canvas coordinates to image coordinates.
/// point 'p' is out of bounds clamp the coordinate.
pub fn get_color_clamp<T: AsPrimitive<f32>>(
    img: &DynamicImage,
    width: T,
    height: T,
    p: Point,
) -> Color {
    let x = ((p.x * img.width() as f32 / width.as_()) as u32).clamp(0, img.width() - 1);
    let y = ((p.y * img.height() as f32 / height.as_()) as u32).clamp(0, img.height() - 1);
    let p = img.get_pixel(x, y);
    image_rgba_to_color(p)
}

/// Get a color from an image by tiling the image.
pub fn get_color_tile<T: AsPrimitive<f32>>(img: &DynamicImage, p: Point) -> Color {
    let x = (p.x as u32).rem_euclid(img.width());
    let y = (p.y as u32).rem_euclid(img.height());
    let p = img.get_pixel(x, y);
    image_rgba_to_color(p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_test() {
        let c1 = Color::BLACK;
        let c2 = gray(255);
        assert_eq!(lerp(c1, c2, 0.5), rgb8(187, 187, 187));
    }
}
