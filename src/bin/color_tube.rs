#![allow(dead_code)]

use noise::OpenSimplex;
use wassily::prelude::*;

// use wassily::skia::Canvas;
use wassily::raqote::Canvas;

const WIDTH: u32 = 8000; // 8191
const HEIGHT: u32 = 4500; // 6144
const XSTEP: f32 = 7.0; // 7.0
const YSTEP: f32 = 80.0; // 80.0
const LENGTH: usize = 800; // 800
const X: f32 = 10.0; // 10.0
const LINES: usize = 1000; // 1000
const COLORS: usize = 1000; // 1000
const SEED: u32 = 2; // 0
const SCALE: f32 = 2.; // 0.0019
const GRID: f32 = 5.0; // 15.0

fn main() {
    let wk = Noise::<_, 3>::new(WIDTH as f32, HEIGHT as f32, OpenSimplex::default())
        .set_seed(SEED)
        .set_scales(SCALE);
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let path = file_path("candy.png");
    let mut palette = Palette::with_img(path, COLORS);

    let _bg = palette.colors[(0.99 * COLORS as f32) as usize];
    canvas.fill(WHITE);

    for i in 0..LINES {
        let mut l1 = point2(GRID * i as f32, wk.height / 2.0);
        let mut l2 = point2(GRID * i as f32, wk.height / 2.0);
        let mut up = vec![];
        for _ in 0..LENGTH {
            if l1.x >= wk.width || l1.y >= wk.height {
                break;
            }
            up.push(l1);
            let angle = wk.noise(l1.x, l1.y, 0.0);
            l1.x += X + XSTEP * angle.cos();
            l1.y += YSTEP * angle.sin();
        }
        let mut dn = vec![];
        for _ in 0..LENGTH {
            if l2.x >= wk.width || l2.y >= wk.height {
                break;
            }
            dn.push(l2);
            let angle = wk.noise(l2.x, l2.y, 1.0);
            l2.x += X + XSTEP * angle.cos();
            l2.y += YSTEP * angle.sin();
        }
        dn.reverse();
        up.extend(dn);

        if up.len() >= 2 {
            let shape = ShapeBuilder::new()
                .fill_color(palette.rand_color())
                .no_stroke()
                .points(&up)
                .build();
            shape.draw(&mut canvas);
        }
    }
    canvas.save("ct.png");
}
