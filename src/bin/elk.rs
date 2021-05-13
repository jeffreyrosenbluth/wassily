use colorous;
use wassily::base::*;
use wassily::kolor::*;
use wassily::shape::*;
use wassily::util::*;
use wassily::point2;

#[cfg(feature = "tiny-skia")]
use wassily::skia::Canvas;

const WIDTH: u32 = 4032;
const HEIGHT: u32 = 3024;

fn main() {
    let mut canvas = Canvas::load_png("fire.png");

    let colors = colorous::REDS;

    for l in 0..HEIGHT {
        let t = l as f64 / HEIGHT as f64;
        let c = colors.eval_continuous(t);
        let mut alpha = 235;
        if l < 7 * HEIGHT / 15 {
            alpha = 0
        }
        let kolor = RGBA::new(
            c.r as f32 / 255.0,
            c.g as f32 / 255.0,
            c.b as f32 / 255.0,
            alpha as f32 / 255.0,
        );
        let h = l as f32;
        let ln = ShapeBuilder::new()
            .stroke_color(kolor)
            .stroke_weight(1.5)
            .line(point2(0.0, h), point2(WIDTH as f32, h))
            .build();
        ln.draw(&mut canvas);
    }

    let rh = 0.2 * HEIGHT as f32;
    let y = -rh + 7.0 / 15.0 * HEIGHT as f32;

    let color = RGBA::new(0.0, 0.0, 0.0, 150.0 / 255.0);
    let shape = ShapeBuilder::new()
        .rect_xywh(point2(0.0, y), point2(WIDTH as f32, rh))
        .fill_color(color)
        .stroke_color(color)
        .stroke_weight(1.5)
        .build();
    shape.draw(&mut canvas);

    let color = RGBA::new(1.0, 1.0, 1.0, 190.0 / 255.0);
    let shape = ShapeBuilder::new()
        .circle(point2(0.88 * WIDTH as f32, 0.15 * HEIGHT as f32), 125.0)
        .stroke_weight(20.0)
        .fill_color(color)
        .stroke_color(color)
        .build();
    shape.draw(&mut canvas);

    canvas.save_png("elk_skia.png");
}
