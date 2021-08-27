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
use crate::util::{TAU, Rand};
use noise::{MultiFractal, NoiseFn, Seedable};
use num_traits::{AsPrimitive, ToPrimitive};

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

    /// The noise value expressed in radians.
    pub fn angle(&self, x: f32, y: f32) -> f32 {
        self.get(x, y) % TAU
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

// Gabor Noise
const PI: f64 = std::f64::consts::PI;

fn gabor(k: f64, r: f64, f0: f64, omega: f64, x: f64, y: f64) -> f64 {
    let guass = k * (-PI / (r * r) * ((x * x) + (y * y))).exp();
    let sin = (2.0 * PI * f0 * (x * omega.cos() + y * omega.sin())).sin();
    guass * sin
}

pub struct Gabor {
    k: f64,
    r: f64,
    f0: f64,
    omega0: Option<f64>,
    kernel_radius: f64,
    impulse_density: f64,
    scale: f64,
}

impl Default for Gabor {
    fn default() -> Self {
        Self::new(1.0, 64.0, 0.01, None, 64.0)
    }
}

impl Gabor {
    pub fn new(k: f64, r: f64, f0: f64, omega0: Option<f64>, impulses_per_kernel: f64) -> Self {
        let kernel_radius = (-(0.05f64).ln() / PI).sqrt() * r;
        let impulse_density = impulses_per_kernel / (PI * kernel_radius * kernel_radius);
        let integral_gabor_filter_squared =
            0.25 * k * k * r * r * (1.0 + (-2.0 * PI * f0 * f0 * r * r).exp());
        let scale = 3.0 * (impulse_density * integral_gabor_filter_squared).sqrt();
        Self {
            k,
            r,
            f0,
            omega0,
            kernel_radius,
            impulse_density,
            scale,
        }
    }

    pub fn k(self, k: f64) -> Self {
        Self {k, ..self}
    }

    pub fn r(self, r: f64) -> Self {
        let kernel_radius = (-(0.05f64).ln() / PI).sqrt() * r;
        Self {r, kernel_radius, ..self}
    }

    pub fn a(self, a: f64) -> Self {
        let r = 1.0 / a;
        let kernel_radius = (-(0.05f64).ln() / PI).sqrt() * r;
        Self {r, kernel_radius, ..self}
    }

    pub fn omega0(self, omega0: Option<f64>) -> Self {
        Self {omega0, ..self}
    }

    pub fn get(&self, x: f64, y: f64) -> f64 {
        let x = x / self.kernel_radius;
        let y = y / self.kernel_radius;
        let int_x = x.floor();
        let int_y = y.floor();
        let frac_x = x - int_x;
        let frac_y = y - int_y;
        let i = int_x as i32;
        let j = int_y as i32;
        let mut ns = 0.0;
        for di in -1..=1 {
            for dj in -1..=1 {
                ns += self.cell(i + di, j + dj, frac_x - di as f64, frac_y - dj as f64);
            }
        }
        ns / self.scale
    }

    fn cell(&self, i: i32, j: i32, x: f64, y: f64) -> f64 {
        let mut rnd = Rand::new((i << 32 + j) as u64);
        let impulses_per_cell = self.impulse_density * self.kernel_radius * self.kernel_radius;
        let mut noise = 0.0;
        for _ in 0..impulses_per_cell as u32 {
            let xi = rnd.rand_range(0.0, 1.0);
            let yi = rnd.rand_range(0.0, 1.0);
            let wi: f64 = rnd.rand_rademacher();
            let omega0i: f64;
            if let Some(o) = self.omega0 {
                omega0i = o;
            } else {
                omega0i = rnd.rand_range(0.0, 2.0 * PI);
            }
            let xix = x - xi;
            let yiy = y - yi;
            if xix * xix + yiy * yiy < 1.0 {
                noise += wi
                    * gabor(
                        self.k,
                        self.r,
                        self.f0,
                        omega0i,
                        xix * self.kernel_radius,
                        yiy * self.kernel_radius,
                    );
            }
        }
        noise
    }
}

impl NoiseFn<f64, 2> for Gabor {
    fn get(&self, point: [f64; 2]) -> f64 {
        self.get(point[0], point[1])
    }
}