use crate::canvas::Canvas;
use crate::color_names::WHITE;
use num_traits::FromPrimitive;
use png;
use rand::{Rng, SeedableRng};
use rand_distr::{uniform::SampleUniform, Distribution, Normal};
use rand_pcg::Pcg64;
use std::io::Read;
use std::{
    collections::hash_map::DefaultHasher,
    fs::{create_dir, write, File},
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};
use tiny_skia::PremultipliedColorU8;

type ViewFn = fn(canvas: &mut Canvas);

pub struct Sketch {
    pub width: u32,
    pub height: u32,
    dir: &'static str,
    name: &'static str,
    ext: &'static str,
    view_fn: ViewFn,
    canvas: Canvas,
    pub source: Option<&'static str>,
}

impl Sketch {
    pub fn new(width: u32, height: u32, view_fn: ViewFn) -> Self {
        let canvas = Canvas::new(width, height);
        Self {
            width,
            height,
            dir: "./",
            name: "sketch",
            ext: "png",
            view_fn,
            canvas,
            source: None,
        }
    }

    pub fn dir(self, dir: &'static str) -> Self {
        Self { dir, ..self }
    }

    pub fn name(self, name: &'static str) -> Self {
        Self { name, ..self }
    }

    pub fn ext(self, ext: &'static str) -> Self {
        Self { ext, ..self }
    }

    pub fn w_f32(&self) -> f32 {
        self.width as f32
    }

    pub fn h_f32(&self) -> f32 {
        self.height as f32
    }

    pub fn source(self, source: &'static str) -> Self {
        Self {
            source: Some(source),
            ..self
        }
    }

    pub fn run(&mut self) {
        self.canvas.fill(*WHITE);
        (self.view_fn)(&mut self.canvas);
    }

    pub fn save(&mut self) {
        if let Some(source) = self.source {
            self.save_with_code(source);
        } else {
            let _ = create_dir(self.dir);
            let path = format!(r"{}/{}", self.dir, self.name);
            let mut num = 0;
            let mut sketch = PathBuf::from(format!(r"{}_{}", path, num));
            sketch.set_extension(self.ext);
            while sketch.exists() {
                num += 1;
                sketch = PathBuf::from(format!(r"{}_{}", path, num));
                sketch.set_extension(self.ext);
            }
            sketch.set_extension(self.ext);
            self.canvas.save_png(&sketch);
        }
    }

    pub fn save_with_code(&mut self, file: &'static str) {
        let _ = create_dir(self.dir);
        let path = format!(r"{}/{}", self.dir, self.name);
        let mut num = 0;
        let mut sketch = PathBuf::from(format!(r"{}_{}", path, num));
        sketch.set_extension(self.ext);
        while sketch.exists() {
            num += 1;
            sketch = PathBuf::from(format!(r"{}_{}", path, num));
            sketch.set_extension(self.ext);
        }
        sketch.set_extension(self.ext);
        let file = Path::new(file);
        let mut source = File::open(file).unwrap();
        let mut contents = String::new();
        let _ = source.read_to_string(&mut contents);
        let mut cargo = File::open("Cargo.toml").unwrap();
        let mut toml = String::new();
        let _ = cargo.read_to_string(&mut toml);
        let data = encode_png(&mut self.canvas, contents, toml).unwrap();
        write(&sketch, data).unwrap();
    }
}

pub fn encode_png(
    canvas: &mut Canvas,
    source: String,
    cargo: String,
) -> Result<Vec<u8>, png::EncodingError> {
    let mut tmp_pixmap = canvas.to_owned();
    // Demultiply alpha.
    for pixel in tmp_pixmap.pixels_mut() {
        let c = pixel.demultiply();
        *pixel = PremultipliedColorU8::from_rgba(c.red(), c.green(), c.blue(), c.alpha()).unwrap();
    }

    let mut data = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut data, canvas.width(), canvas.height());
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder
            .add_text_chunk("source".to_string(), source)
            .unwrap();
        encoder.add_text_chunk("cargo".to_string(), cargo).unwrap();
        let mut writer = encoder.write_header()?;
        writer.write_image_data(tmp_pixmap.data())?;
    }

    Ok(data)
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
