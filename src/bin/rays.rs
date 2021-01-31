use rand::{thread_rng, Rng};
use tiny_skia::*;
use wassily::*;

const WIDTH: u32 = 4032;
const HEIGHT: u32 = 3024;

fn main() {
    let mut pixmap = Pixmap::load_png("shine.png").expect("Can't load png");
    let mut canvas = Canvas::from(pixmap.as_mut());

    let mut rng = thread_rng();

    let mut stroke_paint = Paint::default();
    stroke_paint.anti_alias = true;
    let mut s = Stroke::default();
    s.width = 2.0;

    let rays = 200;

    for l in 0..rays {
        stroke_paint.set_color_rgba8(255, 255, 255, 200);
        let wght = rng.gen_range(1.0..10.0);
        s.width = wght;
        if l % 10 == 0 {
            s.width = 35.0;
            stroke_paint.set_color_rgba8(252, 163, 17, 255);
        }
        let x = rng.gen_range(1000.0..1600.0);
        let x1 = rng.gen_range(0.0..WIDTH as f32);
        line(&mut canvas, x, 0.0, x1, HEIGHT as f32, &s, &stroke_paint)
    }

    pixmap.save_png("image.png").unwrap();
}
