use crate::canvas::Canvas;
use crate::prelude::paint_solid;
use crate::{
    prelude::{Algebra, pt},
    noises::*,
};
use noise::OpenSimplex;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use rand_distr::{Distribution, Normal};
use tiny_skia::{Color, Point};

pub struct SandLine {
    pub start: Point,
    pub end: Point,
    g: f32,
    rng: SmallRng,
    grains: u32,
    thickness: f32,
    color: Color,
}

impl SandLine {
    pub fn new(start: Point, end: Point, seed: u64) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);
        let g = rng.gen_range(0.05..0.95);
        Self {
            start,
            end,
            g,
            rng,
            grains: 64,
            thickness: 40.0,
            color: Color::BLACK,
        }
    }

    pub fn grains(mut self, grains: u32) -> Self {
        self.grains = grains;
        self
    }

    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn draw(&mut self, canvas: &mut Canvas) {
        let v: Point = self.end - self.start;
        let n: Point = (pt(v.y, -v.x)).normalize(); // n . v == 0, n is the normal.
        let length = v.mag();
        for t in 0..length as u32 {
            let t = t as f32 / length;
            let x = self.start.x + t * v.x;
            let y = self.start.y + t * v.y;
            self.g += self.rng.gen_range(-0.05..0.05);
            // clamp g to 0..1 with a 0.05 in the opposite direction
            if self.g < 0.0 {
                self.g = 0.05
            }
            if self.g > 1.0 {
                self.g = 0.95
            }
            let w = self.g / (self.grains - 1) as f32;
            let mut delta = 1.0;
            for i in 0..self.grains {
                let a = 0.1 - i as f32 / (10.0 * self.grains as f32);
                let x = (x + delta * n.x * self.thickness / 2.0 * (i as f32 * w))
                    .clamp(0.0, canvas.width() as f32 - 1.0);
                let y = (y + delta * n.y * self.thickness / 2.0 * (i as f32 * w))
                    .clamp(0.0, canvas.height() as f32 - 1.0);
                delta *= -1.0;
                let mut color = self.color;
                color.set_alpha(a);
                let paint = paint_solid(color);
                canvas.fill_rect(x, y, 1.0, 1.0, &paint)
            }
        }
    }
}

pub struct DotLine {
    pub start: Point,
    pub end: Point,
    noise_strength: f32,
    rng: SmallRng,
    stdev: f32,
    color: Color,
    weight: u32,
}

impl DotLine {
    pub fn new(start: Point, end: Point, seed: u64) -> Self {
        Self {
            start,
            end,
            noise_strength: 20.0,
            rng: SmallRng::seed_from_u64(seed),
            stdev: 1.0,
            color: Color::BLACK,
            weight: 25,
        }
    }

    pub fn noise_strength(mut self, strength: f32) -> Self {
        self.noise_strength = strength;
        self
    }

    pub fn stdev(mut self, stdev: f32) -> Self {
        self.stdev = stdev;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self
    }

    pub fn draw(&mut self, canvas: &mut Canvas) {
        let noise_opts = NoiseOpts::new(
            canvas.width() as f32,
            canvas.height() as f32,
            1.0,
            1.0,
            1.0,
            self.noise_strength,
        );
        let nf = OpenSimplex::default();
        let v: Point = self.end - self.start;
        let n: Point = (pt(v.y, -v.x)).normalize(); // n . v == 0, n is the normal.
        let normal = Normal::new(0.0, self.stdev).unwrap();
        let length = v.mag();
        let mut c = self.color;
        for t in 0..length as u32 {
            let t = t as f32 / length;
            let p = pt(self.start.x + t * v.x, self.start.y + t * v.y);
            let nx = noise3d(nf, &noise_opts, p.x, p.y, 0.0);
            let ny = noise3d(nf, &noise_opts, p.x, p.y, 10.3711);
            for _ in 0..self.weight {
                let r = normal.sample(&mut self.rng);
                let mut a = 1.0 / (20.0 + r.abs());
                a = a.clamp(0.0, 1.0);
                c.set_alpha(a);
                let q = pt(p.x + r * n.x + nx, p.y + r * n.y + ny);
                canvas.dot(q.x, q.y, c);
            }
        }
    }
}
