use noise::NoiseFn;
use palette::{ConvertInto, Lab, Laba, Srgb, Srgba};
use rand::prelude::*;
use rand_pcg::Pcg64;
use tiny_skia::*;

pub const TAU: f32 = std::f32::consts::TAU;
pub const PI: f32 = std::f32::consts::PI;

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

pub struct Wassily {
    pub width: f32,
    pub height: f32,
    rng: Pcg64,
    noise_fn: Box<dyn NoiseFn<[f64; 2]>>,
    noise_scale: f32,
}

impl Wassily {
    pub fn new(width: f32, height: f32) -> Self {
        let rng = Pcg64::seed_from_u64(0);
        let noise_fn = Box::new(noise::OpenSimplex::new());
        let noise_scale = 0.003;
        Self {
            width,
            height,
            rng,
            noise_fn,
            noise_scale,
        }
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.rng = Pcg64::seed_from_u64(seed);
    }

    pub fn set_noise_scale(&mut self, scale: f32) {
        self.noise_scale = scale;
    }

    pub fn set_noise_fn<N: NoiseFn<[f64; 2]> + 'static>(&mut self, nf: N) {
        self.noise_fn = Box::new(nf)
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

    pub fn rand_range(&mut self, low: f32, high: f32) -> f32 {
        self.rng.gen_range(low..high)
    }

    pub fn noise(&self, x: f32, y: f32) -> f32 {
        self.noise_fn
            .get([(self.noise_scale * x) as f64, (self.noise_scale * y) as f64]) as f32
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
        assert_eq!(w.noise(100.0, 100.0), 0.29744336);
    }
}
