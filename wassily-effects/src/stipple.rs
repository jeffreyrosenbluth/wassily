//! # Low-Discrepancy Sampling
//!
//! Advanced sampling algorithms for generating superior point distributions.
//! This module provides low-discrepancy sequences that create more uniform
//! point distributions than pseudo-random sampling, essential for high-quality
//! stippling, sampling, and procedural generation.
//!
//! ## Key Algorithms
//!
//! - **Halton Sequence**: Classic low-discrepancy sequence for 2D sampling
//! - **Uniform Distribution**: Traditional pseudo-random sampling for comparison
//! - **Specialized Samplers**: Optimized for specific generative art applications
//!
//! ## Low-Discrepancy vs Random
//!
//! Low-discrepancy sequences provide better space-filling properties than
//! pseudo-random sampling:
//!
//! - **More Uniform**: Better distribution across the sampling space
//! - **Less Clumping**: Avoids the clustering typical of random sampling
//! - **Deterministic**: Reproducible results with same parameters
//! - **Scalable**: Quality improves with more sample points
//!
//! ## Basic Usage
//!
//! ```no_run
//! use wassily_effects::*;
//! use wassily_core::*;
//!
//! // Generate low-discrepancy points
//! let points = halton_2d(1000, 800.0, 600.0);
//!
//! // Create stipple pattern
//! let mut canvas = Canvas::new(800, 600);
//! canvas.fill(*WHITE);
//!
//! for point in points {
//!     Shape::new()
//!         .circle(point, 2.0)
//!         .fill_color(*BLACK)
//!         .draw(&mut canvas);
//! }
//! ```
//!
//! ## Applications
//!
//! - **Stippling**: High-quality dot patterns and pointillism
//! - **Sampling**: Monte Carlo integration and statistical sampling
//! - **Texture Generation**: Even distribution of texture elements
//! - **Particle Systems**: Better initial particle placement
//! - **Procedural Generation**: More natural-looking random placement
use wassily_geometry::Matrix;
use wassily_core::points::{center, pt, Algebra};
use num_traits::AsPrimitive;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::f32::consts::PI;
use tiny_skia::Point;

/// Generate a set of points using a uniform distribution.
/// This sequence is not low discrepancy.
pub fn uniform<T: AsPrimitive<f32>>(width: T, height: T, n: u32, seed: u64) -> Vec<Point> {
    let mut rng = SmallRng::seed_from_u64(seed);
    let vals: Vec<Point> = (0..n)
        .map(|_| {
            pt(
                rng.random_range(0f32..width.as_()),
                rng.random_range(0f32..height.as_()),
            )
        })
        .collect();
    vals
}

/// Generate a number from the Halton sequence.
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

/// Generate a set of points using the Halton sequence.
pub fn halton_23<T: AsPrimitive<f32>>(width: T, height: T, n: u32, seed: u64) -> Vec<Point> {
    let mut rng = SmallRng::seed_from_u64(seed);
    let k: u32 = rng.random();
    let xs = (k..n + k).map(|i| halton(i, 2));
    let ys = (k..n + k).map(|i| halton(i, 3));
    xs.zip(ys)
        .map(|p| Point::from_xy(p.0 * (width.as_() - 1.0), p.1 * (height.as_() - 1.0)))
        .collect()
}

/// Gererate a set of points using Poisson Disk sampling.
// An improvement to Bridson's Algorithm for Poisson Disc sampling.
// https://observablehq.com/@jrus/bridson-fork/2
pub fn poisson_disk(width: f32, height: f32, radius: f32, seed: u64) -> Vec<Point> {
    const K: usize = 11; // maximum number of samples before rejection
    const M: f32 = 4.0; // a number mutually prime to k
    const EPS: f32 = 0.0000001;
    let mut rng = SmallRng::seed_from_u64(seed);
    let cell_size = radius / 2f32.sqrt();
    let cols = (width / cell_size).ceil() as usize;
    let rows = (height / cell_size).ceil() as usize;
    let mut grid: Matrix<Option<Point>> = Matrix::fill(rows, cols, None);
    // let p0 = pt(rng.random_range(0.0..width), rng.random_range(0.0..height));
    let p0 = center(width, height);
    let mut active = vec![p0];
    let mut ps = vec![p0];
    let x0 = (p0.y / cell_size).floor() as usize;
    let y0 = (p0.x / cell_size).floor() as usize;
    grid[x0][y0] = Some(p0);

    let neighbors = |i: usize, j: usize| -> Vec<(usize, usize)> {
        let i = i as i32;
        let j = j as i32;
        let mut x;
        let mut y;
        let mut cells = vec![];
        for di in -1..=1 {
            x = i + di;
            if !(0..rows as i32).contains(&x) {
                continue;
            }
            for dj in -1..=1 {
                y = j + dj;
                if (0..cols as i32).contains(&y) {
                    cells.push((x as usize, y as usize));
                }
            }
        }
        cells
    };

    while !active.is_empty() {
        let mut found = false;
        let j = rng.random_range(0..active.len());
        let p = active[j];
        let seed: f32 = rng.random();
        for i in 0..K {
            let theta = 2.0 * PI * (seed + M * i as f32 / K as f32);
            let r1: f32 = radius + EPS + radius * 0.5 * rng.random::<f32>();
            let p1 = pt(p.x + r1 * theta.cos(), p.y + r1 * theta.sin());
            let xi = (p1.y / cell_size).floor() as usize;
            let yi = (p1.x / cell_size).floor() as usize;
            if neighbors(xi, yi).iter().any(|(a, b)| {
                let g = grid[*a][*b];
                g.is_some() && g.unwrap().dist2(p1) < radius * radius
            }) || p1.x < 0.0
                || p1.x >= width
                || p1.y < 0.0
                || p1.y >= height
            {
                continue;
            }
            active.push(p1);
            ps.push(p1);
            grid[xi][yi] = Some(p1);
            found = true;
            break;
        }
        if !found {
            active.remove(j);
        }
    }
    ps
}
