use rand::{thread_rng, Rng};
use std::env;
use wassily::prelude::*;
use wassily::skia::Canvas;

const WIDTH: u32 = 8100;
const HEIGHT: u32 = 6075;

const EDGE: f32 = 1000.0;
const ALPHA: f32 = 0.03; //0.03
const N: u32 = 250_000;

fn main() {
    let file = env::args().nth(1).expect("Must proved image filename");
    let img = image::open(file).expect("Cannot open file");
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    canvas.fill(WHITE);
    let ps = stipple(WIDTH as f32, HEIGHT as f32, N);
    let mut rng = thread_rng();
    for p in ps {
        let color = get_color(&img, WIDTH, HEIGHT, p).set_opacity(ALPHA);
        let texture = Texture::solid_color(color);
        if rng.gen_bool(0.5) {
            ShapeBuilder::new()
                .circle(p, EDGE / 2.0)
                .fill_texture(&texture)
                .no_stroke()
                .build()
                .draw(&mut canvas);
        } else {
            canvas.fill_rect(p.x - EDGE / 5.0, p.y - EDGE / 5.0, EDGE, EDGE, &texture)
        }
    }
    canvas.save("hint.png");
}
