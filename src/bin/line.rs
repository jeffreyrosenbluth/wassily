use rand::prelude::*;
use rand_distr::{Distribution, Normal};

use noise::OpenSimplex;
use wassily::prelude::*;
use wassily::skia::Canvas;

const WIDTH: u32 = 1500;
const HEIGHT: u32 = 1500;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    canvas.fill(WHITE);

    let mut palette = Palette::new(vec![]);
    let seed: u64 = random();

    let wiggle = DotLine::new(point2(100.0, 1200.0), point2(1400.0, 300.0)).
    stdev(5.0).
    weight(30).
    noise_strength(30.0).
    color(palette.rand_lab());
    
    palette.set_seed(seed);
    wiggle.draw(&mut canvas);

    let wiggle2 = DotLine::new(point2(100.0, 500.0), point2(2900.0, 2500.0)).
    stdev(5.0).
    weight(30).
    noise_strength(30.0).
    color(palette.rand_lab());
    wiggle2.draw(&mut canvas);


    canvas.save("line.png");
}
