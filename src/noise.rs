use crate::prelude::Point;
use noise::{noise_fns::NoiseFn, MultiFractal, Seedable};

pub struct Noise<T, const N: usize>
where
    T: NoiseFn<N>,
{
    pub width: f32,
    pub height: f32,
    noise_fn: T,
    x_scale: f32,
    y_scale: f32,
    z_scale: f32,
    noise_factor: f32,
}

impl<T, const N: usize> Noise<T, N>
where
    T: NoiseFn<N>,
{
    pub fn new(width: f32, height: f32, noise_fn: T) -> Self {
        let x_scale = 1.0;
        let y_scale = 1.0;
        let z_scale = 1.0;
        let noise_factor = 2.0;
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

    pub fn set_noise_fn(self, noise_fn: T) -> Self {
        Self { noise_fn, ..self }
    }

    pub fn set_noise_factor(self, noise_factor: f32) -> Self {
        Self {
            noise_factor,
            ..self
        }
    }

    pub fn width_n(&self) -> u32 {
        self.width as u32
    }

    pub fn height_n(&self) -> u32 {
        self.height as u32
    }

    pub fn center(&self) -> Point {
        Point::new(self.width / 2.0, self.height / 2.0)
    }
}

impl<T> Noise<T, 2>
where
    T: NoiseFn<2>,
{
    pub fn noise(&self, x: f32, y: f32) -> f32 {
        let center = self.center();
        // Perhaps refactor to use 'ScalePoint' from noise module
        self.noise_factor
            * self.noise_fn.get_f32([
                (1.0 / center.x * self.x_scale * (x - center.x)),
                (1.0 / center.y * self.y_scale * (y - center.y)),
            ])
    }

    pub fn set_noise_scales(self, x_scale: f32, y_scale: f32) -> Self {
        Self {
            x_scale,
            y_scale,
            ..self
        }
    }
}

impl<T> Noise<T, 3>
where
    T: NoiseFn<3>,
{
    pub fn noise(&self, x: f32, y: f32, z: f32) -> f32 {
        let center = self.center();
        self.noise_factor
            * self.noise_fn.get_f32([
                (1.0 / center.x * self.x_scale * (x - center.x)),
                (1.0 / center.y * self.y_scale * (y - center.y)),
                (self.z_scale * z),
            ])
    }

    pub fn set_noise_scales(self, x_scale: f32, y_scale: f32, z_scale: f32) -> Self {
        Self {
            x_scale,
            y_scale,
            z_scale,
            ..self
        }
    }
}

impl<T, const N: usize> Seedable for Noise<T, N>
where
    T: NoiseFn<N> + Seedable,
{
    fn set_seed(self, seed: u32) -> Self {
        Self {
            noise_fn: self.noise_fn.set_seed(seed),
            ..self
        }
    }

    fn seed(&self) -> u32 {
        self.noise_fn.seed()
    }
}

impl<T, const N: usize> MultiFractal for Noise<T, N>
where
    T: NoiseFn<N> + MultiFractal,
{
    fn set_octaves(self, octaves: usize) -> Self {
        Self {
            noise_fn: self.noise_fn.set_octaves(octaves),
            ..self
        }
    }

    fn set_frequency(self, frequency: f64) -> Self {
        Self {
            noise_fn: self.noise_fn.set_frequency(frequency),
            ..self
        }
    }

    fn set_lacunarity(self, lacunarity: f64) -> Self {
        Self {
            noise_fn: self.noise_fn.set_lacunarity(lacunarity),
            ..self
        }
    }

    fn set_persistence(self, persistence: f64) -> Self {
        Self {
            noise_fn: self.noise_fn.set_persistence(persistence),
            ..self
        }
    }
}
