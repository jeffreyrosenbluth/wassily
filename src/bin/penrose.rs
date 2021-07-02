use std::vec;

use euclid::Angle;
use euclid::Point2D;
use euclid::UnknownUnit;
use wassily::prelude::*;
use wassily::skia::Canvas;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 1200;
const THICK: f32 = 60.0;
const GRAINS: u32 = 128;

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

    vec![p1, p2, p3, p4, p5, p6]
}

fn trans_vec(transform: Transform, ps: &Vec<Point>) -> Vec<Point> {
    ps.iter().map(|p| transform.transform_point(*p)).collect()
}

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let (width, height) = (WIDTH as f32, HEIGHT as f32);

    canvas.fill(WHITE);

    let dots = stipple(width, height, 600_000);
    for d in dots {
        pixel(d.x, d.y, BLACK.set_opacity(0.5), &mut canvas);
    }

    let loc = vec2(0.5 * width, 0.6 * height);
    let ps = side_points();

    let mut palette = Palette::new(vec![]);
    palette.set_seed(226349872075752273);


    let trans1 = Transform::identity().post_translate(loc);
    let ps1 = trans_vec(trans1, &ps);
    let c = palette.rand_lab();
    let side1 = ShapeBuilder::new()
        .points(&ps)
        .fill_color(WHITE)
        .no_stroke()
        .transform(&trans1)
        .build();
    side1.draw(&mut canvas);
    for (i, p) in ps1.iter().enumerate() {
        let p2 = if i <= ps1.len() - 2 {
            ps1[i + 1]
        } else {
            ps1[0]
        };
        let mut sand = SandLine::new(*p, p2)
            .thickness(THICK)
            .color(c)
            .grains(GRAINS);
        sand.draw(&mut canvas);
    }

    let trans2 = Transform::identity()
        .post_translate(loc)
        .pre_rotate(Angle::radians(TAU / 3.0));
    let ps2 = trans_vec(trans2, &ps);
    let c = palette.rand_lab();
    let side2 = ShapeBuilder::new()
        .points(&ps)
        .fill_color(WHITE)
        .no_stroke()
        .transform(&trans2)
        .build();
    side2.draw(&mut canvas);
    for (i, p) in ps2.iter().enumerate() {
        let p2 = if i <= ps2.len() - 2 {
            ps2[i + 1]
        } else {
            ps2[0]
        };
        let mut sand = SandLine::new(*p, p2)
            .thickness(THICK)
            .color(c)
            .grains(GRAINS);
        sand.draw(&mut canvas);
    }

    let trans3 = Transform::identity()
        .post_translate(loc)
        .pre_rotate(Angle::radians(2.0 * TAU / 3.0));
    let ps3 = trans_vec(trans3, &ps);
    let c = palette.rand_lab();
    let side3 = ShapeBuilder::new()
        .points(&ps)
        .fill_color(WHITE)
        .no_stroke()
        .transform(&trans3)
        .build();
    side3.draw(&mut canvas);
    for (i, p) in ps3.iter().enumerate() {
        let p2 = if i <= ps3.len() - 2 {
            ps3[i + 1]
        } else {
            ps3[0]
        };
        let mut sand = SandLine::new(*p, p2)
            .thickness(THICK)
            .color(c)
            .grains(GRAINS);
        sand.draw(&mut canvas);
    }

    canvas.save("penrose.png");
}
