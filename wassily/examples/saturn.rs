#![allow(dead_code)]
use rand_distr::{Distribution, Normal, Standard};
use wassily::prelude::*;

const D: u32 = 1;
const SEED: u64 = 84; // 81, 83, 84*, 93, 110, 116, 124, 129, 137, 141,151
const WIDTH: u32 = 1080 / D;
const HEIGHT: u32 = 1080 / D;
const PHASE: f32 = 0.01;
const FREQ: f32 = 2.05;
const STYLE: Style = Style::Clipped;
const STD: f32 = 0.00;

#[derive(Debug, Clone, Copy)]
enum Style {
    Fractal,
    Clipped,
    Standard,
}

impl Distribution<Style> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Style {
        match rng.random_range(0..3) {
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

fn g3(x: f32, y: f32) -> f32 {
    (x * x + y * y).clamp(0.0, 1.0).sqrt()
}

fn g4(x: f32, y: f32) -> f32 {
    let h = 0.2;
    let k = 0.5 * (h + 1.0 / h);
    let px = x.abs();
    let py = y.abs();
    if px < 1.0 && py < px * (k - h) + h {
        k - (px * px + (py - k) * (py - k)).sqrt()
    } else {
        (px * px + (py - h) * (py - h))
            .sqrt()
            .min((px - 1.0).powi(2) + py.powi(2))
    }
}

fn g5(x: f32, y: f32) -> f32 {
    let theta = 0.15 * (y.atan2(x) + PI);
    4.0 * theta.sqrt().clamp(0.0, 1.0)
}

fn g6(x: f32, y: f32) -> f32 {
    x * y * (x + 2.0 * y + 2.0).powf(2.0)
}

fn g7(x: f32, y: f32) -> f32 {
    let r = (x * x + y * y).sqrt();
    let theta = f32::atan2(y, x);
    (1.0 + 0.10 * f32::sin(8.0 * theta).powf(5.0)) * r
}
// float sdVesica(vec2 p, float r, float d)
// {
//     p = abs(p);
//     float b = sqrt(r*r-d*d);
//     return ((p.y-b)*d>p.x*b) ? length(p-vec2(0.0,b))
//                              : length(p-vec2(-d,0.0))-r;
// }

fn g8(x: f32, y: f32, r: f32, d: f32) -> f32 {
    let x = x.abs() + 0.5;
    let y = y.abs();
    let b = (r * r - d * d).sqrt();
    if (y - b) * d > x * b {
        (x.powi(2) + (y - b).powi(2)).sqrt()
    } else {
        ((x - d).powi(2) + y).powi(2).sqrt() - r
    }
}

fn main() {
    use Style::*;
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let mut rng = SmallRng::seed_from_u64(SEED);

    // let nf = FPerlin::new(SEED as u32);
    let nf = RidgedMulti::<Perlin>::new(SEED as u32).set_octaves(4);
    let opts = NoiseOpts::default().scales(0.5).factor(1.0);

    // Choose the colors
    let c1 = rand_okhsl(&mut rng);
    let c2 = rand_okhsl(&mut rng);
    let c3 = rand_okhsl(&mut rng);
    let c4 = rand_okhsl(&mut rng);
    let c5 = rand_okhsl(&mut rng);

    println!("{:?}", c1);
    println!("{:?}", c2);
    println!("{:?}", c3);
    println!("{:?}", c4);
    println!("{:?}", c5);

    // let c4 = *BLACK;
    // let c1 = *RED;
    // let c2 = rgb8(0, 255, 0);
    // let c3 = *BLUE;
    // let c5 = *WHITE;

    canvas.fill(*WHITE);

    let cs = ColorScale::new(c1, c2, c3, c4, c5);
    let normal = Normal::new(0.0, STD).unwrap();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let u = x as f32 / WIDTH as f32 - 0.5;
            let v = y as f32 / HEIGHT as f32 - 0.5;
            // let s = noise2d_01(&nf, &opts, u, v) + normal.sample(&mut rng);
            let s = g8(u, v, 1.0, 0.27) + normal.sample(&mut rng);
            let c = match STYLE {
                Clipped => cs.get_color_clip(s * FREQ, 0.1),
                Fractal => cs.get_color_fractal(s * FREQ, PHASE),
                Standard => cs.get_color(s * FREQ),
            };
            canvas.dot(x as f32, y as f32, c);
        }
    }
    // Shape::new()
    //     .rect_cwh(pt(WIDTH / 2, HEIGHT / 4), pt(WIDTH, HEIGHT / 36))
    //     .fill_color(*WHITE)
    //     .no_stroke()
    //     .draw(&mut canvas);
    // Shape::new()
    //     .rect_cwh(pt(WIDTH / 2, HEIGHT as f32 / 1.1), pt(WIDTH, HEIGHT / 24))
    //     .fill_color(*WHITE)
    //     .no_stroke()
    //     .draw(&mut canvas);
    // Shape::new()
    //     .rect_cwh(pt(WIDTH / 4, HEIGHT / 2), pt(WIDTH / 16, HEIGHT))
    //     .fill_color(*WHITE)
    //     .no_stroke()
    //     .draw(&mut canvas);
    // Shape::new()
    //     .rect_cwh(pt(WIDTH as f32 / 1.1, HEIGHT / 2), pt(WIDTH / 36, HEIGHT))
    //     .fill_color(*WHITE)
    //     .no_stroke()
    //     .draw(&mut canvas);
    canvas.save_png("default.png");
}
