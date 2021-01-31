use noise::NoiseFn;
use tiny_skia::*;
use wassily::*;

const WIDTH: u32 = 4032;
const HEIGHT: u32 = 3024;
const N: f32 = 6.0;
const D: f32 = 11.0;
const K: f32 = 0.05;
const S: f64 = 0.025;

fn rose(n: f32, d: f32) -> Vec<Point> {
    let mut vertices = vec![];
    let size = WIDTH as f32 / 2.8;
    let nn = noise::BasicMulti::new();
    let w = WIDTH as f32;

    for theta in 0..=360 {
        let t = theta as f32;
        let k = t * std::f32::consts::PI * d / 180.0;
        let r = size * (n * k).sin();
        let x = r * k.cos() + w / 2.0;
        let y = r * k.sin() + HEIGHT as f32 / 2.0;
        let delta_x = K * w * nn.get([S * x as f64, S * y as f64, 0.0]) as f32;
        let delta_y = K * w * nn.get([S * x as f64, S * y as f64, 0.1]) as f32;
        vertices.push(pt2(x + delta_x, y + delta_y));
    }
    vertices
}

fn main() {
    let mut pixmap = Pixmap::load_png("grain.png").expect("Can't load png");
    let mut canvas = Canvas::from(pixmap.as_mut());
    background(
        &mut canvas,
        WIDTH,
        HEIGHT,
        Color::from_rgba(0.0, 0.0, 0.0, 0.5).unwrap(),
    );

    let mut spaint = Paint::default();
    spaint.set_color_rgba8(255, 255, 255, 150);
    let mut stroke = Stroke::default();
    stroke.width = 14.0;

    let ps0 = rose(N, D);
    polyline(&mut canvas, &ps0, &stroke, &spaint);
    pixmap.save_png("maurer.png").unwrap();
}
