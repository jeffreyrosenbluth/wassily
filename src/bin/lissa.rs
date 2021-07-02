use wassily::prelude::*;
use wassily::skia::Canvas;

const WIDTH: u32 = 4032;
const HEIGHT: u32 = 3024;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    canvas.fill(RGBA::with_8(50, 50, 50, 175));
    let mut s = Stroke::default();
    s.width = 4.0;

    let rings = 1200;
    for i in 10..rings {
        let alpha = 255 - (150.0 * i as f32 / rings as f32) as u8;
        let sc = RGBA::with_8(255, 255, 255, alpha);
        let delta = std::f32::consts::TAU / rings as f32;
        let t = i as f32 * delta;
        let w = WIDTH as f32 / 2.0 - 200.0;
        let h = HEIGHT as f32 / 2.0 - 200.0;
        let x = 1200.0 * (1.0 * t + std::f32::consts::FRAC_PI_4).sin();
        let y = 1200.0 * (3.0 * t).sin();
        let shape = ShapeBuilder::new()
            .rect_xywh(point2(x + w, y + h), point2(i as f32 * 0.5, i as f32 * 0.0))
            .no_fill()
            .stroke_color(sc)
            .stroke_weight(4.0)
            .build();
        shape.draw(&mut canvas);
    }

    canvas.save("lissa.png");
}
