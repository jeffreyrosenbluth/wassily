use tiny_skia::*;
use wassily::shape::*;
use wassily::util::*;

fn main() {
    let mut img = Pixmap::load_png("turner.png").expect("Can't load png");
    let pixels = img.pixels_mut().iter_mut().map(|c| {
        let r = c.red();
        let g = c.green();
        let b = c.blue();
        let gray = (0.33 * r as f32 + 0.34 * g as f32 + 0.33 * b as f32) as u8;
        let gray = PremultipliedColorU8::from_rgba(gray, gray, gray, 255);
        *c = gray.unwrap();
    });
}
