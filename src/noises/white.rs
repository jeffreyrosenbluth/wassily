use crate::util::*;
use noise::NoiseFn;

/// Stateless pseudrandom number generators. These are often usefull when
/// parallelizing an algorithm where you want to avoid mutable state.
/// See https://github.com/Lokathor/randomize and
/// https://lokathor.github.io/prng/
const PCG_MULTIPLIER: u64 = 6364136223846793005;
const DEFAULT_PCG_INC: u64 = 15726070495360670683;

pub const DEFAULT_PCG_SEED: u64 = 9024823012282619035;

fn rng_u32_to_f32(value: u32) -> f32 {
    let scale = 1.0 / ((1_u32 << 24) as f32);
    let value = value >> 8;
    scale * value as f32
}

fn rng_u64_to_f64(value: u64) -> f64 {
    let scale = 1.0 / ((1_u64 << 53) as f64);
    let value = value >> 11;
    scale * value as f64
}

pub fn lcg(state: u64) -> u64 {
    state
        .wrapping_mul(PCG_MULTIPLIER)
        .wrapping_add(DEFAULT_PCG_INC as u64)
}

pub fn xsh_rr_u64_to_u32(state: u64) -> u32 {
    ((((state >> 18) ^ state) >> 27) as u32).rotate_right((state >> 59) as u32)
}

pub fn pcg_u32(state: u64) -> (u64, u32) {
    let state = if state == 0 { DEFAULT_PCG_SEED } else { state };
    (lcg(state), xsh_rr_u64_to_u32(state))
}

pub fn pcg_01(state: u64) -> (u64, f32) {
    let (state, u) = pcg_u32(state);
    (state, rng_u32_to_f32(u))
}

pub fn pcg_range(state: u64, lower: f32, upper: f32) -> (u64, f32) {
    let w = upper - lower;
    let (state, r) = pcg_01(state);
    (state, lower + w * r)
}

const FNV_PRIME: u32 = 16777619;
const FNV_OFFSET: u32 = 2166136261;

pub fn fnv1a(n: u32) -> u32 {
    let mut hash = FNV_OFFSET;
    let bytes = n.to_be_bytes();
    for b in bytes {
        hash ^= b as u32;
        hash *= FNV_PRIME;
    }
    hash
}

pub fn fnv_01(n: u32) -> f32 {
    let hash = fnv1a(n);
    rng_u32_to_f32(hash)
}

// Pseudorandom functions

pub fn prf(seed: u32, x: u32) -> f64 {
    let t = format!("{},{}", seed, x);
    let h = calculate_hash(t) % 1_000_000_000;
    (h + 1) as f64 / 1_000_000_001_f64
}

pub fn prf2(seed: u32, x: u32, y: u32) -> f64 {
    let t = format!("{},{},{}", seed, x, y);
    let h = calculate_hash(t) % 1_000_000_000;
    (h + 1) as f64 / 1_000_000_001.0
}

pub fn box_muller(seed: u32, x: u32, y: u32) -> (f64, f64) {
    let u1 = prf2(seed, x, y);
    let u2 = prf2(seed + 1, x, y);
    let r = (-2.0 * u1.ln()).sqrt();
    (
        r * (u2 * std::f64::consts::TAU).cos(),
        r * (u2 * std::f64::consts::TAU).sin(),
    )
}

pub fn normal2(seed: u32, mean: f64, std: f64, x: u32, y: u32) -> (f64, f64) {
    let (dx, dy) = box_muller(seed, x, y);
    (mean + std * dx, mean + std * dy)
}

pub fn normal_xy(seed: u32, x: u32, y: u32) -> f64 {
    let (a, b) = box_muller(seed, x, y);
    (a + b) / std::f64::consts::SQRT_2
}

/// White noise
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
        prf2(524287, point[0] as u32, point[1] as u32) * self.factor
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
        normal_xy(524287, point[0] as u32, point[1] as u32) * self.std + self.mean
    }
}
