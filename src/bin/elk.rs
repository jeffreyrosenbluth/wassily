use colorous;
use tiny_skia::*;
use wassily::*;

const WIDTH: u32 = 4032;
const HEIGHT: u32 = 3024;

fn main() {
    let mut pixmap = Pixmap::load_png("elk.png").expect("Can't load png");
    let mut canvas = Canvas::from(pixmap.as_mut());

    let colors = colorous::REDS;

    let mut stroke_paint = Paint::default();
    stroke_paint.anti_alias = true;
    let mut s = Stroke::default();
    s.width = 1.5;

    for l in 0..HEIGHT {
        let t = l as f64 / HEIGHT as f64;
        let c = colors.eval_continuous(t);
        let mut alpha = 235;
        if l < 7 * HEIGHT / 15 {
            alpha = 0
        }
        let kolor = Color::from_rgba8(c.r, c.g, c.b, alpha);
        stroke_paint.set_color(kolor);
        let h = l as f32;
        line(&mut canvas, 0.0, h, WIDTH as f32, h, &s, &stroke_paint);
    }

    let rh = 0.2 * HEIGHT as f32;
    let y = -rh + 7.0 / 15.0 * HEIGHT as f32;
    let mut fill_paint = Paint::default();
    fill_paint.set_color_rgba8(0, 0, 0, 150);
    rectangle(
        &mut canvas,
        0.0,
        y ,
        WIDTH as f32,
        rh,
        &fill_paint,
        &s,
        &fill_paint,
    );

    let mut cpaint = Paint::default();
    cpaint.set_color_rgba8(255, 255, 255, 190);
    let mut spaint = Paint::default();
    spaint.set_color(Color::WHITE);
    s.width = 20.0;
    circle(&mut canvas, 0.88 * WIDTH as f32, 0.15 * HEIGHT as f32, 250.0, &cpaint, &s, &cpaint);

    pixmap.save_png("image.png").unwrap();
}
