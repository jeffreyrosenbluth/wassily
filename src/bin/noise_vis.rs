use noise::{BasicMulti, Fbm, OpenSimplex, Perlin, SuperSimplex};
use tiny_skia::*;
use wassily::noise::*;
use wassily::shape::*;
use wassily::util::pt2;
use colorous::*;

const SIZE: u32 = 8191;
const GRID: f32 = 10.0;
const SCALE: f32 = 2.0;

fn main() {
    let mut canvas = Pixmap::new(SIZE, SIZE).unwrap();
    let mut ns = Noise::<[f64; 2], _>::new(SIZE as f32, SIZE as f32, Fbm::new());
    ns.set_noise_scales(SCALE, SCALE, 1.0);
    let mut sm = 0.5;
    let mut lg = 0.5;
    let mut c: colorous::Color;

    for x in (0..SIZE).step_by(GRID as usize) {
        for y in (0..SIZE).step_by(GRID as usize) {
            let mut n = ns.noise(x as f32, y as f32);
            if n > lg {
                lg = n
            };
            if n < sm {
                sm = n
            }
            if n < 0.0 {
                n = n.abs().clamp(0.0, 1.0);
                c = ORANGES.eval_continuous(n as f64);
            }  else {
                n = n.clamp(0.0, 1.0);
                c = BLUES.eval_continuous(n as f64);
            }

            let square = ShapeBuilder::new()
                .rect_xywh(pt2(x as f32, y as f32), pt2(GRID, GRID))
                .no_stroke()
                .fill_color(tiny_skia::Color::from_rgba8(c.r, c.g, c.b, 255))
                .build();
            square.draw(&mut canvas);
        }
    }
    dbg!(sm, lg);
    canvas.save_png("vis.png").unwrap();
}
