#![allow(dead_code)]

use tiny_skia::*;
use wassily::shape::*;
use wassily::util::*;

const WIDTH: f32 = 8191.0; // 8191.0
const HEIGHT: f32 = 0.75 * WIDTH; // 6144
const XSTEP: f32 = 10.0; // 7.0
const YSTEP: f32 = 10.0; // 80.0
const LENGTH: usize = 800; // 800
const LINES: usize = 2000; // 1000
const COLORS: usize = 2000; // 1000
const SEED: u64 = 1; // 0
const SCALE: f32 = 0.0025; // 0.0019
const RADIUS: f32 = 2000.0;
const K: f32= 4.67;

fn main() {
    let mut wk = Wassily::new(WIDTH, HEIGHT);
    // wk.set_noise_fn(noise::BasicMulti::new());
    wk.set_seed(SEED);
    wk.set_noise_scale(SCALE);
    let mut canvas = Pixmap::new(wk.width_n(), wk.height_n()).unwrap();
    let img = Pixmap::load_png("rg.png").expect("Can't loag image");
    wk.set_colors(img, COLORS);
    let bg = wk.color(99);
    // let bg = Color::frjm_rgba8(255, 248, 235, 255);
    canvas.fill(bg);

    for i in 1..LINES {
        let theta = i as f32 / LINES as f32 * TAU;
        let x = RADIUS * theta.cos() + wk.width / 2.0;
        let y = RADIUS * theta.sin() + wk.height / 2.0;
        let mut l1 = pt2(x, y);
        let mut curve = vec![];

        let mut r_channel = CosChannel::new(0.0 * TAU);
        r_channel.a = 0.5;
        r_channel.b = 1.0 - r_channel.a;
        r_channel.freq = 1.0;
        let mut g_channel = CosChannel::new(0.15 * TAU);
        g_channel.a = 0.5;
        g_channel.b = 1.0 - g_channel.a;
        g_channel.freq = 0.7;
        let mut b_channel = CosChannel::new(0.2 * TAU);
        b_channel.a = 0.5;
        b_channel.b = 1.0 - b_channel.a;
        g_channel.freq = 0.4;

        for _ in 0..LENGTH {
            if !(0.0..wk.width).contains(&l1.x) || !(0.0..wk.height).contains(&l1.y) {
                break;
            }
            curve.push(l1);
            let angle = K * PI * wk.noise(l1.x, l1.y, i as f32);
            l1.x += XSTEP * angle.cos();
            l1.y += YSTEP * angle.sin();
        }

        if curve.len() >= 2 {
            let dot = ShapeBuilder::new()
                .no_stroke()
                .fill_color(black(0.5))
                .circle(pt2(x, y), 1.0)
                .build();
            // dot.draw(&mut canvas);

            let mut c = cos_color(r_channel, g_channel, b_channel, theta);
            // c.set_alpha(0.6);

            let shape = ShapeBuilder::new()
                .no_fill()
                .stroke_weight(2.0)
                .stroke_color(wk.color(i))
                // .stroke_color(c)
                // .stroke_color(color(theta))
                // .stroke_color(wk.color((0.3 * COLORS as f32) as usize))
                .points(&curve)
                .build();
            shape.draw(&mut canvas);
        }
    }
    canvas.save_png("ring.png").unwrap();
}

fn color(theta: f32) -> Color {
    let k = 0.5; // must be >= 0.5
    let r = k + (1.0 - k) * theta.cos();
    let g = k + (1.0 - k) * (theta - PI / 3.0).cos();
    let b = k + (1.0 - k) * (theta - PI * 2.0 / 3.0).cos();
    let a = 0.4;
    Color::from_rgba(r, g, b, a).unwrap()
}
