#![allow(dead_code)]

use euclid::{Rotation2D, Angle};
use noise::Perlin;
use rand::prelude::*;
use std::path;
use wassily::{Point, Transform, kolor::Palette, noise::*, point2, shape::*, util::PI};

#[cfg(feature = "tiny-skia")]
use wassily::skia::Canvas;

const WIDTH: f32 = 8191.0;
const HEIGHT: f32 = 0.80 * WIDTH;

fn transform_pt(t: Transform, p: Point) -> Point {
    let x = p.x * t.m11 + p.y * t.m21 + t.m31;
    let y = p.x * t.m12 + p.y * t.m22 + t.m32;
    point2(x, y)
}

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

fn translate(ts: &[f32], ps: &[Point]) -> Vec<Point> {
    let mut xs = vec![];
    for i in 0..ps.len() {
        xs.push(point2(ps[i].x, ts[i] + ps[i].y));
    }
    xs
}

fn transform(t: Transform, ps: &[Point]) -> Vec<Point> {
    ps.iter().map(|p| transform_pt(t, *p)).collect()
}

fn clamp(w: f32, h: f32, ps: &[Point]) -> Vec<Point> {
    ps.iter()
        .map(|p| point2(p.x.clamp(0.0, w), p.y.clamp(0.0, h)))
        .collect()
}

fn gen(i: i32, s: f32, c: f32) -> Point {
    let x = i as f32 / 800.0 * 3.5 - 2.1;
    let y = -0.2 * x.powi(5) - 0.5 * x.powi(4) + 0.8 * x.powi(3) + 2.3 * x.powi(2) + 0.1 * x - 1.75;
    point2(i as f32, c - s * y)
}

fn main() {
    let mut canvas = Canvas::new(WIDTH as u32, HEIGHT as u32);
    let path = path::Path::new("fruit.png");
    let mut palette = Palette::with_img(path, 500);
    // palette.sort_by_chroma();
    palette.colors.reverse();
    canvas.background(palette.colors[434]);

    let mut rb: Vec<Vec<Point>> = vec![];
    // let s: Vec<f32> = (0..800)
    //     .into_iter()
    //     .map(|i| (3.5 * PI * i as f32 / 800.0).sin())
    //     .collect();
    // let trans = Rotation2D::new(Angle::radians(-2.0)).to_transform();

    let mut noise = Noise::<[f64; 2], _>::new(WIDTH, HEIGHT, Perlin::new());
    noise.set_noise_scales(1.5, 1.5, 1.0);
    noise.set_noise_factor(3.0);
    noise.set_noise_seed(1);

    for r in 0..500 {
        let mut row = vec![];
        for c in 0..800 {
            let mut pt = gen(c, 1000.0, -2000.0 + (r as f32) * 6.0);
            pt.x *= 11.0;
            // let mut pt = point2(11.0 * c as f32, 30.0 * (r as f32 - 4.0));
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
            .fill_color(palette.colors[i])
            .no_stroke()
            .points(&path)
            .build();

        band.draw(&mut canvas);
    }
    canvas.save_png("rainbox_skia.png");
}
