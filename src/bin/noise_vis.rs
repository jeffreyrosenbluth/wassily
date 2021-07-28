#[allow(unused_imports)]
use noise::{BasicMulti, Fbm, OpenSimplex, Perlin, SuperSimplex};

use colorous::*;
use wassily::prelude::*;
use wassily::raqote::Canvas;

const WIDTH: u32 = 19200;
const HEIGHT: u32 = 14400;
const GRID: f32 = 250.0;
const SCALE: f32 = 4.0;
const FACTOR: f32 = 2.0;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let ns = Noise::<_, 2>::new(WIDTH as f32, WIDTH as f32, OpenSimplex::default())
        .scales(SCALE)
        .factor(FACTOR);
    let mut sm = 0.0;
    let mut lg = 0.0;
    let mut c: colorous::Color;

    for x in (0..WIDTH).step_by(GRID as usize) {
        for y in (0..HEIGHT).step_by(GRID as usize) {
            let mut n = ns.get(x as f32, y as f32);
            if n > lg {
                lg = n
            };
            if n < sm {
                sm = n
            }
            if n < 0.0 {
                n = n.abs().clamp(0.0, 1.0);
                c = ORANGES.eval_continuous(n as f64);
            } else {
                n = n.clamp(0.0, 1.0);
                c = BLUES.eval_continuous(n as f64);
            }

            let color = RGBA::rgba8(c.r, c.g, c.b, 255);
            let sc = RGBA::rgba8(255 - c.r, 255 - c.g, 255 - c.b, 255);

            let square = ShapeBuilder::new()
                // .circle(point2(x as f32, y as f32), 0.525 * GRID)
                .rect_xywh(point2(x as f32, y as f32), point2(GRID, GRID))
                .fill_color(color)
                .stroke_color(sc)
                .stroke_weight(GRID / 2.0)
                .build();

            square.draw(&mut canvas);
        }
    }
    dbg!(sm, lg);
    canvas.save("vis_50.png");
}
