use wassily::prelude::*;
use wassily::skia::Canvas;

const WIDTH: u32 = 900;
const HEIGHT: u32 = 900;
const N: f32 = 3.0;
const D: f32 = 72.0;
const L: u32 = 10000;

fn rose(n: f32, d: f32) -> Vec<Point> {
    let mut vertices = vec![];
    let size = WIDTH as f32 / 2.1;
    let w = WIDTH as f32;

    for theta in 0..=L {
        let t = theta as f32;
        let k = t * std::f32::consts::PI * d / 180.0;
        let r = size * (n * k).sin();
        let x = r * k.cos() + w / 2.0;
        let y = r * k.sin() + HEIGHT as f32 / 2.0 + 75.0;
        vertices.push(point2(x, y));
    }
    vertices
}

fn main() {
    // let mut pixmap = Pixmap::new(WIDTH, HEIGHT).unwrap();
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    canvas.fill(RGBA::rgba(0.0, 0.0, 0.0, 0.75));

    let d = D + 0.01;
    let ps0 = rose(N, d);
    let shape = ShapeBuilder::new()
        .no_fill()
        .stroke_color(RGBA::rgba8(255, 255, 255, 100))
        .stroke_weight(0.2)
        .points(&ps0)
        .build();
    shape.draw_cubic(&mut canvas);
    canvas.save("maurer.png");
}
