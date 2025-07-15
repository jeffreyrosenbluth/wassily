//! # Color Palettes and Scaling
//!
//! Tools for managing color palettes and creating smooth color transitions.
//! This module provides the `Palette` struct for basic color management and
//! the `ColorScale` struct for mathematically smooth color interpolation using
//! Fourier series.
//!
//! ## Key Components
//!
//! - **[`Palette`]**: Basic color palette management with random access
//! - **[`ColorScale`]**: Fourier-based smooth color transitions
//! - **Image Extraction**: Extract dominant colors from images
//! - **Utility Functions**: Color manipulation and helper functions
//!
//! ## Palette Management
//!
//! ```no_run
//! use wassily_color::*;
//!
//! // Create a palette from colors
//! let mut palette = Palette::new(vec![
//!     *CRIMSON,
//!     *GOLD,
//!     *FORESTGREEN,
//!     *STEELBLUE,
//! ]);
//!
//! // Access colors
//! let first_color = palette.get_color(0);
//! let random_color = palette.get_random_color();
//! palette.shuffle(); // Randomize order
//! ```
//!
//! ## Color Scaling
//!
//! ```no_run
//! use wassily_color::*;
//!
//! // Create a smooth color scale
//! let scale = ColorScale::new(
//!     *RED, *ORANGE, *YELLOW, *GREEN, *BLUE
//! );
//!
//! // Get interpolated colors
//! let color_at_25_percent = scale.get_color(0.25);
//! let fractal_color = scale.get_color_fractal(0.5, 10.0);
//! ```
//!
//! ## Image Color Extraction
//!
//! ```no_run
//! use wassily_color::*;
//!
//! // Extract dominant colors from an image
//! let palette = Palette::from_image("artwork.jpg", 5);
//! ```

use crate::color::*;

/// Map a value from one range to another.
fn map_range(x: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (x - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
}
use color_thief::{get_palette, ColorFormat};
use image::GenericImageView;
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
        self.colors[self.rng.random_range(0..self.colors.len())]
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
/// Where t is in \[0,1\]. A nice feature of 'ColorScale' is that they are periodic.
#[derive(Clone, Debug)]
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

fn coefficients(
    channel0: f32,
    channel1: f32,
    channel2: f32,
    channel3: f32,
    channel4: f32,
) -> (f32, f32, f32, f32, f32) {
    let a0 = 0.3333 * f(channel0) + 0.3333 * f(channel2) + 0.3333 * f(channel4);
    let a1 = 0.5 * f(channel0) - 0.5 * f(channel1) - 0.5 * f(channel3) + 0.5 * f(channel4);
    let a2 = 0.1667 * f(channel0) - 0.5 * f(channel1) + 0.6667 * f(channel2) - 0.5 * f(channel3)
        + 0.1667 * f(channel4);
    let b1 =
        0.2887 * f(channel0) + 0.2887 * f(channel1) - 0.2887 * f(channel3) - 0.2887 * f(channel4);
    let b2 =
        0.2887 * f(channel0) - 0.2887 * f(channel1) + 0.2887 * f(channel3) - 0.2887 * f(channel4);
    (a0, a1, a2, b1, b2)
}

impl ColorScale {
    pub fn new(color0: Color, color1: Color, color2: Color, color3: Color, color4: Color) -> Self {
        let (ra0, ra1, ra2, rb1, rb2) = coefficients(
            color0.red(),
            color1.red(),
            color2.red(),
            color3.red(),
            color4.red(),
        );
        let (ga0, ga1, ga2, gb1, gb2) = coefficients(
            color0.green(),
            color1.green(),
            color2.green(),
            color3.green(),
            color4.green(),
        );
        let (ba0, ba1, ba2, bb1, bb2) = coefficients(
            color0.blue(),
            color1.blue(),
            color2.blue(),
            color3.blue(),
            color4.blue(),
        );

        Self {
            a0: (ra0, ga0, ba0),
            a1: (ra1, ga1, ba1),
            a2: (ra2, ga2, ba2),
            b1: (rb1, gb1, bb1),
            b2: (rb2, gb2, bb2),
        }
    }

    /// Create a color scale from 2 colors by interpolating the other 3
    /// between them.
    pub fn from_2(color0: Color, color4: Color) -> Self {
        let color1 = color0.lerp(&color4, 0.25);
        let color2 = color0.lerp(&color4, 0.5);
        let color3 = color0.lerp(&color4, 0.75);
        Self::new(color0, color1, color2, color3, color4)
    }

    /// Create a color scale from 3 colors by interpolating the other 2
    /// in between them.
    pub fn from_3_colors(color0: Color, color2: Color, color4: Color) -> Self {
        let color1 = color0.lerp(&color2, 0.5);
        let color3 = color2.lerp(&color4, 0.5);
        Self::new(color0, color1, color2, color3, color4)
    }

    /// Create a color scale from 3 colors plus black and white.
    pub fn from_3_bw(color0: Color, color2: Color, color4: Color) -> Self {
        let color1 = Color::from_rgba8(0, 0, 0, 255);
        let color3 = Color::from_rgba8(255, 255, 255, 255);
        Self::new(color0, color1, color2, color3, color4)
    }

    /// Get a color from the color scale at t in \[0,1\].
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

    /// Get a color from the fractal version of the color scale at t in \[0,1\].
    pub fn get_color_fractal(&self, t: f32, phase: f32) -> Color {
        let a = self.get_color(t);
        let b = self.get_color(2.0 * t).rotate_hue(phase);
        let c = self.get_color(3.0 * t).rotate_hue(2.0 * phase);
        let d = b.lerp(&c, 0.5);
        a.lerp(&d, 0.5)
    }

    /// Get a color from the scale but clip it to limit the range of colors.
    /// Useful for creating a palette of colors that are similar to a standard
    /// linear gradient.  0.1 is a good value for clip.
    pub fn get_color_clip(&self, t: f32, clip: f32) -> Color {
        let t = map_range(t, 0.0, 1.0, clip, 1.0 - clip);
        self.get_color(t)
    }
}
