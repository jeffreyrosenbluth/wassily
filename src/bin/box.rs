#![allow(dead_code)]

use wassily::shape::*;
use wassily::util::*;
use wassily::base::*;
use image::io::Reader as ImageReader;

const WIDTH: f32 = 8191.0; // 8191.0
const HEIGHT: f32 = 0.75 * WIDTH ;// 6144
const XSTEP: f32 = 7.0; // 7.0
const YSTEP: f32 = 150.0; // 80.0
const LENGTH: usize = 800; // 800
const X: f32 = 10.0; // 10.0
const LINES: usize = 2000; // 1000
const COLORS: usize = 1000; // 1000
const SEED: u64 = 1; // 0
const SCALE: f32 = 0.0019; // 0.0019
const GRID: f32 = 5.0; // 15.0

fn main() {
    let mut wk = Wassily::new(WIDTH, HEIGHT);
    wk.set_seed(SEED);
    wk.set_noise_scale(SCALE);
    let mut canvas = Canvas::new(wk.width, wk.height);
    let img = ImageReader::open("organge.png").unwrap().decode().unwrap();
    wk.set_colors(img, COLORS);
    let bg = wk.color(999);
    canvas.fill(bg);

    for i in 1..LINES {
        let mut l1 = pt2(0.0, wk.height / 2.0);
        // let mut l1 = pt2(GRID * i as f32, wk.height - GRID * i as f32);
        let mut up = vec![];
        for _ in 0..LENGTH {
            if !(0.0..wk.width).contains(&l1.x) || !(0.0..wk.height).contains(&l1.y) {
                break;
            }
            up.push(l1);
            let angle = wk.noise(l1.x, l1.y, i as f32);
            l1.x += X + XSTEP * angle.cos();
            l1.y += YSTEP * angle.sin();
        }

        if up.len() >= 2 {
            let shape = ShapeBuilder::new()
                .no_fill()
                .stroke_weight(2.0)
                .stroke_color(wk.rand_color())
                // .stroke_color(wk.color((0.3 * COLORS as f32) as usize))
                .points(&up)
                .build();
            shape.draw(&mut canvas);
        }
    }
    canvas.save_png("box.png").unwrap();
}
