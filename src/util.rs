use rand::{Rng, SeedableRng};
use rand_distr::uniform::SampleUniform;
use rand_pcg::Pcg64;

pub const TAU: f32 = std::f32::consts::TAU;
pub const PI: f32 = std::f32::consts::PI;

pub struct Rand {
    rng: Pcg64,
}

impl Rand {
    pub fn new(seed: u64) -> Self {
        let rng = Pcg64::seed_from_u64(seed);
        Self { rng }
    }

    pub fn rand_range<U: SampleUniform + PartialOrd>(&mut self, low: U, high: U) -> U {
        self.rng.gen_range(low..high)
    }
}

pub fn map_range(x: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (x - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
}

pub fn curl(f: impl Fn(f32, f32) -> f32, x: f32, y: f32, eps: f32) -> f32 {
    let x0 = x - eps;
    let x1 = x + eps;
    let y0 = y - eps;
    let y1 = y + eps;
    let dfdx = (f(x1, y) - f(x0, y)) / (2.0 * eps);
    let dfdy = (f(x, y1) - f(x, y0)) / (2.0 * eps);
    dfdy.atan2(-dfdx)
}
