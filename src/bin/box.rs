#![allow(dead_code)]

use noise::OpenSimplex;
use wassily::prelude::*;

use wassily::skia::Canvas;

const WIDTH: f32 = 8191.0; // 8191.0
const HEIGHT: f32 = 0.75 * WIDTH; // 6144
const XSTEP: f32 = 7.0; // 7.0
const YSTEP: f32 = 80.0; // 80.0
const LENGTH: usize = 800; // 800
const X: f32 = 10.0; // 10.0
const LINES: usize = 200; // 1000
const COLORS: usize = 1000; // 1000
const SEED: u32 = 0; // 0
const SCALE: f32 = 15.0; // 0.0019

fn main() {
    let wk = Noise::<_, 3>::new(WIDTH, HEIGHT, OpenSimplex::default())
    .seed(SEED)
    .scales(SCALE)
    .z_scale(1.0);
    // wk.set_noise_factor(8.0);
    let mut canvas = Canvas::new(WIDTH as u32, HEIGHT as u32);
    let mut palette = Palette::with_img("oragne.png", COLORS);

    let bg = palette.colors[10];
    canvas.fill(bg);

    for i in 1..LINES {
        let mut l1 = point2(i as f32 * 24.0, wk.height / 2.0);
        let mut up = vec![];
        for _ in 0..LENGTH {
            if !(0.0..wk.width).contains(&l1.x) || !(0.0..wk.height).contains(&l1.y) {
                break;
            }
            up.push(l1);
            let angle = wk.get(l1.x, l1.y, i as f32);
            l1.x += X + XSTEP * angle.cos();
            l1.y += YSTEP * angle.sin();
        }

        if up.len() >= 2 {
            let shape = ShapeBuilder::new()
                .no_fill()
                .stroke_weight(24.0)
                .stroke_color(palette.rand_color())
                .points(&up)
                .build();
            shape.draw(&mut canvas);
        }
    }
    canvas.save("box_skia.png");
}
