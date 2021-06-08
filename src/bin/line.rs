use rand::prelude::*;
use wassily::prelude::*;
use wassily::skia::Canvas;

const WIDTH: u32 = 1500;
const HEIGHT: u32 = 1500;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    canvas.fill(WHITE);

    let mut palette = Palette::new(vec![]);
    let seed: u64 = random();

    let wiggle = DotLine::new(point2(100.0, 1200.0), point2(1400.0, 300.0))
        .stdev(5.0)
        .weight(30)
        .noise_strength(30.0)
        .color(palette.rand_lab());

    palette.set_seed(seed);
    wiggle.draw(&mut canvas);

    let wiggle2 = DotLine::new(point2(100.0, 500.0), point2(2900.0, 2500.0)).
    stdev(10.0).
    weight(5).
    noise_strength(30.0).
    // color(palette.rand_lab());
    color(BLACK);
    wiggle2.draw_hair(&mut canvas);

    let c = ShapeBuilder::new()
        .circle(point2(250.0, 250.0), 25.0)
        .cartesian(1500.0, 1500.0)
        .fill_color(RGBA::with_8(255, 0, 0, 255))
        .no_stroke()
        .build();
    c.draw(&mut canvas);

    canvas.save("line.png");
}
