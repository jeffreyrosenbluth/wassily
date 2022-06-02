use crate::canvas::Canvas;
use crate::color_names::WHITE;
use crate::math::Algebra;
use png;
use std::io::Read;
use std::{
    collections::hash_map::DefaultHasher,
    fs::{create_dir, write, File},
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};
use tiny_skia::{Point, PremultipliedColorU8, Rect};

pub type ViewFn<Model> = fn(canvas: &mut Canvas, model: &Model);
// pub type UpdateFn<Model, Update> = fn(model: &mut Model, update: &Update);

pub struct Sketch<M> {
    dir: &'static str,
    name: &'static str,
    ext: &'static str,
    view_fn: ViewFn<M>,
    pub canvas: Canvas,
    pub source: Option<&'static str>,
}

impl<M> Sketch<M> {
    pub fn new(width: u32, height: u32, view_fn: ViewFn<M>) -> Self {
        let canvas = Canvas::new(width, height);
        Self {
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

    pub fn source(self, source: &'static str) -> Self {
        Self {
            source: Some(source),
            ..self
        }
    }

    pub fn run(&mut self, model: &M) {
        self.canvas.fill(*WHITE);
        // (self.update_fn)(&mut self.model, &self.update);
        (self.view_fn)(&mut self.canvas, model);
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
        let file = Path::new(file);
        let mut source = File::open(file).unwrap();
        let mut contents = String::new();
        let _ = source.read_to_string(&mut contents);
        let mut cargo = File::open("Cargo.toml").unwrap();
        let mut toml = String::new();
        let _ = cargo.read_to_string(&mut toml);
        let data = encode_png(&mut self.canvas, contents, toml).unwrap();
        write(&sketch, data).expect(format!("{:?}", &sketch).as_str());
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
        // Need to insure that alpha is greater than all color components
        // because tiny_skia does not export `PremultipliedColrU8::from_rgba_unchecked().
        let m = c.red().max(c.green()).max(c.blue()).max(c.alpha());
        *pixel = PremultipliedColorU8::from_rgba(c.red(), c.green(), c.blue(), m).unwrap();
    }
    let mut data = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut data, canvas.width(), canvas.height());
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder
            .add_text_chunk("Source".to_string(), source)
            .unwrap();
        encoder.add_text_chunk("Software".to_string(), cargo).unwrap();
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

pub fn smooth_step(t: f32) -> f32 {
    let s = t.clamp(0.0, 1.0);
    s * s * (3.0 - 2.0 * s)
}

pub fn smoother_step(t: f32) -> f32 {
    let s = t.clamp(0.0, 1.0);
    s * s * s * (6.0 * s * s - 15.0 * s + 10.0)
}

pub fn bounding_box(points: &[Point], min_size: f32) -> Rect {
    let (left, top, right, bottom) =
        points
            .iter()
            .fold((f32::MAX, f32::MAX, f32::MIN, f32::MIN), |mut acc, p| {
                if p.x < acc.0 {
                    acc.0 = p.x
                }; 
                if p.x > acc.2 {
                    acc.2 = p.x
                };
                if p.y < acc.1 {
                    acc.1 = p.y
                }; 
                if p.y > acc.3 {
                    acc.3 = p.y
                };
                acc
            });
    let right = right.max(left + min_size);
    let bottom = bottom.max(top + min_size);
    Rect::from_ltrb(left, top, right, bottom).unwrap()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Trail {
    Open,
    Closed,
}

pub fn chaiken(pts: &[Point], n: u32, trail: Trail) -> Vec<Point> {
    let mut pts = pts.to_vec();
    const RATIO: f32 = 0.25;
    if n == 0 || pts.len() < 3 {
        if trail == Trail::Closed {
            pts.push(pts[0])
        }
        return pts;
    }
    if trail == Trail::Closed {
        pts.push(pts[0]);
    }
    let mut c: Vec<Point> = pts
        .windows(2)
        .flat_map(|ps| [ps[0].lerp(ps[1], RATIO), ps[1].lerp(ps[0], RATIO)])
        .collect();
    if trail == Trail::Open {
        c.insert(0, pts[0]);
        c.push(pts[pts.len() - 1]);
    }
    chaiken(&c, n - 1, trail)
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::prelude::pt;

    #[test]
    fn smooth_step_test() {
        assert_eq!(smooth_step(0.0), 0.0);
        assert_eq!(smooth_step(1.0), 1.0);
        assert_eq!(smooth_step(0.5), 0.5);
        assert_eq!(smooth_step(0.25), 0.15625);
        assert_eq!(smooth_step(0.75), 0.84375);
    }

    #[test]
    fn smoother_step_test() {
        assert_eq!(smoother_step(0.0), 0.0);
        assert_eq!(smoother_step(1.0), 1.0);
        assert_eq!(smoother_step(0.5), 0.5);
        assert_eq!(smoother_step(0.25), 0.103515625);
        assert_eq!(smoother_step(0.75), 0.8964844);
    }

    #[test]
    fn bounding_box_test() {
        let points = vec![
            pt(10, 10),
            pt(-100, 90),
            pt(100, -80),
            pt(80, 100),
        ];
        let bbox = bounding_box(&points, 0.0);
        assert_eq!(bbox, Rect::from_ltrb(-100.0, -80.0, 100.0, 100.0).unwrap());
    }
}
