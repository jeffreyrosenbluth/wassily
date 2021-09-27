//! The `noise` module is a wrapper around the *excellent* [noise] crate that provides a
//! convenient API for using coherent noise in 2d artworks.
//! Noise remembers the width and height of the canvas and uses the natural
//! scaling of `x_scale / width` and `y_scale / height`. It also handles converting
//! input and output variables to `f32` which is consistent with most rendering
//! backends.
//!
//! # Example
//!
//! ```rust
//! use noise::Fbm;
//! use wassily::prelude::Noise;
//!
//! let ns = Noise::<_, 2>::new(800, 600, Fbm::default())
//!     .scales(4.0) // Set the x and y scales to 4.0.
//!     .factor(5.0) // Set the noise factor to 5.0.
//!     .seed(1) // For Seedable noise functions you can set the seed.
//!     .octaves(4) // For Multifractal noise functions you can set octaves.
//!     .frequency(3.0) // For Multifractal noise functions you can set frequency.
//!     .lacunarity(4.0) // For Multifractal noise functions you can set lacunarity.
//!     .prersistence(1.0) // For Multifractal noise functions you can set persistence.
//! let z = ns.get(400.0, 300.0);
//! ```

use crate::prelude::Point;
use crate::util::TAU;
use noise::{MultiFractal, NoiseFn, Seedable};
use num_traits::{AsPrimitive, ToPrimitive};

pub mod gabor;

/// the `Noise` struct fixes the output type as `f64` as at the time of this
/// writing all of the noise functions are `f64` and sets the scaling
/// parameters for the noise function.
#[derive(Copy, Clone)]
pub struct Noise<T, const N: usize>
where
    T: NoiseFn<f64, N>,
{
    /// The width of the noise domain.
    pub width: f32,
    /// The height of the noise domain.
    pub height: f32,
    noise_fn: T,
    x_scale: f32,
    y_scale: f32,
    z_scale: f32,
    noise_factor: f32,
}

impl<T, const N: usize> Noise<T, N>
where
    T: NoiseFn<f64, N>,
{
    pub fn new<U: AsPrimitive<f32>>(width: U, height: U, noise_fn: T) -> Self {
        let x_scale = 1.0;
        let y_scale = 1.0;
        let z_scale = 1.0;
        let noise_factor = 1.0;
        let width = width.as_();
        let height = height.as_();
        Self {
            width,
            height,
            noise_fn,
            x_scale,
            y_scale,
            z_scale,
            noise_factor,
        }
    }

    /// Set the noise function from the [noise] crate.
    pub fn noise_fn(self, noise_fn: T) -> Self {
        Self { noise_fn, ..self }
    }

    /// Multiplier for the noise value.
    pub fn factor(self, noise_factor: f32) -> Self {
        Self {
            noise_factor,
            ..self
        }
    }

    /// Used to scale the x-coordinate: `x = x_scale * x / width`.
    pub fn x_scale(self, x_scale: f32) -> Self {
        Self { x_scale, ..self }
    }

    /// Used to  the y-coordinate: `y = y_scale * y / height`.
    pub fn y_scale(self, y_scale: f32) -> Self {
        Self { y_scale, ..self }
    }

    /// Used ot scale the z-coordingate: z = z_scale * z.
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

    // Helper function to find the center of the domain.
    fn center(&self) -> Point {
        Point::new(self.width / 2.0, self.height / 2.0)
    }

    // helper function to convert inputs and outputs to f32.
    fn get_f32(&self, point: [f32; N]) -> f32 {
        let coords = point.iter().map(|p| p.to_f64());
        let mut a: [f64; N] = [0.0; N];
        for (i, c) in coords.enumerate() {
            a[i] = c.unwrap();
        }
        self.noise_fn.get(a) as f32
    }
}

impl<T> Noise<T, 2>
where
    T: NoiseFn<f64, 2>,
{
    /// The value of the noise function at the specified coordinates, 2d.
    pub fn get(&self, x: f32, y: f32) -> f32 {
        let center = self.center();
        // Perhaps refactor to use 'ScalePoint' from noise module
        self.noise_factor
            * self.get_f32([
                (1.0 / center.x * self.x_scale * (x - center.x)),
                (1.0 / center.y * self.y_scale * (y - center.y)),
            ])
    }
}

impl<T> Noise<T, 3>
where
    T: NoiseFn<f64, 3>,
{
    /// The value of the noise function at the specified coordinates, 3d.
    pub fn get(&self, x: f32, y: f32, z: f32) -> f32 {
        let center = self.center();
        self.noise_factor
            * self.get_f32([
                (1.0 / center.x * self.x_scale * (x - center.x)),
                (1.0 / center.y * self.y_scale * (y - center.y)),
                (self.z_scale * z),
            ])
    }

    /// The noise value expressed in radians.
    pub fn angle(&self, x: f32, y: f32, z: f32) -> f32 {
        self.get(x, y, z) % TAU
    }
}

impl<T, const N: usize> Noise<T, N>
where
    T: NoiseFn<f64, N> + Seedable,
{
    /// Set the seed for noise functions that are `Seedable`.
    pub fn seed(self, seed: u32) -> Self {
        Self {
            noise_fn: self.noise_fn.set_seed(seed),
            ..self
        }
    }
}

impl<T, const N: usize> Noise<T, N>
where
    T: NoiseFn<f64, N> + MultiFractal,
{
    /// Set number of octaves.
    pub fn octaves(self, octaves: usize) -> Self {
        Self {
            noise_fn: self.noise_fn.set_octaves(octaves),
            ..self
        }
    }

    /// Set frequency.
    pub fn frequency(self, frequency: f64) -> Self {
        Self {
            noise_fn: self.noise_fn.set_frequency(frequency),
            ..self
        }
    }

    /// Set Lacunarity.
    pub fn lacunarity(self, lacunarity: f64) -> Self {
        Self {
            noise_fn: self.noise_fn.set_lacunarity(lacunarity),
            ..self
        }
    }

    /// Set persistence.
    pub fn persistence(self, persistence: f64) -> Self {
        Self {
            noise_fn: self.noise_fn.set_persistence(persistence),
            ..self
        }
    }
}

