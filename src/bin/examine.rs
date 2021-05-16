use wassily::prelude::*;
use wassily::svg::Canvas;

const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 0.80 * WIDTH;

fn main() {
    let mut canvas = Canvas::new(WIDTH as u32, HEIGHT as u32);
    let path = file_path("fruit.png");
    let palette = Palette::steal(path, 16);
    canvas.fill(RGBA::with_8(222, 222, 222, 255));


    let square = ShapeBuilder::new()
        .circle(point2(WIDTH / 2.0, HEIGHT / 2.0), 100.0)
        .fill_color(palette.colors[7])
        .stroke_color(RGBA::black())
        .stroke_weight(5.0)
        .build();
    square.draw(&mut canvas);
    canvas.save("examine.svg");
}
