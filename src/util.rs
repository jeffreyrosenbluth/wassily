use crate::prelude::{point2, Point};
use rand::{Rng, SeedableRng};
use rand_distr::uniform::SampleUniform;
use rand_pcg::Pcg64;
use num_traits::AsPrimitive;

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

pub fn halton(index: u32, base: u32) -> f32 {
    let mut f = 1.0;
    let mut r = 0.0;
    let mut index = index;
    let b = base as f32;
    while index > 0 {
        f /= b;
        r += f * (index % base) as f32;
        index /= base;
    }
    r
}

pub fn stipple<T: AsPrimitive<f32>>(width: T, height: T, n: u32) -> Vec<Point> {
    let xs = (100..n + 99).map(|i| halton(i, 2));
    let ys = (100..n + 99).map(|i| halton(i, 3));
    xs.zip(ys)
        .map(|p| point2(p.0 * width.as_(), p.1 * height.as_()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn halton_2() {
        assert_eq!(halton(1, 2), 0.5);
        assert_eq!(halton(3, 2), 0.75);
        assert_eq!(halton(6, 2), 0.375);
    }

    #[test]
    fn halton_3() {
        assert_eq!(halton(1, 3), 1.0 / 3.0);
        assert_eq!(halton(3, 3), 1.0 / 9.0);
        assert_eq!(halton(6, 3), 2.0 / 9.0);
    }
}
