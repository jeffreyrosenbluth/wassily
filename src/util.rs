use crate::{matrix::Matrix, prelude::BasicModel};
use chrono::prelude::Utc;
use num_traits::{AsPrimitive, FromPrimitive};
use rand::{Rng, SeedableRng};
use rand_distr::{uniform::SampleUniform, Distribution, Normal};
use rand_pcg::Pcg64;
use std::{
    collections::hash_map::DefaultHasher,
    fs::{create_dir, File},
    hash::{Hash, Hasher},
    io::Write,
    path::PathBuf,
    vec,
};
use tiny_skia::{Pixmap, Point};

pub const TAU: f32 = std::f32::consts::TAU;
pub const PI: f32 = std::f32::consts::PI;

pub fn pt<S, T>(x: S, y: T) -> Point
where
    S: AsPrimitive<f32>,
    T: AsPrimitive<f32>,
{
    Point::from_xy(x.as_(), y.as_())
}

pub fn polar<S, T>(theta: S, r: T) -> Point
where
    S: AsPrimitive<f32>,
    T: AsPrimitive<f32>,
{
    Point::from_xy(r.as_() * theta.as_().cos(), r.as_() * theta.as_().sin())
}

pub fn center<S, T>(width: S, height: T) -> Point
where
    S: AsPrimitive<f32>,
    T: AsPrimitive<f32>,
{
    Point::from_xy(width.as_() / 2.0, height.as_() / 2.0)
}

pub trait Algebra: Copy {
    fn scale(self, k: f32) -> Self;
    fn lerp(self, other: Self, t: f32) -> Self;
    fn mag_squared(self) -> f32;
    fn dist2(self, other: Self) -> f32;
    fn dot(self, other: Self) -> f32;

    fn magnitude(self) -> f32 {
        self.mag_squared().sqrt()
    }

    fn normalize(self) -> Self {
        self.scale(1.0 / self.magnitude())
    }

    fn average(self, other: Self) -> Self {
        self.lerp(other, 0.5)
    }

    fn dist(self, other: Self) -> f32 {
        self.dist2(other).sqrt()
    }
}

impl Algebra for Point {
    fn mag_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    fn scale(self, k: f32) -> Self {
        Point::from_xy(k * self.x, k * self.y)
    }

    fn lerp(self, other: Self, t: f32) -> Self {
        let x = self.x * (1.0 - t) + t * other.x;
        let y = self.y * (1.0 - t) + t * other.y;
        Self::from_xy(x, y)
    }

    fn dist2(self, other: Self) -> f32 {
        pt(self.x - other.x, self.y - other.y).mag_squared()
    }

    fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

pub fn save_sketch<T>(model: &T, canvas: &Pixmap)
where
    T: BasicModel,
{
    let ts = format!("{}", Utc::now().timestamp());
    let dir = format!(r"{}/{}/{}", model.name(), model.dir(), model.name());
    let mut sketch = PathBuf::from(format!(r"{}_{}", dir, ts));
    let _ = create_dir(model.dir());
    sketch.set_extension(model.ext());
    canvas.save_png(&sketch).expect(&format!("{:?}", &sketch));
}

pub fn save_json<T>(model: &T)
where
    T: serde::Serialize + BasicModel,
{
    let ts = format!("{}", Utc::now().timestamp());
    let dir = format!(r"{}/{}/{}", model.name(), model.dir(), model.name());
    let mut data_name = PathBuf::from(format!(r"{}_{}", dir, ts));
    let _ = create_dir(model.dir());
    data_name.set_extension("json");
    let json = serde_json::to_string_pretty(&model).expect("Could not serialize data");
    let mut output = File::create(data_name).unwrap();
    write!(output, "{}", json).unwrap();
}

pub fn calculate_hash<T: Hash>(t: T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub struct Rand {
    pub rng: Pcg64,
}

impl Rand {
    pub fn new(seed: u64) -> Self {
        let rng = Pcg64::seed_from_u64(seed);
        Self { rng }
    }

    pub fn rand_range<U: SampleUniform + PartialOrd>(&mut self, low: U, high: U) -> U {
        self.rng.gen_range(low..high)
    }

    pub fn rand_normal(&mut self, mean: f32, std_dev: f32) -> f32 {
        let normal = Normal::new(mean, std_dev).unwrap();
        normal.sample(&mut self.rng)
    }

    pub fn rand_bool(&mut self, p: f32) -> bool {
        self.rng.gen_bool(p as f64)
    }

    pub fn rand_rademacher<T: FromPrimitive>(&mut self) -> T {
        let b = self.rng.gen_bool(0.5);
        if b {
            T::from_i8(1).unwrap()
        } else {
            T::from_i8(-1i8).unwrap()
        }
    }
}

pub fn map_range(x: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (x - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
}

pub fn curl(f: impl Fn(f32, f32) -> f32, x: f32, y: f32, eps: f32) -> f32 {
    let x0 = x - eps;
    let x1 = x + eps;
    let y0 = y - eps;
    let y1 = y + eps;
    let dfdx = (f(x1, y) - f(x0, y)) / (2.0 * eps);
    let dfdy = (f(x, y1) - f(x, y0)) / (2.0 * eps);
    dfdy.atan2(-dfdx)
}

pub fn halton(index: u32, base: u32) -> f32 {
    let mut f = 1.0;
    let mut r = 0.0;
    let mut index = index;
    let b = base as f32;
    while index > 0 {
        f /= b;
        r += f * (index % base) as f32;
        index /= base;
    }
    r
}

pub fn stipple<T: AsPrimitive<f32>>(width: T, height: T, n: u32) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let k: u32 = rng.gen();
    let xs = (k..n + k).map(|i| halton(i, 2));
    let ys = (k..n + k).map(|i| halton(i, 3));
    xs.zip(ys)
        .map(|p| Point::from_xy(p.0 * width.as_(), p.1 * height.as_()))
        .collect()
}

// An improvement to Bridson's Algorithm for Poisson Disc sampling.
// https://observablehq.com/@jrus/bridson-fork/2 
pub fn poisson_disk(width: f32, height: f32, radius: f32) -> Vec<Point> {
    const K: usize = 11; // maximum number of samples before rejection
    const M: f32 = 4.0; // a number mutually prime to k
    const EPS: f32 = 0.0000001;
    let mut rng = Pcg64::seed_from_u64(0);
    let cell_size = radius / 2f32.sqrt();
    let cols = (width / cell_size).ceil() as usize;
    let rows = (height / cell_size).ceil() as usize;
    let mut grid: Matrix<Option<Point>> = Matrix::fill(rows, cols, None);
    // let p0 = pt(rng.gen_range(0.0..width), rng.gen_range(0.0..height));
    let p0 = center(width, height);
    let mut active = vec![p0];
    let mut ps = vec![p0];
    let x0 = (p0.y / cell_size).floor() as usize;
    let y0 = (p0.x / cell_size).floor() as usize;
    grid[x0][y0] = Some(p0);

    let neighbors = |i: usize, j: usize| -> Vec<(usize, usize)> {
        let i = i as i32;
        let j = j as i32;
        let mut x;
        let mut y;
        let mut cells = vec![];
        for di in -1..=1 {
            x = i + di;
            if !(0..rows as i32).contains(&x) {
                continue;
            }
            for dj in -1..=1 {
                y = j + dj;
                if (0..cols as i32).contains(&y) {
                    cells.push((x as usize, y as usize));
                }
            }
        }
        cells
    };

    while active.len() > 0 {
        let mut found = false;
        let j = rng.gen_range(0..active.len());
        let p = active[j];
        let seed: f32 = rng.gen();
        for i in 0..K {
            let theta = 2.0 * PI * (seed + M * i as f32 / K as f32);
            let r1: f32 = radius + EPS + radius * 0.5 * rng.gen::<f32>();
            let p1 = pt(p.x + r1 * theta.cos(), p.y + r1 * theta.sin());
            let xi = (p1.y / cell_size).floor() as usize;
            let yi = (p1.x / cell_size).floor() as usize;
            if neighbors(xi, yi).iter().any(|(a, b)| {
                let g = grid[*a][*b];
                g.is_some() && g.unwrap().dist2(p1) < radius * radius
            }) || p1.x < 0.0
                || p1.x >= width
                || p1.y < 0.0
                || p1.y >= height
            {
                continue;
            }
            active.push(p1);
            ps.push(p1);
            grid[xi][yi] = Some(p1);
            found = true;
            break;
        }
        if !found {
            active.remove(j);
        }
    }
    ps
}

pub fn bias(b: f32, t: f32) -> f32 {
    t / ((1.0 / b - 2.0) * (1.0 - t) + 1.0)
}

pub fn gain(g: f32, t: f32) -> f32 {
    if t < 0.5 {
        bias(g, 2.0 * t) / 2.0
    } else {
        bias(1.0 - g, 2.0 * t - 1.0) / 2.0 + 0.5
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn halton_2() {
        assert_eq!(halton(1, 2), 0.5);
        assert_eq!(halton(3, 2), 0.75);
        assert_eq!(halton(6, 2), 0.375);
    }

    #[test]
    fn halton_3() {
        assert_eq!(halton(1, 3), 1.0 / 3.0);
        assert_eq!(halton(3, 3), 1.0 / 9.0);
        assert_eq!(halton(6, 3), 2.0 / 9.0);
    }

    #[test]
    fn poisson_disk_test() {
        dbg!(poisson_disk(100.0, 100.0, 5.0).len());
    }
}
