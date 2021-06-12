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
        .weight(100)
        .noise_strength(30.0)
        .color(BLACK);

    palette.set_seed(seed);
    wiggle.draw(&mut canvas);

    let mut sand = SandLine::new(point2(100.0, 300.0), point2(1400.0, 1200.0), 64, 50.0, BLACK);
    sand.draw(&mut canvas);



    canvas.save("line.png");
}
