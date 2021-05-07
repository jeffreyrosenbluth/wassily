#![allow(dead_code)]

use noise::{BasicMulti, Perlin};
use tiny_skia::*;
use wassily::kolor::*;
use wassily::noise::*;
use wassily::shape::*;
use wassily::util::{pt2, PI};
use rand::prelude::*;

const WIDTH: f32 = 8191.0; // 8191.0
const HEIGHT: f32 = 0.75 * WIDTH; // 6144
const XSTEP: f32 = 10.0; // 7.0
const YSTEP: f32 = 8.0; // 80.0
const LENGTH: usize = 925; // 800
const LINES: usize = 1; // 1000
const SEED: u32 = 1; // 0
const SEED2: u32 = 30; // 23
const SCALE: f32 = 1.5; // 0.0019
const K: f32 = 1.0;
const DOMAIN: f32 = 4.0 * PI;

fn main() {
    let mut wk = Noise::<[f64; 2], _>::new(WIDTH, HEIGHT, BasicMulti::new());
    let s1: u32 = random(); 
    let s2: u32 = random(); 
    dbg!(s1, s2);
    wk.set_noise_scales(SCALE, SCALE, 1.0);
    wk.set_noise_seed(s1);
    let mut canvas = Pixmap::new(wk.width_n(), wk.height_n()).unwrap();
    let img = Pixmap::load_png("fruit.png").expect("Can't loag image");
    let mut palette = Palette::with_img(img, LINES);
    // palette.sort_by_chroma();
    // let bg = palette.colors[0];
    let bg = Color::BLACK;
    canvas.fill(bg);

    let mut l1 = pt2(0.0, wk.height / 2.0);
    let mut curve1 = vec![];
    for _ in 0..wk.width_n() {
        curve1.push(l1);
        let angle = K * PI * wk.noise(l1.x, l1.y);
        l1.x += XSTEP * angle.cos();
        l1.y += YSTEP * angle.sin();
    }
    let mut l1 = pt2(0.0, wk.height / 2.0);
    let mut curve2 = vec![];
    wk.set_noise_seed(s2);
    for _ in 0..LENGTH {
        if !(0.0..wk.width).contains(&l1.x) || !(0.0..wk.height).contains(&l1.y) {
            break;
        }
        curve2.push(l1);
        let angle = K * PI * wk.noise(l1.x, l1.y);
        l1.x += XSTEP * angle.cos();
        l1.y += YSTEP * angle.sin();
    }
    // let curve3: Vec<Point> = curve.iter().zip(&curve2).map(|(p1, p2)| {
    //     let a =  (p1.x / WIDTH).sqrt();
    //     let y = a * p1.y + (1.0 - a) * p2.y;
    //     pt2(p1.x, y)
    // }).collect();
    let mut curve3:Vec<Point> = vec![];
    for i in 0..curve2.len() {
        if i < 600 {
            let a = 1.0 - i as f32 / 600.0;
            let mut y = curve2[i].y;
            y -= HEIGHT / 2.0;
            y *= a;
            y = HEIGHT / 2.0 + y;
            let p = pt2(curve2[i].x, y);
            curve3.push(p)
        } else {
            curve3.push(curve1[i])
        }
    }

    let shape = ShapeBuilder::new()
        .no_fill()
        .stroke_weight(10.0)
        // .stroke_color(palette.colors[i])
        .stroke_color(Color::WHITE)
        .points(&curve1)
        .build();
    shape.draw(&mut canvas);


    let shape = ShapeBuilder::new()
        .no_fill()
        .stroke_weight(10.0)
        // .stroke_color(palette.colors[i])
        .stroke_color(red(1.0))
        .points(&curve2)
        .build();
    shape.draw(&mut canvas);

    // let shape = ShapeBuilder::new()
    //     .no_fill()
    //     .stroke_weight(10.0)
    //     // .stroke_color(palette.colors[i])
    //     .stroke_color(blue(1.0))
    //     .points(&curve3)
    //     .build();
    // shape.draw(&mut canvas);

    canvas.save_png("go_fruit.png").unwrap();
}
