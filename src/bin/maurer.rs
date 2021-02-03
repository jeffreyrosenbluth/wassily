use tiny_skia::*;
use wassily::*;

const WIDTH: u32 = 900;
const HEIGHT: u32 = 900;
const N: f32 = 6.0;
const D: f32 = 71.0;
const L: u32 = 5000;

fn rose(n: f32, d: f32) -> Vec<Point> {
    let mut vertices = vec![];
    let size = WIDTH as f32 / 2.25;
    let w = WIDTH as f32;

    for theta in 0..=L {
        let t = theta as f32;
        let k = t * std::f32::consts::PI * d / 180.0;
        let r = size * (n * k).sin();
        let x = r * k.cos() + w / 2.0;
        let y = r * k.sin() + HEIGHT as f32 / 2.0;
        vertices.push(pt2(x, y));
    }
    vertices
}

fn main() {
    let mut pixmap = Pixmap::new(WIDTH, HEIGHT).unwrap();
    let mut canvas = Canvas::from(pixmap.as_mut());
    background(
        &mut canvas,
        WIDTH,
        HEIGHT,
        Color::from_rgba(0.0, 0.0, 0.0, 1.0).unwrap(),
    );

    let mut spaint = Paint::default();
    spaint.set_color_rgba8(255, 255, 255, 100);
    spaint.anti_alias = true;
    let mut stroke = Stroke::default();
    stroke.width = 0.2;

    let d = D + 0.01;
    let ps0 = rose(N, d);
    polycurve(&mut canvas, &ps0, &stroke, &spaint, pt2(450., 450.));
    // polyline(&mut canvas, &ps0, &stroke, &spaint);
    pixmap.save_png("maurer.png").unwrap();
}
