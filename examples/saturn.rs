#![allow(dead_code)]
use itertools::Itertools;
use rand_distr::{Distribution, Normal};
use wassily::prelude::*;

const SEED: u64 = 73;
const WIDTH: u32 = 3840;
const HEIGHT: u32 = 2160;
const PHASE: f32 = 15.0;
const FREQ: f32 = 4.0;
const RADIUS: usize = 120;
const SPACING: f32 = 3.0;
const THICKNESS: f32 = 0.20;
const PADDING: usize = 20;
const STYLE: Style = Style::Fractal;
const STD: f32 = 0.0125;

#[derive(Debug, Clone, Copy)]
enum Style {
    Fractal,
    Clipped,
    Standard,
}

impl Distribution<Style> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Style {
        match rng.gen_range(0..3) {
            0 => Style::Fractal,
            1 => Style::Clipped,
            _ => Style::Standard,
        }
    }
}

fn g0(x: f32, y: f32) -> f32 {
    (x * x - y * y / 4.0).clamp(0.0, 1.0).sqrt()
}

fn g1(x: f32, y: f32) -> f32 {
    let x = 4.5 * x;
    let y = 4.5 * y;
    0.25 * (5.0 * x * x + y * y - 4.0) * (x * x + 5.0 * y * y - 4.0)
}

fn g2(x: f32, y: f32) -> f32 {
    (x * x - y * y / 4.0).clamp(0.0, 1.0).sqrt()
}

fn main() {
    use Style::*;
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let mut rng = SmallRng::seed_from_u64(SEED);
    // let mut rng = SmallRng::from_entropy();

    // Choose the colors
    let c1 = rand_okhsl(&mut rng);
    let c2 = rand_okhsl(&mut rng);
    let c3 = rand_okhsl(&mut rng);
    let c4 = rand_okhsl(&mut rng);
    let c5 = rand_okhsl(&mut rng);

    canvas.fill(*WHITE);

    // let permutations: Vec<_> = vec![c1, c2, c3, c4, c5]
    //     .into_iter()
    //     .permutations(5)
    //     .collect();
    // let perm = permutations.choose(&mut rng).unwrap();

    let normal = Normal::new(0.0, STD).unwrap();
    for y in 0..HEIGHT {
        let cs = ColorScale::new(c1, c2, c3, c4, c5);
        for x in 0..WIDTH {
            let u = x as f32 / WIDTH as f32 - 0.5;
            let v = y as f32 / HEIGHT as f32 - 0.5;
            let s = g0(u, v) + normal.sample(&mut rng);
            // let t = y as f32 / HEIGHT as f32 + x as f32 / WIDTH as f32 + normal.sample(&mut rng);
            let c = match STYLE {
                Clipped => cs.get_color_clip(s * FREQ, 0.1),
                Fractal => cs.get_color_fractal(s * FREQ, PHASE),
                Standard => cs.get_color(s * FREQ),
            };
            canvas.dot(x as f32, y as f32, c);
        }
    }
    canvas.save_png("saturn.png");
}
