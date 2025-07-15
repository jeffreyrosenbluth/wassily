//! # Wassily Noise
//!
//! Noise generation utilities optimized for generative art applications.
//! This crate provides ergonomic wrappers around the Rust noise library with
//! f32 support, additional noise functions, and utilities specifically designed
//! for creative coding and generative art.
//!
//! ## Key Features
//!
//! - **f32 Support**: All noise functions work with f32 instead of f64 for better performance
//! - **Scale-Independent**: Noise scaling that adapts to canvas size automatically
//! - **Generative Art Focus**: Additional noise types specifically useful for creative coding
//! - **Easy Integration**: Seamless integration with wassily's canvas and shape systems
//!
//! ## Available Noise Types
//!
//! - **Core Functions**: [`noise2d()`], [`noise3d()`] with normalized variants
//! - **[`curl`]**: Curl noise for fluid-like effects and vector fields
//! - **[`gabor`]**: Gabor noise for procedural textures and patterns
//! - **[`white`]**: White noise and pseudo-random number generation
//! - **[`sinusoid`]**: Sinusoidal noise patterns
//! - **[`img_noise`]**: Image-based noise generation
//!
//! ## Quick Start
//!
//! ```no_run
//! use wassily_noise::*;
//! use noise::{NoiseFn, Perlin};
//!
//! // Create a noise function
//! let perlin = Perlin::new(42);
//! let opts = NoiseOpts::default().scales(0.01);
//!
//! // Generate noise values
//! let value = noise2d(perlin, &opts, 100.0, 150.0);      // Range: [-1, 1]
//! let normalized = noise2d_01(perlin, &opts, 100.0, 150.0); // Range: [0, 1]
//! ```
//!
//! ## Noise Options
//!
//! The [`NoiseOpts`] struct provides fine-grained control over noise generation:
//!
//! ```no_run
//! use wassily_noise::*;
//!
//! let opts = NoiseOpts::default()
//!     .width(800.0)
//!     .height(600.0)
//!     .scales(0.005)        // Uniform scaling
//!     .factor(2.0);         // Amplitude multiplier
//! ```
use noise::NoiseFn;
use num_traits::{AsPrimitive, ToPrimitive};

pub mod curl;
pub mod gabor;
pub mod img_noise;
pub mod sinusoid;
pub mod white;

// Re-export key types and functions for convenience
pub use curl::*;
pub use gabor::*;
pub use img_noise::*;
pub use sinusoid::*;
pub use white::*;

/// Configuration options for noise generation.
///
/// `NoiseOpts` provides fine-grained control over how noise coordinates are scaled
/// and how the output is modified. This allows for canvas-size-independent noise
/// generation and consistent results across different output resolutions.
///
/// ## Scaling Behavior
///
/// - **Input Scaling**: Coordinates are scaled by `scale / (width or height)`
/// - **Output Scaling**: Results are multiplied by `factor`
/// - **Canvas Independence**: Noise patterns remain consistent regardless of canvas size
///
/// ## Example
///
/// ```no_run
/// use wassily_noise::*;
///
/// // Create noise options for a 800x600 canvas
/// let opts = NoiseOpts::default()
///     .width(800.0)
///     .height(600.0)
///     .scales(0.01)     // Fine detail
///     .factor(2.0);     // Amplify output
///
/// // Same pattern will look identical on any canvas size
/// let opts_large = NoiseOpts::default()
///     .width(1600.0)
///     .height(1200.0)
///     .scales(0.01)
///     .factor(2.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct NoiseOpts {
    pub width: f32,
    pub height: f32,
    pub x_scale: f32,
    pub y_scale: f32,
    pub z_scale: f32,
    pub factor: f32,
}

impl NoiseOpts {
    pub fn new(
        width: f32,
        height: f32,
        x_scale: f32,
        y_scale: f32,
        z_scale: f32,
        factor: f32,
    ) -> Self {
        Self {
            width,
            height,
            x_scale,
            y_scale,
            z_scale,
            factor,
        }
    }

    pub fn with_wh<T: AsPrimitive<f32>>(width: T, height: T) -> Self {
        Self {
            width: width.as_(),
            height: height.as_(),
            ..Self::default()
        }
    }

    pub fn width(self, width: f32) -> Self {
        Self { width, ..self }
    }

    pub fn height(self, height: f32) -> Self {
        Self { height, ..self }
    }

    /// Multiplier for the noise value.
    pub fn factor(self, factor: f32) -> Self {
        Self { factor, ..self }
    }

    /// Used to scale the x-coordinate: `x = x_scale * x / width`.
    pub fn x_scale(self, x_scale: f32) -> Self {
        Self { x_scale, ..self }
    }

    /// Used to scale the y-coordinate: `y = y_scale * y / height`.
    pub fn y_scale(self, y_scale: f32) -> Self {
        Self { y_scale, ..self }
    }

    /// Used to scale the z-coordinate: `z = z_scale * z`.
    pub fn z_scale(self, z_scale: f32) -> Self {
        Self { z_scale, ..self }
    }

    /// Set both the x and y scales to the same value.
    pub fn xy_scales(self, scale: f32) -> Self {
        Self {
            x_scale: scale,
            y_scale: scale,
            ..self
        }
    }

    /// Set all scales to the same value.
    pub fn scales(self, scale: f32) -> Self {
        Self {
            x_scale: scale,
            y_scale: scale,
            z_scale: scale,
            ..self
        }
    }
}

impl Default for NoiseOpts {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
            x_scale: 1.0,
            y_scale: 1.0,
            z_scale: 1.0,
            factor: 1.0,
        }
    }
}

fn get_f32<const N: usize>(nf: impl NoiseFn<f64, N>, point: [f32; N]) -> f32 {
    let coords = point.iter().map(|p| p.to_f64());
    let mut a: [f64; N] = [0.0; N];
    for (i, c) in coords.enumerate() {
        a[i] = c.unwrap();
    }
    nf.get(a) as f32
}

/// Get a f32 noise value in the range [-1.0, 1.0].
pub fn noise2d(nf: impl NoiseFn<f64, 2>, opts: &NoiseOpts, x: f32, y: f32) -> f32 {
    opts.factor
        * get_f32(
            nf,
            [
                1.0 / opts.width * opts.x_scale * x,
                1.0 / opts.height * opts.y_scale * y,
            ],
        )
}

/// Get a f32 noise value in the range [0.0, 1.0].
pub fn noise2d_01(nf: impl NoiseFn<f64, 2>, opts: &NoiseOpts, x: f32, y: f32) -> f32 {
    0.5 * noise2d(&nf, opts, x, y) + 0.5
}

/// Get a f32 noise value in the range [-1.0, 1.0].
pub fn noise3d(nf: impl NoiseFn<f64, 3>, opts: &NoiseOpts, x: f32, y: f32, z: f32) -> f32 {
    opts.factor
        * get_f32(
            nf,
            [
                1.0 / opts.width * opts.x_scale * x,
                1.0 / opts.height * opts.y_scale * y,
                opts.z_scale * z,
            ],
        )
}

/// Get a f32 noise value in the range [0.0, 1.0].
pub fn noise3d_01(nf: impl NoiseFn<f64, 3>, opts: &NoiseOpts, x: f32, y: f32, z: f32) -> f32 {
    0.5 * noise3d(&nf, opts, x, y, z) + 0.5
}