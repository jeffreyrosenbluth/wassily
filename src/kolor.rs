//! Utilities to manage colors.

use crate::noises::white::normal_xy;
use crate::points::pt;
use image::{DynamicImage, GenericImageView};
use num_traits::AsPrimitive;
use palette::{
    rgb::{Rgb, Rgba},
    Alpha, FromColor, Hsl, Hsla, Hsluv, Hsluva, Hsv, Hsva, Hwb, Hwba, IntoColor, Lab, Laba, Lch,
    Lcha, Lighten, Mix, Okhsl, Okhsla, Okhsv, Okhsva, Saturate, ShiftHue, Srgb, Srgba, Xyz, Xyza,
};
use rand::{Rng, RngCore};
use rand_distr::{Distribution, Normal};
use tiny_skia::{Color, Point};

/// Trait for converting between color types and TinySkia's `Color`.
pub trait ConvertColor {
    fn to_color(&self) -> Color;
    fn from_color(color: &Color) -> Self;
}

impl ConvertColor for Color {
    fn to_color(&self) -> Color {
        *self
    }

    fn from_color(color: &Color) -> Self {
        *color
    }
}

impl ConvertColor for Srgb {
    fn to_color(&self) -> Color {
        Color::from_rgba(self.red, self.green, self.blue, 1.0).unwrap()
    }

    fn from_color(color: &Color) -> Self {
        let (r, g, b, _) = color.as_f32s();
        Rgb::new(r, g, b)
    }
}

impl ConvertColor for Srgba {
    fn to_color(&self) -> Color {
        Color::from_rgba(self.red, self.green, self.blue, self.alpha).unwrap()
    }

    fn from_color(color: &Color) -> Self {
        let (r, g, b, a) = color.as_f32s();
        Rgba::new(r, g, b, a)
    }
}

/// Implement ConvertColor for opaque palette color types.
macro_rules! convert_color {
    ($c:ty) => {
        impl ConvertColor for $c {
            fn to_color(&self) -> Color {
                let srgb =
                    <palette::rgb::Srgb as FromColor<$c>>::from_color(*self).into_components();
                Color::from_rgba(srgb.0, srgb.1, srgb.2, 1.0).unwrap()
            }

            fn from_color(color: &Color) -> $c {
                let (r, g, b, _) = color.as_f32s();
                let srgb: Alpha<Rgb, f32> = Rgba::new(r, g, b, 1.0);
                srgb.into_color()
            }
        }
    };
}

/// Implement ConvertColor for palette color types swith alpha.
macro_rules! convert_color_alpha {
    ($c:ty) => {
        impl ConvertColor for $c {
            fn to_color(&self) -> Color {
                let srgba =
                    <palette::rgb::Srgba as FromColor<$c>>::from_color(*self).into_components();
                Color::from_rgba(srgba.0, srgba.1, srgba.2, srgba.3).unwrap()
            }

            fn from_color(color: &Color) -> Self {
                let (r, g, b, a) = color.as_f32s();
                let srgb: Alpha<Rgb, f32> = Rgba::new(r, g, b, a);
                srgb.into_color()
            }
        }
    };
}

convert_color!(Hsluv);
convert_color_alpha!(Hsluva);
convert_color!(Hsl);
convert_color_alpha!(Hsla);
convert_color!(Lch);
convert_color_alpha!(Lcha);
convert_color!(Lab);
convert_color_alpha!(Laba);
convert_color!(Xyz);
convert_color_alpha!(Xyza);
convert_color!(Hsv);
convert_color_alpha!(Hsva);
convert_color!(Hwb);
convert_color_alpha!(Hwba);
convert_color!(Okhsl);
convert_color_alpha!(Okhsla);
convert_color!(Okhsv);
convert_color_alpha!(Okhsva);

impl ConvertColor for image::Rgb<u8> {
    fn to_color(&self) -> Color {
        rgb8(self.0[0], self.0[1], self.0[2])
    }
    fn from_color(color: &Color) -> image::Rgb<u8> {
        let r = color.red() * 255.0 + 0.5;
        let g = color.green() * 255.0 + 0.5;
        let b = color.blue() * 255.0 + 0.5;
        image::Rgb([r as u8, g as u8, b as u8])
    }
}

impl ConvertColor for image::Rgba<u8> {
    fn to_color(&self) -> Color {
        Color::from_rgba8(self.0[0], self.0[1], self.0[2], self.0[3])
    }
    fn from_color(color: &Color) -> image::Rgba<u8> {
        let r = color.red() * 255.0 + 0.5;
        let g = color.green() * 255.0 + 0.5;
        let b = color.blue() * 255.0 + 0.5;
        let a = color.alpha() * 255.0 + 0.5;
        image::Rgba([r as u8, g as u8, b as u8, a as u8])
    }
}

/// Build a mapping form points on a canvas to colors. The mapping is done by
/// providing 3 functions that map numbers to rgb components and a function to
/// create an f32 from a point.
pub struct ColorMapping {
    pub red_fn: Box<dyn Fn(f32) -> f32>,
    pub green_fn: Box<dyn Fn(f32) -> f32>,
    pub blue_fn: Box<dyn Fn(f32) -> f32>,
    pub color_grad: Box<dyn Fn(Point) -> f32>,
}

impl ColorMapping {
    pub fn new(
        red_fn: Box<dyn Fn(f32) -> f32>,
        green_fn: Box<dyn Fn(f32) -> f32>,
        blue_fn: Box<dyn Fn(f32) -> f32>,
        color_grad: Box<dyn Fn(Point) -> f32>,
    ) -> Self {
        Self {
            red_fn,
            green_fn,
            blue_fn,
            color_grad,
        }
    }

    pub fn get(&self, x: f32, y: f32) -> Color {
        let t = (self.color_grad)(pt(x, y));
        let r = (self.red_fn)(t);
        let g = (self.green_fn)(t);
        let b = (self.blue_fn)(t);
        Color::from_rgba(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0), 1.0).unwrap()
    }
}

/// The `Colorful` trait exists primarily to add methods to tiny-skia's `Color`
/// type. Of coures, it can be implemented for other color types as well.
pub trait Colorful {
    /// Same as origingal color with a new alpha.
    fn opacity(&self, alpha: f32) -> Self;

    /// A tuple of the color's rgba components as f32s.
    fn as_f32s(&self) -> (f32, f32, f32, f32);

    /// A tuple of the color's rgba components as u8s.
    fn as_u8s(&self) -> (u8, u8, u8, u8);

    /// Linear interpolation between two colors.
    /// Colors should be converted to a linear color space before mixing
    /// and then back to the original color space.
    fn lerp(&self, color2: &Self, t: f32) -> Self;

    /// Perturb a color based on its position on the canvas. The perturbation is
    /// done by adding a random number from a normal distribution with the given
    /// mean and standard deviation to each of the color's components.
    fn jiggle_xy(&self, x: u32, y: u32, mean: f32, std: f32) -> Self;

    /// Perturb a color's lightness based on its position on the canvas. The
    /// perturbation is done by adding a random number from a normal distribution
    /// with the given mean and standard deviation to the color's lightness.
    fn jiggle_xy_lightness(&self, x: u32, y: u32, mean: f32, std: f32) -> Self;

    /// Perturb a color's saturation based on its position on the canvas. The
    /// perturbation is done by adding a random number from a normal distribution
    /// with the given mean and standard deviation to the color's saturation.
    fn jiggle_xy_saturation(&self, x: u32, y: u32, mean: f32, std: f32) -> Self;

    /// Perturb a color's hue based on its position on the canvas. The
    /// perturbation is done by adding a random number from a normal distribution
    /// with the given mean and standard deviation to the color's hue.
    fn jiggle_xy_hue(&self, x: u32, y: u32, mean: f32, std: f32) -> Self;

    /// Convert a color to grayscale.
    fn grayscale(&self) -> Self;

    /// Rotate the hue of a color by the given number of degrees.
    fn rotate_hue(&self, degrees: f32) -> Self;

    /// Change the lighness of a color to it's square, i.e. tightening
    /// it away from lighter or darker which ever is closer.
    fn tighten(&self) -> Self;

    /// Change the lighness of a color to it's square root, i.e. spreading
    /// it towards lighter or darker which ever is closer.
    fn spread(&self) -> Self;

    /// Tint a color by mixing it with white. 0 is no white, 1 is all white.
    fn tint(&self, t: f32) -> Self;

    /// Tone a color by mixing it with gray. 0 is no gray, 1 is all gray.
    fn tone(&self, t: f32) -> Self;

    /// Shade a color by mixing it with black. 0 is no black, 1 is all black.
    fn shade(&self, t: f32) -> Self;

    /// Lighten a color by the given factor.
    fn lighten(&self, factor: f32) -> Self;

    /// Lighten a color a fixed abount.
    fn lighten_fixed(&self, amount: f32) -> Self;

    /// Darken a color by the given factor.
    fn darken(&self, factor: f32) -> Self
    where
        Self: Sized,
    {
        self.lighten(-factor)
    }

    /// Darken a color a fixed abount.
    fn darken_fixed(&self, amount: f32) -> Self
    where
        Self: Sized,
    {
        self.lighten_fixed(-amount)
    }

    /// Saturate a color by the given factor.
    fn saturate(&self, factor: f32) -> Self;

    /// Saturate a color a fixed abount.
    fn saturate_fixed(&self, amount: f32) -> Self;

    /// Desaturate a color by the given factor.
    fn desaturate(&self, factor: f32) -> Self
    where
        Self: Sized,
    {
        self.saturate(-factor)
    }

    /// Desaturate a color a fixed abount.
    fn desaturate_fixed(&self, amount: f32) -> Self
    where
        Self: Sized,
    {
        self.saturate_fixed(-amount)
    }
}

impl Colorful for Color {
    fn opacity(&self, alpha: f32) -> Self {
        let mut c = *self;
        c.set_alpha(alpha);
        c
    }

    fn as_f32s(&self) -> (f32, f32, f32, f32) {
        (self.red(), self.green(), self.blue(), self.alpha())
    }

    fn as_u8s(&self) -> (u8, u8, u8, u8) {
        let r = self.red() * 255.0 + 0.5;
        let g = self.green() * 255.0 + 0.5;
        let b = self.blue() * 255.0 + 0.5;
        let a = self.alpha() * 255.0 + 0.5;
        (r as u8, g as u8, b as u8, a as u8)
    }

    /// Perturb a color based on its position on the canvas. The perturbation is
    /// done by adding a random number from a normal distribution with the given
    /// mean and standard deviation to each of the color's components.
    fn jiggle_xy(&self, x: u32, y: u32, mean: f32, std: f32) -> Color {
        let (r, g, b, a) = self.as_f32s();
        Color::from_rgba(
            (r + (std * normal_xy(x as f64, y as f64) as f32 + mean)).clamp(0.0, 1.0),
            (g + (std * normal_xy(x as f64 + 1.0, y as f64) as f32 + mean)).clamp(0.0, 1.0),
            (b + (std * normal_xy(x as f64, y as f64 + 1.0) as f32 + mean)).clamp(0.0, 1.0),
            a,
        )
        .unwrap()
    }

    /// Perturb a color's lightness based on its position on the canvas. The
    /// perturbation is done by adding a random number from a normal distribution
    /// with the given mean and standard deviation to the color's lightness.
    fn jiggle_xy_lightness(&self, x: u32, y: u32, mean: f32, std: f32) -> Color {
        let mut hsluva: Hsluva = <Hsluva as ConvertColor>::from_color(self);
        hsluva.l += (std * normal_xy(x as f64, y as f64) as f32 + mean) * 100.0;
        hsluva.to_color()
    }

    /// Perturb a color's saturation based on its position on the canvas. The
    /// perturbation is done by adding a random number from a normal distribution
    /// with the given mean and standard deviation to the color's saturation.
    fn jiggle_xy_saturation(&self, x: u32, y: u32, mean: f32, std: f32) -> Color {
        let mut hsluva: Hsluva = <Hsluva as ConvertColor>::from_color(self);
        hsluva.saturation += (std * normal_xy(x as f64, y as f64) as f32 + mean) * 100.0;
        hsluva.to_color()
    }

    /// Perturb a color's hue based on its position on the canvas. The
    /// perturbation is done by adding a random number from a normal distribution
    /// with the given mean and standard deviation to the color's hue.
    fn jiggle_xy_hue(&self, x: u32, y: u32, mean: f32, std: f32) -> Color {
        let mut hsluva: Hsluva = <Hsluva as ConvertColor>::from_color(self);
        hsluva.hue += (std * normal_xy(x as f64, y as f64) as f32 + mean) * 360.0;
        hsluva.to_color()
    }

    /// Convert a color to grayscale.
    fn grayscale(&self) -> Self {
        let (r, g, b, _) = self.as_f32s();
        let c = (255.0 * (0.2989 * r + 0.5870 * g + 0.1140 * b)).clamp(0.0, 255.0) as u8;
        rgb8(c, c, c)
    }

    /// Rotate the hue of a color by the given number of degrees.
    fn rotate_hue(&self, degrees: f32) -> Color {
        let okhsla: Okhsla = <Okhsla as ConvertColor>::from_color(self);
        okhsla.shift_hue(degrees).to_color()
    }
    /// Change the lighness of a color to it's square, i.e. tightening
    /// it away from lighter or darker which ever is closer.
    fn tighten(&self) -> Color {
        let mut okhsla: Okhsla = <Okhsla as ConvertColor>::from_color(self);
        let l1 = okhsla.lightness / 50.0 - 1.0;
        let l2 = l1.abs() * l1.abs() * l1.signum();
        let l3 = 50.0 * (l2 + 1.0);
        okhsla.lightness = l3;
        okhsla.to_color()
    }

    /// Change the lighness of a color to it's square root, i.e. spreading
    /// it towards lighter or darker which ever is closer.
    fn spread(&self) -> Color {
        let mut okhsla: Okhsla = <Okhsla as ConvertColor>::from_color(self);
        let l1 = okhsla.lightness / 50.0 - 1.0;
        let l2 = l1.abs().sqrt() * l1.signum();
        let l3 = 50.0 * (l2 + 1.0);
        okhsla.lightness = l3;
        okhsla.to_color()
    }

    /// Tint a color by mixing it with white. 0 is no white, 1 is all white.
    fn tint(&self, t: f32) -> Color {
        self.lerp(&rgb8(255, 255, 255), t)
    }

    /// Tone a color by mixing it with gray. 0 is no gray, 1 is all gray.
    fn tone(&self, t: f32) -> Color {
        self.lerp(&rgb8(127, 127, 127), t)
    }

    /// Shade a color by mixing it with black. 0 is no black, 1 is all black.
    fn shade(&self, t: f32) -> Color {
        self.lerp(&rgb8(0, 0, 0), t)
    }

    /// Saturate a color by the given factor.
    fn saturate(&self, factor: f32) -> Color {
        let okhsla: Okhsla = <Okhsla as ConvertColor>::from_color(self);
        okhsla.saturate(factor).to_color()
    }

    /// Saturate a color a fixed abount.
    fn saturate_fixed(&self, amount: f32) -> Self {
        let okhsla: Okhsla = <Okhsla as ConvertColor>::from_color(self);
        okhsla.saturate_fixed(amount).to_color()
    }

    /// Lighten a color by the given factor.
    fn lighten(&self, factor: f32) -> Self {
        let okhsla: Okhsla = <Okhsla as ConvertColor>::from_color(self);
        okhsla.lighten(factor).to_color()
    }

    /// Lighten a color a fixed abount.
    fn lighten_fixed(&self, amount: f32) -> Self {
        let okhsla: Okhsla = <Okhsla as ConvertColor>::from_color(self);
        okhsla.lighten_fixed(amount).to_color()
    }

    /// Linearly interpolate between two colors in a linear color space and convert
    /// back to sRGBA.
    fn lerp(&self, color2: &Color, t: f32) -> Self {
        let s = t.clamp(0.0, 1.0);
        let c1 = <Srgba as ConvertColor>::from_color(self).into_linear();
        let c2 = <Srgba as ConvertColor>::from_color(color2).into_linear();
        Srgba::from_linear(c1.mix(c2, s)).to_color()
    }
}

/// Create an opaque color from red, green, and blue f32 components.
pub fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color::from_rgba(r, g, b, 1.0).unwrap_or_else(|| {
        panic!(
            "color components must be between 0 and 1: ({}, {}, {}",
            r, g, b
        )
    })
}

/// Create an opaque color from red, green, and blue u8 components.
pub fn rgb8(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgba8(r, g, b, 255)
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

/// Perturb the r, g, b channels of an `Color` color using a normal distribution.
/// The value is clamped to [0, 1] and applied as a percentage.
pub fn jiggle<R: RngCore>(rng: &mut R, std_dev: f32, color: Color) -> Color {
    let normal = Normal::new(0.0, std_dev).unwrap();
    let (r, g, b, a) = color.as_f32s();
    Color::from_rgba(
        (r + normal.sample(rng)).clamp(0.0, 1.0),
        (g + normal.sample(rng)).clamp(0.0, 1.0),
        (b + normal.sample(rng)).clamp(0.0, 1.0),
        a,
    )
    .unwrap()
}

/// Perturb the lighness of a color by a sample from the normal distribution with
/// standard deviation `std_dev`.
pub fn jiggle_lightness<R: RngCore>(rng: &mut R, std_dev: f32, color: Color) -> Color {
    let normal = Normal::new(0.0, std_dev).unwrap();
    let mut okhsla: Okhsla = <Okhsla as ConvertColor>::from_color(&color);
    okhsla.lightness += normal.sample(rng) * 100.0;
    okhsla.to_color()
}

/// Perturb the saturation of a color by a sample from the normal distribution with
/// standard deviation `std_dev`.
pub fn jiggle_saturation<R: RngCore>(rng: &mut R, std_dev: f32, color: Color) -> Color {
    let normal = Normal::new(0.0, std_dev).unwrap();
    let mut okhsla: Okhsla = <Okhsla as ConvertColor>::from_color(&color);
    okhsla.saturation += normal.sample(rng) * 100.0;
    okhsla.to_color()
}

/// Perturb the hue of a color by a sample from the normal distribution with
/// standard deviation `std_dev`.
pub fn jiggle_hue<R: RngCore>(rng: &mut R, std_dev: f32, color: Color) -> Color {
    let normal = Normal::new(0.0, std_dev).unwrap();
    let mut okhsla: Okhsla = <Okhsla as ConvertColor>::from_color(&color);
    okhsla.hue += normal.sample(rng) * 360.0;
    okhsla.to_color()
}

/// Generate a random opaque color from the Okhsl color space.
pub fn rand_okhsl<R: RngCore>(rng: &mut R) -> Color {
    let normal = Normal::new(0.0, 0.25).unwrap();
    let h: f32 = rng.gen_range(0.0..360.0);
    let s: f32 = 0.65 + normal.sample(rng);
    let l: f32 = 0.5 + normal.sample(rng);
    Okhsl::new(h, s.clamp(0.0, 1.0), l.clamp(0.0, 1.0)).to_color()
}

/// Generate a random color from the Okhsla color space.
pub fn rand_okhsla<R: RngCore>(rng: &mut R) -> Color {
    let normal = Normal::new(0.0, 0.25).unwrap();
    let h: f32 = rng.gen_range(0.0..360.0);
    let s: f32 = 0.7 + normal.sample(rng);
    let l: f32 = 0.5 + normal.sample(rng);
    let a: f32 = rng.gen_range(0.0..1.0);
    Okhsla::new(h, s, l, a).to_color()
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
        Some(p.to_color())
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
    p.to_color()
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
    p.to_color()
}

/// Get a color from an image by tiling the image.
pub fn get_color_tile<T: AsPrimitive<f32>>(img: &DynamicImage, p: Point) -> Color {
    let x = (p.x as u32).rem_euclid(img.width());
    let y = (p.y as u32).rem_euclid(img.height());
    let p = img.get_pixel(x, y);
    p.to_color()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_test() {
        let c1 = Color::BLACK;
        let c2 = gray(255);
        assert_eq!(c1.lerp(&c2, 0.5), rgb8(187, 187, 187));
    }
}
