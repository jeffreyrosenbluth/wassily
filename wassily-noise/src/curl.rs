//! # Curl Noise
//!
//! Curl noise generates vector fields with fluid-like, swirling patterns by computing
//! the curl of a scalar noise field. This creates divergence-free vector fields that
//! are particularly useful for simulating fluid flow, particle systems, and organic
//! movement patterns in generative art.
//!
//! ## Key Features
//!
//! - **Fluid-like Motion**: Creates natural, swirling vector fields
//! - **Divergence-free**: Mathematically guarantees no sources or sinks
//! - **Configurable**: Adjustable epsilon value for different curl characteristics
//! - **Composable**: Works with any underlying noise function
//!
//! ## Usage
//!
//! ```no_run
//! use wassily_noise::*;
//! use noise::{Perlin, NoiseFn};
//!
//! // Create curl noise from Perlin noise
//! let perlin = Perlin::new(42);
//! let curl_noise = Curl::new(perlin).eps(0.001);
//!
//! // Use in particle systems or flow fields
//! let flow_vector = curl_noise.get([x, y]);
//! ```
//!
//! ## Applications
//!
//! - **Particle Systems**: Natural, flowing particle movement
//! - **Flow Fields**: Organic line and curve generation
//! - **Fluid Simulation**: Realistic fluid-like patterns
//! - **Organic Textures**: Natural, non-repetitive surface patterns
use noise::{NoiseFn, Seedable};
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Curl<T> {
    pub source: T,
    pub eps: f64,
}

impl<T> Curl<T> {
    /// Creates a new `Curl` noise function with the given source noise function and a default
    /// epsilon value of 0.0001.
    pub fn new(source: T) -> Self {
        Self {
            source,
            eps: 0.0001,
        }
    }

    /// Sets a custom epsilon value for the `Curl` noise function and returns a new instance.
    pub fn eps(self, eps: f64) -> Self {
        Self { eps, ..self }
    }
}

impl<T> NoiseFn<f64, 2> for Curl<T>
where
    T: NoiseFn<f64, 2>,
{
    /// Calculates the Curl noise function at a given point using the source noise function and the epsilon value.
    /// The Curl value at the given point, represented as a f64 value.
    fn get(&self, point: [f64; 2]) -> f64 {
        let x = point[0];
        let y = point[1];
        let x0 = x - self.eps;
        let x1 = x + self.eps;
        let y0 = y - self.eps;
        let y1 = y + self.eps;
        let dfdx = (self.source.get([x1, y]) - self.source.get([x0, y])) / (2.0 * self.eps);
        let dfdy = (self.source.get([x, y1]) - self.source.get([x, y0])) / (2.0 * self.eps);
        dfdy.atan2(-dfdx) / PI
    }
}

impl<T> Seedable for Curl<T>
where
    T: Seedable,
{
    fn set_seed(self, seed: u32) -> Self {
        Self {
            source: self.source.set_seed(seed),
            ..self
        }
    }

    fn seed(&self) -> u32 {
        self.source.seed()
    }
}
