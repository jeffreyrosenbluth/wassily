use palette::{
    rgb::{Rgb, Rgba},
    white_point::D65,
    Alpha, ConvertInto, Lab, Laba, Lcha, Srgb, Srgba,
};
use rand::prelude::*;
use rand_pcg::Pcg64;
use tiny_skia::{Color, Pixmap};

pub fn black(alpha: f32) -> Color {
    Color::from_rgba(0.0, 0.0, 0.0, alpha).unwrap()
}

pub fn white(alpha: f32) -> Color {
    Color::from_rgba(1.0, 1.0, 1.0, alpha).unwrap()
}

pub fn red(alpha: f32) -> Color {
    Color::from_rgba(1.0, 0.0, 0.0, alpha).unwrap()
}

pub fn green(alpha: f32) -> Color {
    Color::from_rgba(0.0, 1.0, 0.0, alpha).unwrap()
}

pub fn blue(alpha: f32) -> Color {
    Color::from_rgba(0.0, 0.0, 1.0, alpha).unwrap()
}

/// Convert a tiny_skia 'Color' to a palette Lcha.
pub fn lcha(c: Color) -> Lcha<D65> {
    let r = c.red();
    let g = c.green();
    let b = c.blue();
    let a = c.alpha();
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

pub fn cos_color(r: CosChannel, g: CosChannel, b: CosChannel, theta: f32) -> Color {
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

pub struct Palette {
    pub colors: Vec<Color>,
    rng: Pcg64,
}

impl Palette {
    /// Generate a palatte from a vector of 'Color's
    pub fn new(colors: Vec<Color>) -> Self {
        let rng = Pcg64::seed_from_u64(0);
        Palette { colors, rng }
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.rng = Pcg64::seed_from_u64(seed);
    }

    /// Generate a palatte from the colors in an image and sort them by
    /// there euclidean distance from Black;
    pub fn with_img(img: Pixmap, n: usize) -> Self {
        let mut cs = vec![];
        let w = img.width();
        let h = img.height();
        let delta = (w as f32 * h as f32 / n as f32).sqrt();
        let mut x = 0.0;
        let mut y = 0.0;
        while x <= w as f32 {
            while y <= h as f32 {
                let p = img.pixel(x as u32, y as u32).unwrap();
                let r = p.red();
                let g = p.green();
                let b = p.blue();
                let a = p.alpha();
                let c = Color::from_rgba8(r, g, b, a);
                cs.push(c);
                y += delta;
            }
            x += delta;
            y = 0.0;
        }
        cs.truncate(n);
        cs.sort_by_cached_key(|c| {
            (1000.0 * (c.red() * c.red() + c.green() * c.green() + c.blue() * c.blue())) as u32
        });
        Self::new(cs)
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

    pub fn rand_color(&mut self) -> Color {
        self.colors[self.rng.gen_range(0..self.colors.len())]
    }

    pub fn rand_lab(&mut self) -> Color {
        let l: f32 = self.rng.gen_range(0.0..100.0);
        let a: f32 = self.rng.gen_range(-128.0..127.0);
        let b: f32 = self.rng.gen_range(-128.0..127.0);
        let rgb: Srgb = Lab::new(l, a, b).convert_into();
        let c = rgb.into_components();
        Color::from_rgba(c.0, c.1, c.2, 1.0).unwrap()
    }

    pub fn rand_laba(&mut self) -> Color {
        let l: f32 = self.rng.gen_range(0.0..100.0);
        let a: f32 = self.rng.gen_range(-128.0..127.0);
        let b: f32 = self.rng.gen_range(-128.0..127.0);
        let o: f32 = self.rng.gen_range(0.0..1.0);
        let rgba: Srgba = Laba::new(l, a, b, o).convert_into();
        let c = rgba.into_components();
        Color::from_rgba(c.0, c.1, c.2, c.3).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_set() {
        let img = Pixmap::new(70, 50).unwrap();
        let palette = Palette::with_img(img, 100);
        assert_eq!(palette.colors.len(), 100);
    }
}
