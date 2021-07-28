use noise::OpenSimplex;
use wassily::prelude::*;

use wassily::skia::Canvas;

const WIDTH: f32 = 8191.0;
const HEIGHT: f32 = 8191.0;
const XSTEP: f32 = 10.0; // 7.0
const YSTEP: f32 = 10.0; // 80.0
const LENGTH: usize = 1200; // 800
const LINES: usize = 4000; // 1000
const SEED: u32 = 1; // 0
const SCALE: f32 = 10.0; // 0.0019
const RADIUS: f32 = 2500.0;
const K: f32 = 3.25;

fn main() {
    let wk = Noise::<_, 3>::new(WIDTH, HEIGHT, OpenSimplex::default())
        .seed(SEED)
        .xy_scales(SCALE)
        .z_scale(SCALE / WIDTH)
        .factor(1.0);
    let mut canvas = Canvas::new(WIDTH as u32, HEIGHT as u32);
    let path = "hl.png";
    let mut palette = Palette::with_img(path, LINES);
    palette.rotate_hue(150.0);
    // palette.sort_by_chroma();
    let mut stolen = Palette::steal(path, 16);
    stolen.rotate_hue(150.0);

    let bg = stolen.colors[2];

    canvas.fill(bg);

    for i in 1..LINES {
        let theta = i as f32 / LINES as f32 * TAU;
        let x = RADIUS * theta.cos() + wk.width / 2.0;
        let y = RADIUS * theta.sin() + wk.height / 2.0;
        let mut l1 = point2(x, y);
        let mut curve = vec![];

        for _ in 0..LENGTH {
            if !(0.0..wk.width).contains(&l1.x) || !(0.0..wk.height).contains(&l1.y) {
                break;
            }
            curve.push(l1);
            let angle = K * PI * wk.get(l1.x, l1.y, i as f32);
            l1.x += XSTEP * angle.cos();
            l1.y += YSTEP * angle.sin();
        }

        if curve.len() >= 2 {
            let shape = ShapeBuilder::new()
                .no_fill()
                .stroke_weight(8.0)
                .stroke_color(palette.rand_color())
                // .stroke_color(palette.colors[i])

                .points(&curve)
                .build();
            shape.draw(&mut canvas);
        }
    }
    canvas.save("solar_k3.25.png");
}