use std::cell::RefCell;

use noise::NoiseFn;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

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

const FNV_PRIME_32: u32 = 16777619;
const FNV_OFFSET_32: u32 = 2166136261;
const FNV_PRIME_64: u64 = 1099511628211;
const FNV_OFFSET_64: u64 = 14695981039346656037;

pub fn fnv1a_32(n: u32) -> u32 {
    let mut hash = FNV_OFFSET_32;
    let bytes = n.to_be_bytes();
    for b in bytes {
        hash ^= b as u32;
        hash *= FNV_PRIME_32;
    }
    hash
}

pub fn fnv01_32(n: u32) -> f32 {
    let hash = fnv1a_32(n);
    rng_u32_to_f32(hash)
}

pub fn fnv1a_64(n: u64) -> u64 {
    let mut hash = FNV_OFFSET_64;
    let bytes = n.to_be_bytes();
    for b in bytes {
        hash ^= b as u64;
        hash *= FNV_PRIME_64;
    }
    hash
}

pub fn fnv01_64(n: u64) -> f64 {
    let hash = fnv1a_64(n);
    rng_u64_to_f64(hash)
}

pub fn prf(m: f64, n: f64) -> f64 {
    fn dot(a: (f64, f64), b: (f64, f64)) -> f64 {
        a.0 * b.0 + a.1 * b.1
    }
    let k1 = (127.1_f64, 311.7_f64);
    let k2 = (269.6_f64, 183.3_f64);
    let k3 = (12.9898, 78.233);
    let mn = (m, n);
    let x = dot(mn, k1);
    let y = dot(mn, k2);
    (dot((x, y), k3) * 43758.5453123).sin().fract()
}

pub fn box_muller(x: f64, y: f64) -> (f64, f64) {
    let u1 = prf(x, y);
    let u2 = prf(x + 1.0, y + 1.0);
    let r = (-2.0 * u1.ln()).sqrt();
    (
        r * (u2 * std::f64::consts::TAU).cos(),
        r * (u2 * std::f64::consts::TAU).sin(),
    )
}

pub fn normal2(mean: f64, std: f64, x: f64, y: f64) -> (f64, f64) {
    let (dx, dy) = box_muller(x, y);
    (mean + std * dx, mean + std * dy)
}

pub fn normal_xy(x: f64, y: f64) -> f64 {
    let (a, b) = box_muller(x, y);
    (a + b) / std::f64::consts::SQRT_2
}

/// White noise
pub struct White {}

impl White {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for White {
    fn default() -> Self {
        Self {}
    }
}

impl NoiseFn<f64, 2> for White {
    fn get(&self, point: [f64; 2]) -> f64 {
        prf(point[0], point[1])
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
        normal_xy(point[0], point[1]) * self.std + self.mean
    }
}
