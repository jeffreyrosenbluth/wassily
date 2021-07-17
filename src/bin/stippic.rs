use image::GenericImageView;
use wassily::prelude::*;
use wassily::skia::Canvas;

const FILE: &'static str = "pills.png";
const EDGE: f32 = 500.0;
const ALPHA: f32 = 0.05; //0.03
const N: u32 = 100_000;

fn main() {
    let img = image::open(file_path(FILE)).expect("Cannot open file");
    let w = img.width() as f32;
    let h = img.height() as f32;
    let mut canvas = Canvas::new(img.width(), img.height());
    canvas.fill(WHITE);
    let ps = stipple(w, h, N);
    for p in ps {
        let pixel = img.get_pixel(p.x as u32, p.y as u32);
        let color = RGBA::rgb8(pixel[0], pixel[1], pixel[2]).set_opacity(ALPHA);
        let texture = Texture::solid_color(color);
        canvas.fill_rect(p.x - EDGE / 5.0, p.y - EDGE / 5.0, EDGE, EDGE, &texture)
    }
    canvas.save("stippic.png");
}