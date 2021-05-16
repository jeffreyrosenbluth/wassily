use std::path;
use wassily::raqote::Canvas;
use wassily::{
    base::*,
    kolor::{black, Palette},
    point2,
    shape::*,
    Point,
};

const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 0.80 * WIDTH;

fn main() {
    let mut canvas = Canvas::new(WIDTH as u32, HEIGHT as u32);
    let path = path::Path::new("fruit.png");
    let mut palette = Palette::steal(path, 16);
    let n = palette.colors.len();

    let rect = ShapeBuilder::new()
        .rect_xywh(point2(0.0, 0.0), point2(WIDTH, HEIGHT))
        .no_stroke()
        .fill_color(RGBA::with_8(50, 50, 50, 255))
        .build();
    rect.draw(&mut canvas);

    let square = ShapeBuilder::new()
        .circle(point2(WIDTH / 2.0, HEIGHT / 2.0), 100.0)
        .fill_color(RGBA::with_8(200, 100, 150, 255))
        .stroke_color(RGBA::with_8(0, 0, 255, 255))
        .stroke_weight(5.0)
        .build();
    square.draw(&mut canvas);
    canvas.save_png("examine.png");
}
