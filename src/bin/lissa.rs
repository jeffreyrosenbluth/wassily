use tiny_skia::*;
use wassily::*;

const WIDTH: u32 = 4032 ;
const HEIGHT: u32 = 3024;

fn main() {
    let mut pixmap = Pixmap::load_png("grid3.png").expect("Can't load png");
    let mut canvas = Canvas::from(pixmap.as_mut());

    let mut fill_paint = Paint::default();
    fill_paint.anti_alias = true;
    fill_paint.set_color(Color::TRANSPARENT);
    
    background(&mut canvas, WIDTH, HEIGHT, Color::from_rgba8(50, 50, 50, 175));

    let mut stroke_paint = Paint::default();
    stroke_paint.anti_alias = true;
    let mut s = Stroke::default();
    s.width = 4.0;


    let rings = 1200;
    for i in 0..rings {
        let alpha = 255 - (150.0 * i as f32 / rings as f32) as u8;
        stroke_paint.set_color_rgba8(255, 255, 255, alpha);
        let delta = std::f32::consts::TAU / rings as f32;
        let t = i as f32 * delta;
        let w = WIDTH as f32 / 2.0 - 200.0;
        let h = HEIGHT as f32 / 2.0 - 200.0;
        let x =  1200.0 * (1.0 * t + std::f32::consts::FRAC_PI_4).sin();
        let y =  1200.0 * (3.0 * t).sin();
        rectangle(
            &mut canvas,
            x + w,
            y + h,
            i as f32 * 0.5,
            i as f32 * 0.5,
            &fill_paint,
            &s,
            &stroke_paint,
        );
    }

    pixmap.save_png("image.png").unwrap();
}
