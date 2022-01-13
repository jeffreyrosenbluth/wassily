use crate::util::*;
use noise::NoiseFn;

pub fn prf(seed: u32, x: u32) -> f64 {
    let t = format!("{},{}", seed, x);
    let h = calculate_hash(t) % 1_000_000_000;
    (h + 1) as f64 / 1_000_000_001 as f64
}

pub fn prf2(seed: u32, x: u32, y: u32) -> f64 {
    let t = format!("{},{},{}", seed, x, y);
    let h = calculate_hash(t) % 1_000_000_000;
    (h + 1) as f64 / 1_000_000_001 as f64
}

pub fn box_muller(seed: u32, x: u32, y: u32) -> (f64, f64) {
    let u1 = prf2(seed, x, y);
    let u2 = prf2(seed + 1, x, y);
    let r = (-2.0 * u1.ln()).sqrt();
    (r * (u2 * std::f64::consts::TAU).cos(), r * (u2 * std::f64::consts::TAU).sin())
}

pub fn normal2(seed: u32, mean: f64, std: f64, x: u32, y: u32) -> (f64, f64) {
    let (dx, dy) = box_muller(seed, x, y);
    (mean + std * dx, mean + std * dy)
}

pub fn normal_xy(seed: u32, x: u32, y: u32) -> f64 {
    let (a, b) = box_muller(seed, x, y);
    (a + b) / std::f64::consts::SQRT_2    
}

pub struct White {
    factor: f64,
}

impl White {
    pub fn new(factor: f64) -> Self {
        Self { factor }
    }
}

impl Default for White {
    fn default() -> Self {
        Self { factor: 1.0 }
    }
}

impl NoiseFn<f64, 2> for White {
    fn get(&self, point: [f64; 2]) -> f64 {
        prf2(8765321, point[0] as u32, point[1] as u32) * self.factor
    }
}

pub struct Guassian {
    mean: f64,
    std: f64,
}

impl Guassian {
    pub fn new(mean: f64, std: f64) -> Self {
        Self { mean, std }
    }
}

impl Default for Guassian {
    fn default() -> Self {
        Self {
            mean: 0.0,
            std: 1.0,
        }
    }
}

impl NoiseFn<f64, 2> for Guassian {
    fn get(&self, point: [f64; 2]) -> f64 {
        normal_xy(8765321, point[0] as u32, point[1] as u32) * self.std + self.mean
    }
}