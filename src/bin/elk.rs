use colorous;
use tiny_skia::*;
use wassily::shape::*;
use wassily::util::*;

const WIDTH: u32 = 4032;
const HEIGHT: u32 = 3024;

fn main() {
    let mut canvas = Pixmap::load_png("elk.png").expect("Can't load png");

    let colors = colorous::REDS;

    for l in 0..HEIGHT {
        let t = l as f64 / HEIGHT as f64;
        let c = colors.eval_continuous(t);
        let mut alpha = 235;
        if l < 7 * HEIGHT / 15 {
            alpha = 0
        }
        let kolor = Color::from_rgba8(c.r, c.g, c.b, alpha);
        let h = l as f32;
        let ln =ShapeBuilder::new()
            .stroke_color(kolor)
            .stroke_weight(1.5)
            .line(pt2(0.0, h), pt2(WIDTH as f32, h))
            .build();
        ln.draw(&mut canvas);
    }

    let rh = 0.2 * HEIGHT as f32;
    let y = -rh + 7.0 / 15.0 * HEIGHT as f32;
    
    let color = Color::from_rgba8(0, 0, 0, 150);
    let shape = ShapeBuilder::new()
        .rect_xywh(pt2(0.0, y), pt2(WIDTH as f32, rh))
        .fill_color(color)
        .stroke_color(color)
        .stroke_weight(1.5)
        .build();
    shape.draw(&mut canvas);

    let color = Color::from_rgba8(255, 255, 255, 190);
    let shape = ShapeBuilder::new()
        .circle(pt2(0.88 * WIDTH as f32, 0.15 * HEIGHT as f32), 250.0)
        .stroke_weight(20.0)
        .fill_color(color)
        .stroke_color(color)
        .build();
    shape.draw(&mut canvas);
    canvas.save_png("image.png").unwrap();
}
