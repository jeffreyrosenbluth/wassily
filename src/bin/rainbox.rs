#![allow(dead_code)]

use noise::{Perlin, Seedable};
use wassily::prelude::*;

use wassily::skia::Canvas;

const WIDTH: f32 = 8191.0;
const HEIGHT: f32 = 4500.0;

fn from_xy(h: f32, ps: &[Point]) -> Vec<Point> {
    ps.iter().map(|p| point2(p.x, p.y + h / 2.0)).collect()
}

fn scale(fs: &[f32], ps: &[Point]) -> Vec<Point> {
    let mut xs = vec![];
    for i in 0..ps.len() {
        xs.push(point2(ps[i].x, fs[i] * ps[i].y));
    }
    xs
}

fn clamp(w: f32, h: f32, ps: &[Point]) -> Vec<Point> {
    ps.iter()
        .map(|p| point2(p.x.clamp(0.0, w), p.y.clamp(0.0, h)))
        .collect()
}

fn gen(i: i32, s: f32, c: f32) -> Point {
    let x = i as f32 / 800.0 * 3.5 - 2.1;
    let y = -0.2 * x.powi(5) - 0.5 * x.powi(4) + 0.8 * x.powi(3) + 2.3 * x.powi(2) + 0.1 * x - 1.75;
    // let y = (x / 4.0).sin();
    point2(i as f32, c - s * y)
}

fn main() {
    let mut canvas = Canvas::new(WIDTH as u32, HEIGHT as u32);
    let path = file_path("hl.png");
    let mut palette = Palette::with_img(path, 1000);
    // palette.colors.reverse();

    // let mut palette = Palette::steal(path, 16);
    palette.sort_by_hue();
    palette.colors.reverse();
    palette.rotate_hue(180.0);
    let n = palette.colors.len();
    // canvas.background(black(1.0));
    canvas.fill(palette.colors[10]);

    let mut rb: Vec<Vec<Point>> = vec![];

    // let s: Vec<f32> = (0.    // let trans = Transform::create_rotation(Angle::radians(-2.0));

    let noise = Noise::<_, 2>::new(WIDTH, HEIGHT, Perlin::default())
        .set_noise_scales(1.5, 1.5)
        .set_noise_factor(5.0)
        .set_seed(1);

    for r in 0..700 {
        let mut row = vec![];
        for c in 0..800 {
            let mut pt = gen(c, 1000.0, -2100.0 + (r as f32) * 6.0);
            pt.x *= 11.0;
            pt.y += 400.0 * noise.noise(pt.x, pt.y);
            row.push(pt);
        }
        // rb.push(clamp(WIDTH, HEIGHT, &from_xy(HEIGHT, &transform(trans, &scale(&s, &row)))));
        rb.push(clamp(
            WIDTH,
            HEIGHT,
            &from_xy(HEIGHT, &scale(&vec![1.0; 800], &row)),
        ));
    }

    for i in 0..rb.len() - 1 {
        let mut path = rb[i].clone();
        let mut back = rb[i + 1].clone();
        back.reverse();
        path.extend(back);
        let band = ShapeBuilder::new()
            .fill_color(palette.colors[i % n])
            .no_stroke()
            .points(&path)
            .build();

        band.draw(&mut canvas);
    }
    canvas.save("rainbox.png");
}
