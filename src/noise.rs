use crate::Point;
use noise::{MultiFractal, NoiseFn, Seedable};
use std::marker::PhantomData;

pub struct Noise<N, T>
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
    phantom: PhantomData<N>,
}

impl<N, T> Noise<N, T>
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
            phantom: PhantomData,
        }
    }

    pub fn set_noise_scales(&mut self, x_scale: f32, y_scale: f32, z_scale: f32) {
        self.x_scale = x_scale;
        self.y_scale = y_scale;
        self.z_scale = z_scale;
    }

    pub fn set_noise_scale(&mut self, scale: f32) {
        self.x_scale = scale;
        self.y_scale = scale;
        self.z_scale = scale;
    }

    pub fn set_noise_fn(&mut self, nf: T) {
        self.noise_fn = nf;
    }

    pub fn set_noise_factor(&mut self, f: f32) {
        self.noise_factor = f;
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

impl<T> Noise<[f64; 2], T>
where
    T: NoiseFn<[f64; 2]>,
{
    pub fn noise(&self, x: f32, y: f32) -> f32 {
        let center = self.center();
        self.noise_factor
            * self.noise_fn.get([
                (1.0 / center.x * self.x_scale * (x - center.x)) as f64,
                (1.0 / center.y * self.y_scale * (y - center.y)) as f64,
            ]) as f32
    }
}

impl<T> Noise<[f64; 3], T>
where
    T: NoiseFn<[f64; 3]>,
{
    pub fn noise(&self, x: f32, y: f32, z: f32) -> f32 {
        let center = self.center();
        self.noise_factor
            * self.noise_fn.get([
                (1.0 / center.x * self.x_scale * (x - center.x)) as f64,
                (1.0 / center.y * self.y_scale * (y - center.y)) as f64,
                (self.z_scale * z) as f64,
            ]) as f32
    }
}

impl<N, T> Noise<N, T>
where
    T: NoiseFn<N> + Seedable + Clone,
{
    pub fn noise_seed(&self) -> u32 {
        self.noise_fn.seed()
    }

    pub fn set_noise_seed(&mut self, seed: u32) {
        let nf = self.noise_fn.clone().set_seed(seed);
        self.set_noise_fn(nf);
    }
}

impl<N, T> Noise<N, T>
where
    T: NoiseFn<N> + MultiFractal + Clone,
{
    pub fn set_octaves(&mut self, o: usize) {
        let nf = self.noise_fn.clone().set_octaves(o);
        self.set_noise_fn(nf);
    }

    pub fn set_frequency(&mut self, f: f64) {
        let nf = self.noise_fn.clone().set_frequency(f);
        self.set_noise_fn(nf);
    }

    pub fn set_persistence(&mut self, p: f64) {
        let nf = self.noise_fn.clone().set_persistence(p);
        self.set_noise_fn(nf);
    }

    pub fn set_lacunarity(&mut self, l: f64) {
        let nf = self.noise_fn.clone().set_lacunarity(l);
        self.set_noise_fn(nf);
    }
}
