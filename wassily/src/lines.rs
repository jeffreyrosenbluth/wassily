use crate::core::canvas::Canvas;
use crate::kolor::Colorful;
use crate::prelude::WHITE;
use crate::{
    curves::ParametricPath,
    noises::*,
    prelude::{paint_solid, pt, Algebra},
    core::shape::Shape,
    stipple::halton,
};
use noise::OpenSimplex;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rand_distr::{Distribution, Normal};
use tiny_skia::{Color, Point};

pub struct FadeLine {
    pub start: Point,
    pub end: Point,
    pub color: Color,
    pub thickness: f32,
    pub subdivisions: u32,
    pub min_opacity: f32,
    pub max_opacity: f32,
    rng: SmallRng,
}

/*
```rust
fn main() {
    let width = 1080;
    let height = 1080;
    let linecolor = *WHITE;
    let mut canvas = Canvas::new(width, height);
    canvas.fill(rgb8(0, 0, 0));
    canvas.fill(rgb8(1, 8, 48));
    for i in (0..width).step_by(10) {
        let mut fl = FadeLine::new(pt(i, 0), pt(i, height), i as u64);
        fl = fl.thickness(0.20).color(linecolor);
        fl.draw(&mut canvas);
    }
    for i in (0..height).step_by(10) {
        let mut fl = FadeLine::new(pt(0, i), pt(width, i), 1137 + i as u64);
        fl = fl.thickness(0.20).color(linecolor);
        fl.draw(&mut canvas);
    }
    canvas.save_png("./blue.png");
}
````
 */
impl FadeLine {
    pub fn new(start: Point, end: Point, seed: u64) -> Self {
        let color = *WHITE;
        let thickness = 1.0;
        let subdivisions = 25;
        let min_opacity = 0.1;
        let max_opacity = 0.9;
        let rng = SmallRng::seed_from_u64(seed);
        Self {
            start,
            end,
            color,
            thickness,
            subdivisions,
            min_opacity,
            max_opacity,
            rng,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn subdivisions(mut self, subdivisions: u32) -> Self {
        self.subdivisions = subdivisions;
        self
    }

    pub fn min_opacity(mut self, min_opacity: f32) -> Self {
        self.min_opacity = min_opacity;
        self
    }

    pub fn max_opacity(mut self, max_opacity: f32) -> Self {
        self.max_opacity = max_opacity;
        self
    }

    pub fn draw(&mut self, canvas: &mut Canvas) {
        let k: u32 = self.rng.random();
        let mut ts: Vec<f32> = (0..self.subdivisions).map(|i| halton(i + k, 2)).collect();
        ts.sort_by(|a, b| a.partial_cmp(b).unwrap());
        ts[0] = 0.0;
        ts[self.subdivisions as usize - 1] = 1.0;
        let pp = ParametricPath::new(vec![self.start, self.end]);
        for t in ts.windows(2) {
            let ps = pp.section(t[0], t[1]);
            let c = self
                .color
                .opacity(self.rng.random_range(self.min_opacity..self.max_opacity));
            Shape::new()
                .points(&ps)
                .no_fill()
                .stroke_weight(self.thickness)
                .stroke_color(c)
                .draw(canvas);
        }
    }
}

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
        let g = rng.random_range(0.05..0.95);
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
            self.g += self.rng.random_range(-0.05..0.05);
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
                Shape::new()
                    .rect_xywh(pt(x, y), pt(1, 1))
                    .fill_paint(&paint)
                    .draw(canvas);
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
