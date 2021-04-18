use rand::{thread_rng, Rng};
use tiny_skia::*;
use wassily::shape::*;
use wassily::util::*;

const WIDTH: u32 = 4032;
const HEIGHT: u32 = 3024;

fn main() {
    let mut layer0 = Pixmap::load_png("elk.png").expect("Can't load png");
    let mut layer1 = Pixmap::new(WIDTH, HEIGHT).unwrap();

    let mut paint = PixmapPaint::default();
    paint.quality = FilterQuality::Bicubic;
    // paint.blend_mode = BlendMode::Hue;

    let mut rng = thread_rng();

    // for _ in 0..20000 {
    //     let x = rng.gen_range(0.0..WIDTH as f32);
    //     let y = rng.gen_range(0.0..HEIGHT as f32);
    //     let r = rng.gen_range(5.0..20.0);
    //     // let r = 10.0;
    //     let mut square = ShapeBuilder::new()
    //         .rect_xywh(pt2(x, y), pt2(r, r))
    //         .fill_color(Color::TRANSPARENT)
    //         .no_stroke()
    //         .build();

    //     let mut p = square.fill_paint.unwrap();
    //     p.blend_mode = BlendMode::Source;
    //     square.fill_paint = Some(p);

    //     square.draw(&mut canvas1);
    // }

    let center = pt2(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0);
    let mut circle = ShapeBuilder::new()
        .circle(center, 1000.0)
        .fill_color(Color::from_rgba8(255, 0, 0, 255))
        .no_stroke()
        .build();

    let mut p = circle.fill_paint.unwrap();
    p.blend_mode = BlendMode::Xor;
    circle.fill_paint = Some(p);

    circle.draw(&mut layer0);

    // canvas0.draw_pixmap(0, 0, layer1.as_ref(), &paint);
    layer0.save_png("xor.png").unwrap();
}
