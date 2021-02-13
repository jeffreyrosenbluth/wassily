#![allow(dead_code)]

use noise::NoiseFn;
use rand::{thread_rng, Rng};
use tiny_skia::*;
use wassily::grid::*;
use wassily::shape::*;
use wassily::util::*;

const WIDTH: u32 = 4032;
const HEIGHT: u32 = 3024;

const GRID_SPACING: f32 = 2.0;
const LENGTH: usize = 250;
const K: f64 = 0.002;
const LINES: usize = 10_000;
const STEP: f32 = 2.0;
const ALPHA: f32 = 0.9;
const EPSILON: f32 = 0.001;

fn curl(f: impl Fn(f32, f32) -> f32, x: f32, y: f32, eps: f32) -> f32 {
    let x0 = x - eps;
    let x1 = x + eps;
    let y0 = y - eps;
    let y1 = y + eps;
    let dfdx = (f(x1, y) - f(x0, y)) / (2.0 * eps);
    let dfdy = (f(x, y1) - f(x, y0)) / (2.0 * eps);
    dfdy.atan2(-dfdx)
}

fn offset(mut points: Vec<Point>, delta_x: f32, delta_y: f32) -> Vec<Point> {
    let mut rev = points.clone();
    rev.reverse();
    let rev = rev.iter_mut().map(|p| pt2(p.x + delta_x, p.y + delta_y));
    points.extend(rev);
    points
}

fn show_grid(canvas: &mut Canvas, grid: &Grid<f32>) {
    let mut paint = Paint::default();
    paint.set_color_rgba8(40, 30, 150, 175);
    let mut stroke = Stroke::default();
    stroke.width = 2.0;
    for (p, a) in grid.iter() {
        let mut pb = PathBuilder::new();
        pb.move_to(p.x, p.y);
        let dx = GRID_SPACING * a.cos();
        let dy = GRID_SPACING * a.sin();
        pb.line_to(p.x + dx, p.y + dy);
        let path = pb.finish().unwrap();
        canvas.stroke_path(&path, &paint, &stroke);
    }
}

fn main() {
    let w = WIDTH as f32;
    let h = HEIGHT as f32;
    let mut rng = thread_rng();

    let mut pixmap = Pixmap::new(WIDTH, HEIGHT).unwrap();
    let mut canvas = Canvas::from(pixmap.as_mut());

    background(&mut canvas, WIDTH, HEIGHT, Color::BLACK);

    let nn = noise::BasicMulti::new();
    let f = |x, y| {
        curl(
            |x, y| nn.get([K * x as f64, K * y as f64]) as f32 * TAU,
            x,
            y,
            EPSILON,
        )
    };
    // let grid = Grid::new(1.1 * w, 1.1 * h, GRID_SPACING, |x, y| {
    //     nn.get([K * x as f64, K * y as f64]) as f32 * TAU
    // });
    let g = |x: f32, y: f32| {
        let a: f32 = 3.0 / 7.0 * x / 1000.0 * TAU;
        let b: f32 = 5.0 / 13.0 * y / 1000.0 * TAU;
        a.sin() * b.sin().powf(3.0)
    };

    let grid = Grid::new(1.1 * w, 1.1 * h, GRID_SPACING, g);

    for _l in 0..LINES {
        let mut ps = vec![];
        let mut velocity = pt2(0.0, 0.0);
        let mut loc = pt2(rng.gen_range(-400.0..w * 1.1), rng.gen_range(-300.0..h * 1.1));
        for _i in 0..LENGTH {
            ps.push(loc);
            let angle = &grid.get(loc.x, loc.y);
            let acc = pt2(STEP * angle.cos(), STEP * angle.sin());
            velocity = pt2(
                (1.0 - ALPHA) * velocity.x + acc.x * ALPHA,
                (1.0 - ALPHA) * velocity.y + acc.y * ALPHA,
            );
            loc += velocity;
        }

        // ps = offset(ps, 25.0, 0.0);
        let poly = ShapeBuilder::new()
            .points(&ps)
            .stroke_color(Color::WHITE)
            .stroke_weight(1.0)
            // .fill_color(Color::from_rgba(0.6, 0.6, 0.7, 0.1).unwrap())
            // .no_stroke()
            .no_fill()
            .build();

        poly.draw(&mut canvas);
    }

    // show_grid(&mut canvas, &grid);
    pixmap.save_png("fat.png").unwrap();
}
