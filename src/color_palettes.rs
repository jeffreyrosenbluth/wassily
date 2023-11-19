//! A Palette of colors and functions to manage them.

use crate::kolor::*;
use crate::util::map_range;
use color_thief::{get_palette, ColorFormat};
use image::GenericImageView;
use noise::NoiseFn;
use palette::{Lcha, Okhsla};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::{
    f32::consts::TAU,
    ops::{Index, IndexMut},
    path::Path,
};
use tiny_skia::Color;

/// A Palette of colors and functions to manage them.
#[derive(Clone, Debug)]
pub struct Palette {
    pub colors: Vec<Color>,
    rng: SmallRng,
    pub current: usize,
}

impl Default for Palette {
    fn default() -> Self {
        Palette::new(vec![])
    }
}

impl Palette {
    /// Generate a palette from a vector of 'Color's
    pub fn new(colors: Vec<Color>) -> Self {
        let rng = SmallRng::seed_from_u64(0);
        Palette {
            colors,
            rng,
            current: 0,
        }
    }

    /// Set the seed of the random number generator used in all of the random
    /// color functions.
    pub fn set_seed(&mut self, seed: u64) {
        self.rng = SmallRng::seed_from_u64(seed);
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
                    cs.push(p.to_color());
                    y += delta;
                }
                x += delta;
                y = 0.0;
            }
            cs.truncate(n)
        } else {
            for (_, _, p) in img.pixels() {
                cs.push(p.to_color());
            }
            cs.sort_by_cached_key(|c| c.as_u8s());
            cs.dedup_by_key(|c| c.as_u8s());
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
        self.colors = self.colors.iter().map(|c| c.tighten()).collect();
    }

    /// Change the lighness of the colors to their square root, i.e. spreading
    /// them towards lighter or darker which ever is closer.
    pub fn spread(&mut self) {
        self.colors = self.colors.iter().map(|c| c.spread()).collect();
    }

    /// Rotate the hue of each color in the palette.
    pub fn rotate_hue(&mut self, degrees: f32) {
        self.colors = self.colors.iter().map(|c| c.rotate_hue(degrees)).collect();
    }

    /// Saturate the colors in the palette by a factor.
    pub fn saturate(&mut self, factor: f32) {
        self.colors = self.colors.iter().map(|c| c.saturate(factor)).collect();
    }

    /// Saturate the colors in the palette by a fixed amount.
    pub fn saturate_fixed(&mut self, amount: f32) {
        self.colors = self
            .colors
            .iter()
            .map(|c| c.saturate_fixed(amount))
            .collect();
    }

    /// Desaturate the colors in the palette by a factor.
    pub fn desaturate(&mut self, factor: f32) {
        self.colors = self.colors.iter().map(|c| c.desaturate(factor)).collect();
    }

    /// Desaturate the colors in the palette by a fixed amount.
    pub fn desaturate_fixed(&mut self, amount: f32) {
        self.colors = self
            .colors
            .iter()
            .map(|c| c.desaturate_fixed(amount))
            .collect();
    }

    /// Lighten the colors in the palette by a factor.
    pub fn lighten(&mut self, factor: f32) {
        self.colors = self.colors.iter().map(|c| c.lighten(factor)).collect();
    }

    /// Lighten the colors in the palette by a fixed amount.
    pub fn lighten_fixed(&mut self, amount: f32) {
        self.colors = self
            .colors
            .iter()
            .map(|c| c.lighten_fixed(amount))
            .collect();
    }

    /// Darken the colors in the palette by a factor.
    pub fn darken(&mut self, factor: f32) {
        self.colors = self.colors.iter().map(|c| c.darken(factor)).collect();
    }

    /// Darken the colors in the palette by a fixed amount.
    pub fn darken_fixed(&mut self, amount: f32) {
        self.colors = self.colors.iter().map(|c| c.darken_fixed(amount)).collect();
    }

    /// Sort the colors by hue using the Okhsl color space.
    pub fn sort_by_hue(&mut self) {
        self.colors.sort_by_cached_key(|c| {
            let okhsla: Okhsla = <Okhsla as ConvertColor>::from_color(c);
            (1000.0 * okhsla.hue.into_radians()) as u32
        })
    }

    /// Sort the colors by saturation using the Okhsl color space.
    pub fn sort_by_saturation(&mut self) {
        self.colors.sort_by_cached_key(|c| {
            let okhsla: Okhsla = <Okhsla as ConvertColor>::from_color(c);
            (1000.0 * okhsla.saturation) as u32
        })
    }

    /// Sort the colors by lightness using the Okhsl color space.
    pub fn sort_by_lightness(&mut self) {
        self.colors.sort_by_cached_key(|c| {
            let okhsla: Okhsla = <Okhsla as ConvertColor>::from_color(c);
            (1000.0 * okhsla.lightness) as u32
        })
    }

    /// Sore the colors by chroma using the CIELCh color space.
    pub fn sort_by_chroma(&mut self) {
        self.colors.sort_by_cached_key(|c| {
            let lcha: Lcha = <Lcha as ConvertColor>::from_color(c);
            (1000.0 * lcha.chroma) as u32
        })
    }

    /// Sort the colors by alpha(opacity) using the Okhsl color space.
    pub fn sort_by_alpha(&mut self) {
        self.colors.sort_by_cached_key(|c| {
            let okhsla: Okhsla = <Okhsla as ConvertColor>::from_color(c);
            (1000.0 * okhsla.alpha) as u32
        })
    }

    /// Choose a color from the palette at random.
    pub fn rand_color(&mut self) -> Color {
        self.colors[self.rng.gen_range(0..self.colors.len())]
    }

    /// Perturb the colors in the palette using a normal distrtibution with
    /// standard deviation `std_dev` considered as a percentage.
    pub fn jiggle(&mut self, std_dev: f32) {
        let cs: Vec<Color> = self
            .colors
            .iter()
            .map(|c| jiggle(&mut self.rng, std_dev, *c))
            .collect();
        self.colors = cs;
    }

    /// The number of colors in the `Palette`.
    pub fn len(&self) -> usize {
        self.colors.len()
    }

    /// Is the `Palette` empty?
    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }
}

/// Allow colors to be accessed as if `Palette` was an array, e.g. `palette[42]`.
/// If the index is out of bounds, it will wrap around.
impl Index<usize> for Palette {
    type Output = Color;

    fn index(&self, index: usize) -> &Self::Output {
        let index = index % self.colors.len();
        &self.colors[index]
    }
}

/// Allow colors to be accessed and mutated as if `Palette` was an array, e.g. `palette[42] = GRAY`.
/// If the index is out of bounds, it will wrap around.
impl IndexMut<usize> for Palette {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let index = index % self.colors.len();
        &mut self.colors[index]
    }
}

/// An interator for the `Palette`.
impl IntoIterator for Palette {
    type Item = Color;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.colors.into_iter()
    }
}

/// A color scale is a gradient made from five colors. The gradient is
/// is created by finding the 5 term fourier series that fits the five colors.
/// The colors are the values of the function at t = 1/6,2/6, 3/6, 4/6, and 5/6.
/// Where t is in [0,1].
pub struct ColorScale {
    pub a0: (f32, f32, f32),
    pub a1: (f32, f32, f32),
    pub a2: (f32, f32, f32),
    pub b1: (f32, f32, f32),
    pub b2: (f32, f32, f32),
}

// Map color channel for [0,1] to [-1,1].
fn f(x: f32) -> f32 {
    2.0 * x - 1.0
}

impl ColorScale {
    pub fn new(color0: Color, color1: Color, color2: Color, color3: Color, color4: Color) -> Self {
        let ra0 = 0.3333 * f(color0.red()) + 0.3333 * f(color2.red()) + 0.3333 * f(color4.red());
        let ra1 = 0.5 * f(color0.red()) - 0.5 * f(color1.red()) - 0.5 * f(color3.red())
            + 0.5 * f(color4.red());
        let ra2 = 0.1667 * f(color0.red()) - 0.5 * f(color1.red()) + 0.6667 * f(color2.red())
            - 0.5 * f(color3.red())
            + 0.1667 * f(color4.red());
        let rb1 = 0.2887 * f(color0.red()) + 0.2887 * f(color1.red())
            - 0.2887 * f(color3.red())
            - 0.2887 * f(color4.red());
        let rb2 = 0.2887 * f(color0.red()) - 0.2887 * f(color1.red()) + 0.2887 * f(color3.red())
            - 0.2887 * f(color4.red());

        let ga0 =
            0.3333 * f(color0.green()) + 0.3333 * f(color2.green()) + 0.3333 * f(color4.green());
        let ga1 = 0.5 * f(color0.green()) - 0.5 * f(color1.green()) - 0.5 * f(color3.green())
            + 0.5 * f(color4.green());
        let ga2 = 0.1667 * f(color0.green()) - 0.5 * f(color1.green()) + 0.6667 * f(color2.green())
            - 0.5 * f(color3.green())
            + 0.1667 * f(color4.green());
        let gb1 = 0.2887 * f(color0.green()) + 0.2887 * f(color1.green())
            - 0.2887 * f(color3.green())
            - 0.2887 * f(color4.green());
        let gb2 = 0.2887 * f(color0.green()) - 0.2887 * f(color1.green())
            + 0.2887 * f(color3.green())
            - 0.2887 * f(color4.green());

        let ba0 = 0.3333 * f(color0.blue()) + 0.3333 * f(color2.blue()) + 0.3333 * f(color4.blue());
        let ba1 = 0.5 * f(color0.blue()) - 0.5 * f(color1.blue()) - 0.5 * f(color3.blue())
            + 0.5 * f(color4.blue());
        let ba2 = 0.1667 * f(color0.blue()) - 0.5 * f(color1.blue()) + 0.6667 * f(color2.blue())
            - 0.5 * f(color3.blue())
            + 0.1667 * f(color4.blue());
        let bb1 = 0.2887 * f(color0.blue()) + 0.2887 * f(color1.blue())
            - 0.2887 * f(color3.blue())
            - 0.2887 * f(color4.blue());
        let bb2 = 0.2887 * f(color0.blue()) - 0.2887 * f(color1.blue()) + 0.2887 * f(color3.blue())
            - 0.2887 * f(color4.blue());

        Self {
            a0: (ra0, ga0, ba0),
            a1: (ra1, ga1, ba1),
            a2: (ra2, ga2, ba2),
            b1: (rb1, gb1, bb1),
            b2: (rb2, gb2, bb2),
        }
    }

    pub fn get_color(&self, t: f32) -> Color {
        let red = 0.5
            + 0.5
                * (self.a0.0
                    + self.a1.0 * (TAU * t).cos()
                    + self.a2.0 * (2.0 * TAU * t).cos()
                    + self.b1.0 * (TAU * t).sin()
                    + self.b2.0 * (2.0 * TAU * t).sin());
        let green = 0.5
            + 0.5
                * (self.a0.1
                    + self.a1.1 * (TAU * t).cos()
                    + self.a2.1 * (2.0 * TAU * t).cos()
                    + self.b1.1 * (TAU * t).sin()
                    + self.b2.1 * (2.0 * TAU * t).sin());
        let blue = 0.5
            + 0.5
                * (self.a0.2
                    + self.a1.2 * (TAU * t).cos()
                    + self.a2.2 * (2.0 * TAU * t).cos()
                    + self.b1.2 * (TAU * t).sin()
                    + self.b2.2 * (2.0 * TAU * t).sin());

        Color::from_rgba(
            red.clamp(0.0, 1.0),
            green.clamp(0.0, 1.0),
            blue.clamp(0.0, 1.0),
            1.0,
        )
        .unwrap()
    }
}
pub struct NoiseColorScale {
    pub color_scale: ColorScale,
    pub noise: Box<dyn NoiseFn<f64, 2>>,
    pub scale: f64,
}

impl NoiseColorScale {
    pub fn new(
        color1: Color,
        color2: Color,
        color3: Color,
        color4: Color,
        color5: Color,
        noise: Box<dyn NoiseFn<f64, 2>>,
        scale: f64,
    ) -> Self {
        let color_scale = ColorScale::new(color1, color2, color3, color4, color5);
        Self {
            color_scale,
            noise,
            scale,
        }
    }
    pub fn set_scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    pub fn set_noise(mut self, noise: Box<dyn NoiseFn<f64, 2>>) -> Self {
        self.noise = noise;
        self
    }

    pub fn get_color(&self, x: f32, y: f32) -> Color {
        let s = self
            .noise
            .get([self.scale * x as f64, self.scale * y as f64]) as f32;
        self.color_scale.get_color(s)
    }
}

/// 'ColorRamp' is a color gradient made from four colors. The gradient is
/// defined by two cubic curves, one for the red, green, and blue channels.
/// The first curve is fit to the points (0, 1/3, 2/3, 0) and the second curve
/// is fit to the points (0.2, 0.4, 0.6, 0.8).
/// The curves are combined with weigth (1-alpha) on the second curve.
/// The input t for the second curve is mapped to a sine wave with a frequency.
/// This crates a color gradient with lots of variation and includes the origingal
/// four colors.
pub struct ColorRamp {
    a0: (f32, f32, f32),
    b0: (f32, f32, f32),
    c0: (f32, f32, f32),
    d0: (f32, f32, f32),
    a1: (f32, f32, f32),
    b1: (f32, f32, f32),
    c1: (f32, f32, f32),
    d1: (f32, f32, f32),
    alpha: f32,
    frequency: f32,
}

impl ColorRamp {
    pub fn new(
        color1: Color,
        color2: Color,
        color3: Color,
        color4: Color,
        alpha: f32,
        frequency: f32,
    ) -> Self {
        let (ra0, rb0, rc0, rd0) =
            fit_cubic_0(color1.red(), color2.red(), color3.red(), color4.red());
        let (ga0, gb0, gc0, gd0) = fit_cubic_0(
            color1.green(),
            color2.green(),
            color3.green(),
            color4.green(),
        );
        let (ba0, bb0, bc0, bd0) =
            fit_cubic_0(color1.blue(), color2.blue(), color3.blue(), color4.blue());

        let (ra1, rb1, rc1, rd1) =
            fit_cubic_1(color1.red(), color2.red(), color3.red(), color4.red());
        let (ga1, gb1, gc1, gd1) = fit_cubic_1(
            color1.green(),
            color2.green(),
            color3.green(),
            color4.green(),
        );
        let (ba1, bb1, bc1, bd1) =
            fit_cubic_1(color1.blue(), color2.blue(), color3.blue(), color4.blue());
        Self {
            a0: (ra0, ga0, ba0),
            b0: (rb0, gb0, bb0),
            c0: (rc0, gc0, bc0),
            d0: (rd0, gd0, bd0),
            a1: (ra1, ga1, ba1),
            b1: (rb1, gb1, bb1),
            c1: (rc1, gc1, bc1),
            d1: (rd1, gd1, bd1),
            alpha,
            frequency,
        }
    }

    /// The default color ramp has a frequency of 2.0 and an alpha of 0.3333.
    /// i.e. 2/3 weight on curve 1 and 1/3 weight on curve 2.
    pub fn standard(color1: Color, color2: Color, color3: Color, color4: Color) -> Self {
        Self::new(color1, color2, color3, color4, 0.3333, 2.0)
    }

    pub fn set_alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha;
        self
    }

    pub fn get_color(&self, t: f32) -> Color {
        if t > 1.0 || t < 0.0 {
            println!("WARNING: t must be between 0 and 1, but t = {}", t);
        }
        let t = t.clamp(0.0, 1.0);
        let t2 = t * t;
        let t3 = t2 * t;
        let red0 = (self.a0.0 * t3 + self.b0.0 * t2 + self.c0.0 * t + self.d0.0).clamp(0.0, 1.0);
        let green0 = (self.a0.1 * t3 + self.b0.1 * t2 + self.c0.1 * t + self.d0.1).clamp(0.0, 1.0);
        let blue0 = (self.a0.2 * t3 + self.b0.2 * t2 + self.c0.2 * t + self.d0.2).clamp(0.0, 1.0);

        let u = 0.5 + 0.5 * (TAU * self.frequency * t).sin();
        let u2 = u * u;
        let u3 = u2 * u;
        let red1 = (self.a1.0 * u3 + self.b1.0 * u2 + self.c1.0 * u + self.d1.0).clamp(0.0, 1.0);
        let green1 = (self.a1.1 * u3 + self.b1.1 * u2 + self.c1.1 * u + self.d1.1).clamp(0.0, 1.0);
        let blue1 = (self.a1.2 * u3 + self.b1.2 * u2 + self.c1.2 * u + self.d1.2).clamp(0.0, 1.0);

        let red = (1.0 - self.alpha) * red0 + self.alpha * red1;
        let green = (1.0 - self.alpha) * green0 + self.alpha * green1;
        let blue = (1.0 - self.alpha) * blue0 + self.alpha * blue1;
        Color::from_rgba(red, green, blue, 1.0).unwrap()
    }
}

fn fit_cubic_0(y0: f32, y1: f32, y2: f32, y3: f32) -> (f32, f32, f32, f32) {
    let a = -4.5 * y0 + 13.5 * y1 - 13.5 * y2 + 4.5 * y3;
    let b = 9.0 * y0 - 22.5 * y1 + 18.0 * y2 - 4.5 * y3;
    let c = -5.5 * y0 + 9.0 * y1 - 4.5 * y2 + y3;
    let d = y0;
    (a, b, c, d)
}
fn fit_cubic_1(y0: f32, y1: f32, y2: f32, y3: f32) -> (f32, f32, f32, f32) {
    let a = -20.8333 * y0 + 62.5 * y1 - 62.5 * y2 + 20.8333 * y3;
    let b = 37.5 * y0 - 100.0 * y1 + 87.5 * y2 - 25.0 * y3;
    let c = -21.6667 * y0 + 47.5 * y1 - 35.0 * y2 + 9.1667 * y3;
    let d = 4.0 * y0 - 6.0 * y1 + 4.0 * y2 - y3;
    (a, b, c, d)
}

pub struct NoiseColorRamp {
    pub color_quad: ColorRamp,
    pub noise: Box<dyn NoiseFn<f64, 2>>,
    pub scale: f64,
    pub alpha: f32,
}

impl NoiseColorRamp {
    pub fn new(
        color1: Color,
        color2: Color,
        color3: Color,
        color4: Color,
        noise: Box<dyn NoiseFn<f64, 2>>,
        scale: f64,
        alpha: f32,
        frequency: f32,
    ) -> Self {
        let color_quad = ColorRamp::new(color1, color2, color3, color4, alpha, frequency);
        Self {
            color_quad,
            noise,
            scale,
            alpha,
        }
    }
    pub fn set_scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    pub fn set_noise(mut self, noise: Box<dyn NoiseFn<f64, 2>>) -> Self {
        self.noise = noise;
        self
    }

    pub fn get_color(&self, x: f32, y: f32) -> Color {
        let s = self
            .noise
            .get([self.scale * x as f64, self.scale * y as f64]) as f32;
        let t = map_range(s, -2.0, 2.0, 0.0, 1.0);
        self.color_quad.get_color(t)
    }
}
