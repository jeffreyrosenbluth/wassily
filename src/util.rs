use std::marker::PhantomData;

use noise::{MultiFractal, NoiseFn, OpenSimplex, Seedable};
use palette::{ConvertInto, Lab, Laba, Srgb, Srgba};
use rand::prelude::*;
use rand_distr::uniform::SampleUniform;
use rand_pcg::Pcg64;
use tiny_skia::*;

pub const TAU: f32 = std::f32::consts::TAU;
pub const PI: f32 = std::f32::consts::PI;
const EPSILON: f32 = 0.001;

pub fn pt2(x: f32, y: f32) -> Point {
    Point::from_xy(x, y)
}

pub fn map_range(x: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (x - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
}

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

pub struct CosChannel {
    pub a: f32,
    pub b: f32,
    pub freq: f32, // radians
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
    Color::from_rgba(red, green, blue, 1.0).unwrap()
}

fn curl(f: impl Fn(f32, f32) -> f32, x: f32, y: f32, eps: f32) -> f32 {
    let x0 = x - eps;
    let x1 = x + eps;
    let y0 = y - eps;
    let y1 = y + eps;
    let dfdx = (f(x1, y) - f(x0, y)) / (2.0 * eps);
    let dfdy = (f(x, y1) - f(x, y0)) / (2.0 * eps);
    dfdy.atan2(-dfdx)
}

pub struct WK<N, T>
where
    T: NoiseFn<N>,
{
    pub width: f32,
    pub height: f32,
    rng: Pcg64,
    noise_fn: T,
    x_scale: f32,
    y_scale: f32,
    z_scale: f32,
    noise_factor: f32,
    colors: Vec<Color>,
    phantom: PhantomData<N>,
}

impl<N, T> WK<N, T>
where
    T: NoiseFn<N>,
{
    pub fn new(width: f32, height: f32, noise_fn: T) -> Self {
        let rng = Pcg64::seed_from_u64(0);
        let x_scale = 1.0;
        let y_scale = 1.0;
        let z_scale = 1.0;
        let noise_factor = 2.0;
        let colors = vec![];
        Self {
            width,
            height,
            rng,
            noise_fn,
            x_scale,
            y_scale,
            z_scale,
            noise_factor,
            colors,
            phantom: PhantomData,
        }
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.rng = Pcg64::seed_from_u64(seed);
    }

    pub fn set_noise_scales(&mut self, x_scale: f32, y_scale: f32, z_scale: f32) {
        self.x_scale = x_scale;
        self.y_scale = y_scale;
        self.z_scale = z_scale;
    }

    pub fn set_noise_fn(&mut self, nf: T) {
        self.noise_fn = nf;
    }

    pub fn set_colors(&mut self, img: Pixmap, n: usize) {
        let w = img.width();
        let h = img.height();
        for _ in 0..n {
            let i = self.rand_range(0.0, w as f32) as u32;
            let j = self.rand_range(0.0, h as f32) as u32;
            let p = img.pixel(i, j).unwrap();
            let r = p.red();
            let g = p.green();
            let b = p.blue();
            let c = Color::from_rgba8(r, g, b, 255);
            self.colors.push(c);
        }
        self.colors.sort_by_cached_key(|c| {
            (1000.0 * (c.red() * c.red() + c.green() * c.green() + c.blue() * c.blue())) as u32
        })
    }

    pub fn color(&self, i: usize) -> Color {
        self.colors[i]
    }

    pub fn colors(&self) -> Vec<Color> {
        self.colors.clone()
    }

    pub fn rand_color(&mut self) -> Color {
        let n = self.colors.len();
        let i = self.rand_range(0, n);
        self.colors[i]
    }

    pub fn width_n(&self) -> u32 {
        self.width as u32
    }

    pub fn height_n(&self) -> u32 {
        self.height as u32
    }

    pub fn center(&self) -> Point {
        pt2(self.width / 2.0, self.height / 2.0)
    }

    pub fn rand_range<U: SampleUniform + PartialOrd>(&mut self, low: U, high: U) -> U {
        self.rng.gen_range(low..high)
    }

    pub fn rand_rgb(&mut self) -> Color {
        let l: f32 = self.rand_range(0.0, 100.0);
        let a: f32 = self.rand_range(-128.0, 127.0);
        let b: f32 = self.rand_range(-128.0, 127.0);
        let rgb: Srgb = Lab::new(l, a, b).convert_into();
        let c = rgb.into_components();
        Color::from_rgba(c.0, c.1, c.2, 1.0).unwrap()
    }

    pub fn rand_rgba(&mut self) -> Color {
        let l: f32 = self.rand_range(0.0, 100.0);
        let a: f32 = self.rand_range(-128.0, 127.0);
        let b: f32 = self.rand_range(-128.0, 127.0);
        let o: f32 = self.rand_range(0.5, 1.0);
        let rgba: Srgba = Laba::new(l, a, b, o).convert_into();
        let c = rgba.into_components();
        Color::from_rgba(c.0, c.1, c.2, c.3).unwrap()
    }
}

impl<T> WK<[f64; 2], T>
where
    T: NoiseFn<[f64; 2]>,
{
    pub fn noise(&self, x: f32, y: f32) -> f32 {
        let center = self.center();
        self.noise_factor
            * self.noise_fn.get([
                (1.0 / center.x * self.x_scale * (x - center.x)) as f64,
                (1.0 / center.y * self.y_scale * (y - center.y)) as f64,
            ]) as f32
    }
}

impl<T> WK<[f64; 3], T>
where
    T: NoiseFn<[f64; 3]>,
{
    pub fn noise(&self, x: f32, y: f32, z: f32) -> f32 {
        let center = self.center();
        self.noise_factor
            * self.noise_fn.get([
                (1.0 / center.x * self.x_scale * (x - center.x)) as f64,
                (1.0 / center.y * self.y_scale * (y - center.y)) as f64,
                (self.z_scale * z) as f64,
            ]) as f32
    }
}

impl<N, T> WK<N, T>
where
    T: NoiseFn<N> + Seedable + Clone,
{
    pub fn noise_seed(&self) -> u32 {
        self.noise_fn.seed()
    }

    pub fn set_noise_seed(&mut self, seed: u32) {
        let nf = self.noise_fn.clone().set_seed(seed);
        self.set_noise_fn(nf);
    }
}

impl<N, T> WK<N, T>
where
    T: NoiseFn<N> + MultiFractal + Clone,
{
    pub fn set_octaves(&mut self, o: usize) {
        let nf = self.noise_fn.clone().set_octaves(o);
        self.set_noise_fn(nf);
    }

    pub fn set_frequency(&mut self, f: f64) {
        let nf = self.noise_fn.clone().set_frequency(f);
        self.set_noise_fn(nf);
    }

    pub fn set_persistence(&mut self, p: f64) {
        let nf = self.noise_fn.clone().set_persistence(p);
        self.set_noise_fn(nf);
    }

    pub fn set_lacunarity(&mut self, l: f64) {
        let nf = self.noise_fn.clone().set_lacunarity(l);
        self.set_noise_fn(nf);
    }
}

pub struct Wassily {
    pub width: f32,
    pub height: f32,
    rng: Pcg64,
    noise_fn: Box<dyn NoiseFn<[f64; 3]>>,
    noise_scale: f32,
    colors: Vec<Color>,
    simplex_noise: OpenSimplex,
}

impl Wassily {
    pub fn new(width: f32, height: f32) -> Self {
        let rng = Pcg64::seed_from_u64(0);
        let noise_fn = Box::new(noise::OpenSimplex::new());
        let noise_scale = 0.003;
        let colors = vec![];
        let simplex_noise = OpenSimplex::new();
        Self {
            width,
            height,
            rng,
            noise_fn,
            noise_scale,
            colors,
            simplex_noise,
        }
    }

    pub fn set_simplex_seed(mut self, seed: u32) {
        self.simplex_noise = self.simplex_noise.set_seed(seed);
    }

    pub fn simplex(&self, x: f32, y: f32, z: f32) -> f32 {
        let x = map_range(x, 0.0, self.width, -1.0, 1.0) as f64;
        let y = map_range(y, 0.0, self.height, -1.0, 1.0) as f64;
        self.noise_fn.get([x, y, z as f64]) as f32
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.rng = Pcg64::seed_from_u64(seed);
    }

    pub fn set_noise_scale(&mut self, scale: f32) {
        self.noise_scale = scale;
    }

    pub fn set_noise_fn<N: NoiseFn<[f64; 3]> + 'static>(&mut self, nf: N) {
        self.noise_fn = Box::new(nf)
    }

    pub fn set_colors(&mut self, img: Pixmap, n: usize) {
        let w = img.width();
        let h = img.height();
        for _ in 0..n {
            let i = self.rand_range(0.0, w as f32) as u32;
            let j = self.rand_range(0.0, h as f32) as u32;
            let p = img.pixel(i, j).unwrap();
            let r = p.red();
            let g = p.green();
            let b = p.blue();
            let c = Color::from_rgba8(r, g, b, 255);
            self.colors.push(c);
        }
        self.colors.sort_by_cached_key(|c| {
            (1000.0 * (c.red() * c.red() + c.green() * c.green() + c.blue() * c.blue())) as u32
        })
    }

    pub fn color(&self, i: usize) -> Color {
        self.colors[i]
    }

    pub fn colors(&self) -> Vec<Color> {
        self.colors.clone()
    }

    pub fn rand_color(&mut self) -> Color {
        let n = self.colors.len();
        let i = self.rand_range(0, n);
        self.colors[i]
    }

    pub fn width_n(&self) -> u32 {
        self.width as u32
    }

    pub fn height_n(&self) -> u32 {
        self.height as u32
    }

    pub fn center(&self) -> Point {
        pt2(self.width / 2.0, self.height / 2.0)
    }

    pub fn rand_range<T: SampleUniform + PartialOrd>(&mut self, low: T, high: T) -> T {
        self.rng.gen_range(low..high)
    }

    pub fn noise(&self, x: f32, y: f32, z: f32) -> f32 {
        self.noise_fn.get([
            (self.noise_scale * x) as f64,
            (self.noise_scale * y) as f64,
            (self.noise_scale * z) as f64,
        ]) as f32
    }

    pub fn curl(&self, x: f32, y: f32, z: f32) -> f32 {
        curl(|x, y| self.noise(x, y, z) * TAU, x, y, EPSILON)
    }

    pub fn rand_rgb(&mut self) -> Color {
        let l: f32 = self.rand_range(0.0, 100.0);
        let a: f32 = self.rand_range(-128.0, 127.0);
        let b: f32 = self.rand_range(-128.0, 127.0);
        let rgb: Srgb = Lab::new(l, a, b).convert_into();
        let c = rgb.into_components();
        Color::from_rgba(c.0, c.1, c.2, 1.0).unwrap()
    }

    pub fn rand_rgba(&mut self) -> Color {
        let l: f32 = self.rand_range(0.0, 100.0);
        let a: f32 = self.rand_range(-128.0, 127.0);
        let b: f32 = self.rand_range(-128.0, 127.0);
        let o: f32 = self.rand_range(0.5, 1.0);
        let rgba: Srgba = Laba::new(l, a, b, o).convert_into();
        let c = rgba.into_components();
        Color::from_rgba(c.0, c.1, c.2, c.3).unwrap()
    }
}

pub trait Fill {
    fn fill_r(&mut self, rect: Rect, paint: &Paint) -> Option<()>;
    fn fill_p(&mut self, path: &Path, paint: &Paint) -> Option<()>;
}

pub trait Strk {
    fn stroke_p(&mut self, path: &Path, paint: &Paint, stroke: &Stroke) -> Option<()>;
}

impl Fill for Pixmap {
    fn fill_r(&mut self, rect: Rect, paint: &Paint) -> Option<()> {
        self.fill_rect(rect, paint, Transform::identity(), None)
    }
    fn fill_p(&mut self, path: &Path, paint: &Paint) -> Option<()> {
        self.fill_path(path, paint, FillRule::Winding, Transform::identity(), None)
    }
}

impl Strk for Pixmap {
    fn stroke_p(&mut self, path: &Path, paint: &Paint, stroke: &Stroke) -> Option<()> {
        self.stroke_path(path, paint, stroke, Transform::identity(), None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dims() {
        let w = Wassily::new(10.5, 20.6);
        assert_eq!(w.width, 10.5);
        assert_eq!(w.height, 20.6);
        assert_eq!(w.width_n(), 10);
        assert_eq!(w.height_n(), 20);
        assert_eq!(w.center(), pt2(5.25, 10.3));
    }

    #[test]
    fn rand() {
        let mut w = Wassily::new(1000.0, 800.0);
        assert_eq!(w.rand_range(0.0, 10.0), 8.315847);
    }

    #[test]
    fn simplex() {
        let w = Wassily::new(1000.0, 800.0);
        assert_eq!(w.noise(100.0, 100.0, 0.0), 0.29744336);
    }
}
