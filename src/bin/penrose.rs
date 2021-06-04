use std::vec;
use noise::{Seedable, OpenSimplex};

use euclid::Angle;
use euclid::Point2D;
use euclid::UnknownUnit;
use rand::prelude::*;
use wassily::prelude::*;
use wassily::skia::Canvas;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 1200;
const SCALE: f32 = 2.5;
const FACTOR: f32 = 40.0;
const SEGMENTS: u32 = 1000;

fn side_points() -> Vec<Point> {
    let sin30: f32 = (PI / 6.0).sin();
    let cos30: f32 = (PI / 6.0).cos();
    let cos60: f32 = sin30;
    let sin60: f32 = cos30;
    let e = 410.0;
    let w = 90.0;

    let e3 = e + 3.0 * w;
    let e4 = e + 4.0 * w;

    let p1 = point2(-e * cos60, e * cos60 / 3f32.sqrt());
    let p2 = point2(p1.x - w, p1.y);
    let p3 = point2(p2.x + e3 * cos60, p2.y - e3 * sin60);
    let p4 = point2(p3.x + e4 * sin30, p3.y + e4 * cos30);
    let p5: Point2D<_, UnknownUnit> = point2(p4.x - w * cos60, p4.y + w * sin60);
    let p6: Point2D<_, UnknownUnit> = point2(p5.x - e3 * cos60, p5.y - e3 * sin60);

    let (q1, q2) = segment(p2, p3, p5, p6, SEGMENTS);
    let mut ps = vec![p1];
    ps.extend(q1);
    ps.push(p4);
    ps.extend(q2);
    ps

    // vec![p1, p2, p3, p4, p5, p6]
}

fn segment(p1: Point, p2: Point, p3: Point, p4: Point, n: u32) -> (Vec<Point>, Vec<Point>) {
    let wk = Noise::<_, 3>::new(WIDTH as f32, HEIGHT as f32, OpenSimplex::default())
        .set_noise_scales(SCALE, SCALE, 1.0)
        .set_seed(7)
        .set_noise_factor(FACTOR);
    let mut alphas: Vec<f32> = vec![];
    for _i in 0..n - 1 {
        alphas.push(random());
    }
    alphas.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut ps = vec![p1];
    let mut qs = vec![p3];
    for a in alphas {
        let px = p1.x + a * (p2.x - p1.x);
        let py = p1.y + a * (p2.y - p1.y);
        let dx = wk.noise(px, py, 0.0);
        let dy = wk.noise(px, py, 10.0);
        ps.push(point2(px + dx, py + dy));
        let qx = p3.x + a * (p4.x - p3.x);
        let qy = p3.y + a * (p4.y - p3.y);
        let dx = wk.noise(qx, qy, 0.0);
        let dy = wk.noise(qx, qy, 10.0);
        qs.push(point2(qx + dx, qy + dy));
    }
    // ps.push(p2);
    // qs.push(p4);
    (ps, qs)
}

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let (width, height) = (WIDTH as f32, HEIGHT as f32);

    let loc = vec2(0.5 * width, 0.6 * height);
    let ps = side_points();

    let mut palette = Palette::new(vec![]);
    // let seed = 14105855538212455564;
    let seed: u64 = random();
    dbg!(seed);
    palette.set_seed(seed);

    canvas.fill(RGBA::new(1.0, 0.97, 0.86, 1.0));

    let trans1 = Transform::identity().post_translate(loc);
    let side1 = ShapeBuilder::new()
        .points(&ps)
        .fill_color(palette.rand_laba())
        .no_stroke()
        .transform(trans1)
        .build();
    side1.draw(&mut canvas);

    let trans2 = Transform::identity()
        .post_translate(loc)
        .pre_rotate(Angle::radians(TAU / 3.0));
    let side2 = ShapeBuilder::new()
        .points(&ps)
        .fill_color(palette.rand_laba())
        .no_stroke()
        .transform(trans2)
        .build();
    side2.draw(&mut canvas);

    let trans3 = Transform::identity()
        .post_translate(loc)
        .pre_rotate(Angle::radians(2.0 * TAU / 3.0));
    let side3 = ShapeBuilder::new()
        .points(&ps)
        .fill_color(palette.rand_laba())
        .no_stroke()
        .transform(trans3)
        .build();
    side3.draw(&mut canvas);

    // for p in ps {
    //     let c = ShapeBuilder::new()
    //         .circle(p, 6.0)
    //         .fill_color(RGBA::new(1.0, 0.0, 0.0, 1.0))
    //         .no_stroke()
    //         .transform(trans1)
    //         .build();
    //     c.draw(&mut canvas);
    // }

    canvas.save("triangle.png");
}
