use crate::base::*;
use crate::noise::*;
use crate::prelude::{vec2, Vector, BLACK};
use crate::util::Rand;
use noise::OpenSimplex;
use rand::prelude::*;
use rand_distr::{Distribution, Normal};

pub struct SandLine {
    pub start: Point,
    pub end: Point,
    g: f32,
    rando: Rand,
    grains: u32,
    thickness: f32,
    color: RGBA,
}

impl SandLine {
    pub fn new(start: Point, end: Point) -> Self {
        let seed: u64 = random();
        let mut rando = Rand::new(seed);
        let g = rando.rand_range(0.05, 0.95);
        Self {
            start,
            end,
            g,
            rando,
            grains: 64,
            thickness: 40.0,
            color: BLACK,
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

        pub fn color(mut self, color: RGBA) -> Self {
            self.color = color;
            self
        }

    pub fn draw<T: Sketch>(&mut self, canvas: &mut T) {
        let v: Vector = self.end - self.start;
        let n: Vector = vec2(v.y, -v.x).normalize(); // n . v == 0, n is the normal.
        let length = v.length();
        for t in 0..length as u32 {
            let t = t as f32 / length;
            let x = self.start.x + t * v.x;
            let y = self.start.y + t * v.y;
            self.g += self.rando.rand_range(-0.05, 0.05);
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
                let x = x + delta * n.x * self.thickness / 2.0 * (i as f32 * w);
                let y = y + delta * n.y * self.thickness / 2.0 * (i as f32 * w);
                delta *= -1.0;
                pixel(x, y, self.color.set_opacity(a), canvas);
            }
        }
    }
}

pub struct DotLine {
    pub start: Point,
    pub end: Point,
    noise_strength: f32,
    stdev: f32,
    color: RGBA,
    weight: u32,
}

impl DotLine {
    pub fn new(start: Point, end: Point) -> Self {
        Self {
            start,
            end,
            noise_strength: 20.0,
            stdev: 1.0,
            color: BLACK,
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

    pub fn color(mut self, color: RGBA) -> Self {
        self.color = color;
        self
    }

    pub fn weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self
    }

    pub fn draw<T: Sketch>(&self, canvas: &mut T) {
        let ns = Noise::<_, 3>::new(1200.0, 1200.0, OpenSimplex::default())
            .set_noise_factor(self.noise_strength)
            .set_scales(1.0);
        let v: Vector = self.end - self.start;
        let n: Vector = vec2(v.y, -v.x).normalize(); // n . v == 0, n is the normal.
        let mut rng = thread_rng();
        let normal = Normal::new(0.0, self.stdev).unwrap();
        let length = v.length();
        let c = RGBA::new(self.color.r, self.color.g, self.color.b, 1.0);
        for t in 0..length as u32 {
            let t = t as f32 / length;
            let p = point2(self.start.x + t * v.x, self.start.y + t * v.y);
            let nx = ns.noise(p.x, p.y, 0.0);
            let ny = ns.noise(p.x, p.y, 10.3711);
            for _ in 0..self.weight {
                let r = normal.sample(&mut rng);
                let mut a = 1.0 / (20.0 + r.abs());
                a = a.clamp(0.0, 1.0);
                let o = c.set_opacity(a);
                let q = point2(p.x + r * n.x + nx, p.y + r * n.y + ny);
                pixel(q.x, q.y, o, canvas);
            }
        }
    }
}
