use crate::canvas::Canvas;
use crate::color_names::WHITE;
use png;
use std::io::Read;
use std::{
    fs::{create_dir, write, File},
    path::{Path, PathBuf},
};
use tiny_skia::PremultipliedColorU8;

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
        self.canvas.pixmap.fill(*WHITE);
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
    let mut tmp_pixmap = canvas.pixmap.to_owned();
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
        encoder
            .add_text_chunk("Software".to_string(), cargo)
            .unwrap();
        let mut writer = encoder.write_header()?;
        writer.write_image_data(tmp_pixmap.data())?;
    }
    Ok(data)
}
