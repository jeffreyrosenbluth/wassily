use crate::prelude::*;
use noise::OpenSimplex;
use rand::prelude::*;
use rand_distr::{Distribution, Normal};

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
            .set_noise_scales(10.0, 10.0, 1.0);
        let v: Vector = (self.end - self.start).normalize();
        let n: Vector = vec2(v.y, -v.x); // n . v == 0, n is the normal.
        let mut rng = thread_rng();
        let normal = Normal::new(0.0, self.stdev).unwrap();
        let length = ((self.end.x - self.start.x).powi(2) + (self.end.y - self.start.y).powi(2))
            .sqrt() as u32;
        let c = RGBA::new(self.color.r, self.color.g, self.color.b, 0.20);
        for t in 0..length {
            let t = t as f32;
            let p: Point = point2(self.start.x + t * v.x, self.start.y + t * v.y);
            let nx = ns.noise(p.x, p.y, 0.0);
            let ny = ns.noise(p.x, p.y, 10.3711);
            for _ in 0..self.weight {
                let r = normal.sample(&mut rng);
                let q = point2(p.x + r * n.x + nx, p.y + r * n.y + ny);
                let dot = ShapeBuilder::new()
                    .rect_xywh(q, point2(1.0, 1.0))
                    .no_stroke()
                    .fill_color(c)
                    .build();
                dot.draw(canvas);
            }
        }
    }
}