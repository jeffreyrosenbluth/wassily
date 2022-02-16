use crate::{
    prelude::BasicModel,
};
use chrono::prelude::Utc;
use num_traits::FromPrimitive;
use rand::{Rng, SeedableRng};
use rand_distr::{uniform::SampleUniform, Distribution, Normal};
use rand_pcg::Pcg64;
use std::{
    collections::hash_map::DefaultHasher,
    fs::{create_dir, File},
    hash::{Hash, Hasher},
    io::Write,
    path::PathBuf,
};
use tiny_skia::Pixmap;

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